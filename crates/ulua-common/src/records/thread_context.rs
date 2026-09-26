//! cpp `struct ThreadContext`（`Common/include/Luau/TimeTrace.h:66-136`）的移植：
//! 类型定义与全部机制方法集中在本文件（原 `methods/thread_context_*.rs` 五枚碎片
//! 已折叠回来，逐方法单文件没有独立概念，只产生 `thread_context_thread_context_*`
//! 这类退化名）。
//!
//! cpp 构造函数把 `this` 登记进全局表并取回 threadId；Rust 里 `ThreadContext`
//! 是可移动的值（TLS 实例的地址并不稳定），登记自身指针当场就会悬垂，
//! 因此全局侧只登记 `create_thread` 分配的线程 id（见 [`ThreadContext::new`]）。
//! C++ `~ThreadContext() { if (!events.empty()) flushEvents(); releaseThread(*globalContext, this); }`
//! 对应 [`Drop`]。

use alloc::vec::Vec;
use std::sync::{Arc, OnceLock};

use crate::{
  enums::event_type::EventType,
  functions::{
    create_thread::create_thread, create_token::create_token, flush_events::flush_events,
    get_clock_microseconds::get_clock_microseconds, get_global_context::get_global_context,
    release_thread::release_thread,
  },
  records::{event::Event, global_context::GlobalContext},
};

/// cpp `#if defined(LUAU_ENABLE_TIME_TRACE)`：整套打点机制的总开关。
/// 关闭时 `LUAU_TIMETRACE_*` 宏展开为空（cpp 的 `#else` 分支），本模块的运行时
/// 逻辑也一律不触发。`cfg!` 是编译期常量，相关分支会被直接优化掉。
pub(crate) const ENABLED: bool = cfg!(feature = "luau_enable_time_trace");

/// cpp `struct ThreadContext`（`TimeTrace.h:66-136`）：整套打点机制的线程事件
/// 缓冲。由 `functions::with_thread_context` 以线程局部实例形式持有，构造时向
/// 全局上下文登记、析构时注销并落盘残余事件。
///
/// cpp 镜像工件，sync-cpp 维护；类型保留 `pub` 仅作 `with_thread_context` 闭包
/// 参数的宏 ABI 面，其机制方法已全部收进 `pub(crate)`。
///
/// 不可拷贝：构造即注册（`create_thread`）、析构即注销（`release_thread`）；
/// 克隆体会共享同一 `thread_id`，其 `Drop` 会错销对方的注册记录，故不派生 `Clone`。
#[derive(Debug)]
pub struct ThreadContext {
  pub(crate) global_context: Arc<GlobalContext>,
  pub(crate) thread_id: u32,
  pub(crate) events: Vec<Event>,
  pub(crate) data: Vec<u8>,
}

impl ThreadContext {
  /// cpp `flushEvents()` 的事件数水位（`TimeTrace.h:82-93,111-117`）：超过即落盘。
  const K_EVENT_FLUSH_LIMIT: usize = 8192;

  /// cpp `ThreadContext()` + `~ThreadContext()`（`TimeTrace.h:66-80`，
  /// `Common/src/TimeTrace.cpp:92-110`）：向全局上下文登记线程 id。
  pub(crate) fn new() -> Self {
    let global_context = get_global_context();

    // `threadId = createThread(*globalContext, this)`；feature 关闭时整套
    // 打点机制不存在（cpp 的 `#if defined(LUAU_ENABLE_TIME_TRACE)`），id 留 0。
    let thread_id = if ENABLED {
      create_thread(&global_context)
    } else {
      0
    };

    ThreadContext {
      global_context,
      thread_id,
      events: Vec::new(),
      data: Vec::new(),
    }
  }

