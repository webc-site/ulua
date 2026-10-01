use alloc::{collections::BTreeMap, string::String};
use core::{
  cell::RefCell,
  sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use itoa::Buffer;
use ulua_common::functions::c_str::{cstr, cstr_cow};
use ulua_vm::{
  functions::{lua_callbacks::lua_callbacks, lua_getinfo::lua_getinfo},
  records::lua_state::LuaState,
};

use crate::{
  functions::{ZERO_DEBUG, state_ref::state},
  records::profiler::{GC_STATE_COUNT, ProfilerMain, ProfilerShared},
};

/// cpp 文件静态量 `static Profiler gProfiler` 的跨线程共享发布面：`ticks`/`samples`
/// 由采样线程唯一写入，`exit` 主线程落、采样线程读——纯原子字段，静态量普通放置
/// 即 `Sync`，读写零 unsafe（review.md §2：无同步共享面收进最小原子发布点）。
pub(crate) static G_PROFILER_SHARED: ProfilerShared = ProfilerShared {
  exit: AtomicBool::new(false),
  ticks: AtomicU64::new(0),
  samples: AtomicU64::new(0),
};

// VM/主线程独占字段：`thread_local` 把 cpp「文件静态量只在 VM/主线程触碰」的前提
// 写进类型系统（单线程所有 ⇒ 借用天然互斥），原 `UnsafeCell` 分区视图与手工
// `unsafe impl Sync` 契约随之消失（review.md §2）。`frequency` 与回调表句柄不再
// 驻场：`profiler_start` 按值搬进 `thread::spawn`，采样线程栈上独占读。
thread_local! {
  pub(crate) static G_PROFILER_MAIN: RefCell<ProfilerMain> = const {
    RefCell::new(ProfilerMain {
      thread: None,
      current_ticks: 0,
      stack_scratch: String::new(),
      data: BTreeMap::new(),
      gc: [0; GC_STATE_COUNT],
      callbacks: None,
    })
  };
}

/// `lua_getinfo` 选项串「取 short_src+name」（NUL 结尾字节串，经 `cstr` 收口点
/// 转 C 指针，review.md §10：不散落 `.as_ptr().cast()`）。
const GETINFO_SN_OPT: &[u8] = b"sn\0";

/// 采样栈快照：`lua_getinfo` 逐级上爬拼 `src,line,linedefined;…` 串（真 FFI 边
/// 界的遍历循环，(c) 保留）。拼好的串写入复用的 `stack_scratch`。
///
/// `l` 为 VM 线程当前有效状态；`stack` 必须是 `G_PROFILER_MAIN` 的
/// `stack_scratch` 字段在本帧的独占借用（thread_local 单线程所有），回调期间
/// 无其它别名——两者均以借用类型表达，本函数体内只剩 `lua_getinfo` 导出一处
/// `unsafe`（what 串经 `cstr` 门面收口）。
fn collect_stack(l: &mut LuaState, gc: i32, stack: &mut String) {
  stack.clear();
  if gc > 0 {
    stack.push_str("GC,GC,");
  }

  // C++ `LuaDebug ar = {}`：编译期零初值（见 functions::ZERO_DEBUG）
  let mut ar = ZERO_DEBUG;
  // 一枚复用的 itoa 栈缓冲（采样热路径，免 core::fmt 开销与逐级重初始化）
  let mut num = Buffer::new();
  // cpp `for (level = 0; lua_getinfo(...); level++)`：open-ended range 即同形，
  // getinfo 返回 0 即 break。
  for level in 0.. {
    // Safety: `lua_getinfo` 为 unsafe 导出；`&mut ar` 以 `&mut T → *mut T` 隐式转换
    // 交出本地独占出参，getinfo 成功时把 short_src/name 填为 NUL 结尾串（串缓冲由
    // 调用帧持有，本循环窗口内有效）；what 为 NUL 结尾静态字节串（`cstr` 门面）。
    if unsafe { lua_getinfo(l, level, cstr(GETINFO_SN_OPT), &mut ar) } == 0 {
      break;
    }

    if !stack.is_empty() {
      stack.push(';');
    }

    // 「判空 + NUL 截断解码」样板收敛到 cstr_cow 门面：null 译空串，push 空串
    // 与原判空跳过的观察行为一致。
    // Safety: `cstr_cow` 为 unsafe fn；short_src/name 为 null 或 NUL 结尾串（上一行
    // getinfo 契约）。
    stack.push_str(&unsafe { cstr_cow(ar.short_src) });
    stack.push(',');
    // Safety: 同上。
    stack.push_str(&unsafe { cstr_cow(ar.name) });
    stack.push(',');
    if ar.linedefined > 0 {
      stack.push_str(num.format(ar.linedefined));
    }
  }
}

/// 把本次采样的栈快照累加进 `data`（cpp `data[stack] += elapsed` 的 Rust 化）。
///
/// 纯 Rust 集合运算：入参已是具名可变引用，无需 `unsafe`。空栈直接跳过，与 cpp
/// `if (stack.empty()) return;` 一致。
fn accumulate_sample(data: &mut BTreeMap<String, u64>, stack: &str, elapsed_ticks: u64) {
  if stack.is_empty() {
    return;
  }
  // 仅新栈首次出现时才复制 key（每秒最多 frequency 次采样，热路径上
  // 克隆整条栈字符串开销可观）
  match data.get_mut(stack) {
    Some(ticks) => *ticks += elapsed_ticks,
    None => {
      data.insert(stack.to_owned(), elapsed_ticks);
    }
  }
}

/// Faithful port of Profiler.cpp's `static void profilerTrigger(LuaState* l, int gc)`.
///
/// DELIBERATE DEVIATION（review.md §9.3）：作为 VM safepoint interrupt 的实际处理体，
/// 在 VM 线程上解引用 `*mut LuaState` 并经 `lua_getinfo`/`lua_callbacks` 采样栈
/// （ulua-vm c-API 边界）；VM 线程独占字段全部经 `G_PROFILER_MAIN`（thread_local）
/// 取用，跨线程共享只剩 `G_PROFILER_SHARED` 的原子发布面。
///
/// 前置条件（原 `# Safety` 契约，由唯一调用方 [`profiler_interrupt`]
/// （`crates/ulua-repl-cli/src/functions/profiler_loop.rs`）的 safepoint 回调约定
/// 成立；与 [`crate::functions::counters_track::counters_track`] 同款「安全入口 +
/// 边界块」形态）：`l` 必须指向存活的 `LuaState` 且本函数在 VM 线程上调用。
pub(crate) fn profiler_trigger(l: *mut LuaState, gc: i32) {
  // Safety: l 由 safepoint 回调约定保证为 VM 线程当前有效状态（前置条件见本 fn
  // 文档），经 `state` 门面一次物化后全走安全方法/带契约块。
  let l = state(l);
  G_PROFILER_MAIN.with(|cell| {
    // 触发窗口内整帧独占本线程字段：一次 borrow_mut，逐字段切分借用。
    // 本帧不调用会重入 interrupt 的 VM 原语（getinfo 纯读栈），无双重借用。
    let mut p = cell.borrow_mut();

    // ticks 是采样线程唯一发布的累计量：原子读与并发写者安全共存
    // （Relaxed，与 cpp 同语义）。
    let current = G_PROFILER_SHARED.ticks.load(Ordering::Relaxed);
    let elapsed_ticks = current - p.current_ticks;

    if elapsed_ticks != 0 {
      let ProfilerMain {
        stack_scratch,
        data,
        gc: gc_ticks,
        ..
      } = &mut *p;

      // stack_scratch 是本帧独占字段借用，采样窗口内无其它别名
      // （collect_stack 的文档契约）。
      collect_stack(l, gc, stack_scratch);
      accumulate_sample(data, stack_scratch, elapsed_ticks);

      if gc > 0 {
        // cpp `gProfiler.gc[gc]` 无检查（越界即 UB）；Rust 侧用 get_mut 挡住超出
        // GC 状态数的异常入参，越界不入账，取安全方向。
        if let Some(slot) = gc_ticks.get_mut(gc as usize) {
          *slot += elapsed_ticks;
        }
      }
    }

    p.current_ticks = current;

    // 摘除 interrupt 接线：接线过的回调表优先（判空即 cpp 的 nullptr 检查）。
    // Safety: callbacks 由 profiler_start 于采样线程 spawn 前写入、本线程只读本
    // 字段；`Option<NonNull>` 是 Copy，按值读出后 `as_mut` 改写的是指针对象的
    // interrupt 槽——该写发生在 VM safepoint 处理体内（safepoint 窗口外无人读写
    // 该槽），指针对象随状态存活覆盖本回调窗口。
    unsafe {
      match p.callbacks {
        Some(mut callbacks) => callbacks.as_mut().interrupt = None,
        // else 分支：采样线程 shim 未发布 callbacks 指针，直接清活跃状态的
        // interrupt 槽，等价 cpp `gProfiler.callbacks->interrupt = nullptr`。
        // Safety: l 为 VM 线程当前有效状态（前置条件见本 fn 文档）；lua_callbacks
        // 对有效状态恒返回非空回调表，本步只清 interrupt 单槽。
        None => (*lua_callbacks(l)).interrupt = None,
      }
    }
  });
}
