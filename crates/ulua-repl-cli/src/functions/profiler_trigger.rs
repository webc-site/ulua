use alloc::{collections::BTreeMap, string::String};
use core::{
  cell::RefCell,
  sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use itoa::Buffer;
use ulua_vm::{
  functions::{lua_callbacks::lua_callbacks, lua_getinfo::lua_getinfo},
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::records::profiler::{GC_STATE_COUNT, ProfilerMain, ProfilerShared};

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

/// `lua_getinfo` 选项串「取 short_src+name」：review.md §10 后 `what` 形参为选项
/// 字节窗（`auxgetinfo` 全窗迭代），故这里就是选项本身，不再补终止 `\0`。
const GETINFO_SN_OPT: &[u8] = b"sn";

/// 采样栈快照：`lua_getinfo` 逐级上爬拼 `src,line,linedefined;…` 串（真 FFI 边
/// 界的遍历循环，(c) 保留）。拼好的串写入复用的 `stack_scratch`。
///
/// `l` 为 VM 线程当前有效状态；`stack` 必须是 `G_PROFILER_MAIN` 的
/// `stack_scratch` 字段在本帧的独占借用（thread_local 单线程所有），回调期间
/// 无其它别名——两者均以借用类型表达，本函数体内只剩 `lua_getinfo` 导出一处
/// `unsafe`（选项窗为静态切片，句柄由 `as_mut_ptr` 就地派生）。
fn collect_stack(l: &mut LuaState, gc: i32, stack: &mut String) {
  stack.clear();
  if gc > 0 {
    stack.push_str("GC,GC,");
  }

  // C++ `LuaDebug ar = {}`：`Default` 逐字段给出「未回填即空」初值（安全、可折叠）
  let mut ar = LuaDebug::default();
  // 一枚复用的 itoa 栈缓冲（采样热路径，免 core::fmt 开销与逐级重初始化）
  let mut num = Buffer::new();
  // cpp `for (level = 0; lua_getinfo(...); level++)`：open-ended range 即同形，
  // getinfo 返回 0 即 break。
  for level in 0.. {
    // Safety: `lua_getinfo` 为 unsafe 导出；`l.as_mut_ptr()` 是本帧独占借用的地址
    // （采样在 VM safepoint 窗口内、单线程驱动），`&mut ar` 是本地独占出参；选项窗
    // 为静态只读切片且不含 `f`，不压栈。
    if unsafe { lua_getinfo(l.as_mut_ptr(), level, GETINFO_SN_OPT, &mut ar) } == 0 {
      break;
    }

    if !stack.is_empty() {
      stack.push(';');
    }

    // 记录字段已是原生字节窗：`short_src` 为写端截断后的有效字节，`name` 的 `None`
    // （旧 null 哨兵）→ `unwrap_or_default()` 空窗，push 空串与原判空跳过的观察
    // 行为一致；lossy 解码与旧 `cstr_cow` 消费面同点。
    stack.push_str(&String::from_utf8_lossy(ar.short_src.bytes()));
    stack.push(',');
    stack.push_str(&String::from_utf8_lossy(ar.name.unwrap_or_default()));
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
// review.md §2/§3 收形：`l` 由裸 `*mut LuaState` 收编为借用 `&mut LuaState`，真实物化
// 点上移到 safepoint C 回调 `profiler_interrupt`（其体内一次 `&mut *l`）。采样本体
// collect_stack/lua_callbacks 均在该借用上走 ulua-vm 引用形安全面/带契约块。
pub(crate) fn profiler_trigger(l: &mut LuaState, gc: i32) {
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