  /// cpp `eventEnter(uint16_t token)`（`TimeTrace.h:101-104`）：以当前时钟取
  /// 时间戳入队 Enter 事件。cpp 镜像工件（sync-cpp 维护）；仅本 crate 打点机制
  /// 消费，降 `pub(crate)`。
  pub(crate) fn event_enter(&mut self, token: u16) {
    let microsec = get_clock_microseconds();
    self.events.push(Event {
      r#type: EventType::Enter,
      token,
      data: microsec,
    });
  }

  /// cpp `eventLeave()`（`TimeTrace.h:111-117`）：追加 Leave 事件，并在事件数超过
  /// `K_EVENT_FLUSH_LIMIT` 时落盘。cpp 镜像工件（sync-cpp 维护）；仅本 crate 打点
  /// 机制消费，降 `pub(crate)`。
  pub(crate) fn event_leave(&mut self) {
    self.events.push(Event {
      r#type: EventType::Leave,
      token: 0,
      data: get_clock_microseconds(),
    });

    if self.events.len() > Self::K_EVENT_FLUSH_LIMIT {
      self.flush_events();
    }
  }

  /// cpp `eventArgument(name, value)`（`TimeTrace.h:119-128`）：把 name 与 value
  /// 连同结尾 NUL 追加进 `data` 缓冲区，各记一条 ArgName/ArgValue 事件，事件的
  /// `data` 字段是其在缓冲区中的起始偏移。
  ///
  /// cpp 镜像工件，sync-cpp 维护；保留 `pub`：`LUAU_TIMETRACE_ARGUMENT` 启用形态
  /// 经 `with_thread_context` 在下游展开时调用本方法（宏 ABI 面）。
  ///
  /// 上游只在 `eventLeave` 中做 flush 阈值检查，这里保持一致：参数总是成对出现在
  /// 某个 `Scope` 内，其 Leave 必然跟进。
  pub fn event_argument(&mut self, name: &str, value: &str) {
    self.data_argument(EventType::ArgName, name);
    self.data_argument(EventType::ArgValue, value);
  }

  /// `eventArgument` 的单个键或值入队：追加 NUL 结尾字符串并记录其偏移。
  fn data_argument(&mut self, r#type: EventType, text: &str) {
    let pos = self.data.len() as u32;
    self.data.extend_from_slice(text.as_bytes());
    self.data.push(b'\0');
    self.events.push(Event {
      r#type,
      token: 0,
      data: pos,
    });
  }

  /// cpp `flushEvents()`（`TimeTrace.h:82-93`）：补一条 `flushEvents` Enter、把
  /// 缓冲的 events/data 交给全局上下文落盘，随后清空缓冲并补一条 Leave。
  ///
  /// cpp 镜像工件（sync-cpp 维护）；仅本 crate 打点机制（Leave 水位与
  /// `Drop`）消费，降 `pub(crate)`。
  pub(crate) fn flush_events(&mut self) {
    // `static uint16_t flushToken = createToken(*globalContext, "flushEvents", "TimeTrace");`
    // C++ magic static 保证只注册一次；OnceLock 提供同样的线程安全语义。
    static FLUSH_TOKEN: OnceLock<u16> = OnceLock::new();

    let flush_token =
      *FLUSH_TOKEN.get_or_init(|| create_token(&self.global_context, "flushEvents", "TimeTrace"));

    self.event_enter(flush_token);

    flush_events(
      &self.global_context,
      self.thread_id,
      &self.events,
      &self.data,
    );

    self.events.clear();
    self.data.clear();

    self.event_leave();
  }
}

impl Drop for ThreadContext {
  fn drop(&mut self) {
    // cpp 的 `~ThreadContext` 整体位于 `#if defined(LUAU_ENABLE_TIME_TRACE)` 内：
    // 开关关闭时既无事件需要落盘，也没有登记可供注销。
    if ENABLED {
      if !self.events.is_empty() {
        self.flush_events();
      }

      // `releaseThread(*globalContext, this)`：注销线程 id，与 `new` 里的
      // `create_thread` 严格配对。
      release_thread(&self.global_context, self.thread_id);
    }
  }
}
