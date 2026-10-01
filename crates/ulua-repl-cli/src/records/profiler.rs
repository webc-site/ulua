use alloc::{collections::BTreeMap, string::String};
use core::{
  ptr::NonNull,
  sync::atomic::{AtomicBool, AtomicU64},
};
use std::thread::JoinHandle;

use ulua_vm::records::lua_callbacks::LuaCallbacks;

/// GC 状态槽数量（cpp Profiler.cpp 的 `uint64_t gc[16]`）
pub(crate) const GC_STATE_COUNT: usize = 16;

/// cpp 文件静态 `gProfiler` 的跨线程共享发布面：采样线程唯一写入、VM/主线程只读。
/// 全部为原子字段，天然 `Sync`——静态量普通放置即可，读写零 unsafe
/// （review.md §2：把 `UnsafeCell` + 手工 `Sync` 契约压缩到「确需共享」的最小面，
/// 其余字段按线程归属拆开）。
pub(crate) struct ProfilerShared {
  /// 采样线程退出标志（仅经 Relaxed 原子访问，与 cpp 语义一致）。
  pub(crate) exit: AtomicBool,
  /// 累计 tick 发布面。
  pub(crate) ticks: AtomicU64,
  /// 采样次数发布面。
  pub(crate) samples: AtomicU64,
}

/// VM/主线程独占字段：经 `thread_local` 把「单线程所有」写进类型系统，
/// 逐字段借用即 `&mut`，与 cpp 无同步文件静态量同一前提（本 REPL 的 VM 与
/// 主线程同为一线程；`profiler_dump` 在 join 采样线程后于同线程读取）。
/// 若前提被违背（VM 异线程），行为退化为「该线程数据不入账」，不再是数据竞争。
pub(crate) struct ProfilerMain {
  /// 采样线程句柄：`profiler_start` 存入、`profiler_stop` 取出并 join（同线程串行）。
  pub(crate) thread: Option<JoinHandle<()>>,
  /// 上次触发时的 tick。
  pub(crate) current_ticks: u64,
  /// 复用的栈快照缓冲。
  pub(crate) stack_scratch: String,
  /// 栈 → tick 累加表（cpp `DenseHashMap<string,uint64>` 的 Rust 等价收集）。
  pub(crate) data: BTreeMap<String, u64>,
  /// 按 GC 状态计的 tick。
  pub(crate) gc: [u64; GC_STATE_COUNT],
  /// cpp `Profiler::callbacks`（可空）：未接线即 `None`，null 哨兵由 `Option` 表达
  /// （review.md §2）。指向 `lua_callbacks(l)` 返回的状态固定区回调表，存活随
  /// 状态覆盖整个采样窗口（start 晚于建状态、stop 的 join 先于 close）。
  pub(crate) callbacks: Option<NonNull<LuaCallbacks>>,
}

/// 交入采样线程的回调表句柄（`profiler_start` 在 spawn 前按值传入）。
///
/// cpp 的对应形态是采样线程读文件静态 `gProfiler.callbacks`；这里改为值搬运 +
/// `Send` 契约，共享面上不再出现可被任意线程读写的裸指针字段。
// DELIBERATE DEVIATION（review.md §9.3）：跨线程搬运 VM 回调表句柄需要 `Send`，
// `NonNull` 本体非 `Send`，以本包装承载单点契约。
// Safety：指针对象为 `lua_callbacks(l)` 返回的状态固定区 `LuaCallbacks`，其存活期
// 覆盖采样窗口（`profiler_start` 晚于状态创建、`profiler_stop` join 先于状态关闭，
// 二者把句柄生命周期夹在中间）；采样线程经它只写 VM safepoint 契约下独占的
// `interrupt` 槽（与 cpp `gProfiler.callbacks->interrupt = profilerTrigger` 同形），
// 不触碰其余字段、不转存该引用。
#[derive(Clone, Copy)]
pub(crate) struct SamplerCallbacks(pub(crate) NonNull<LuaCallbacks>);

// Safety: 见类型上的契约说明——存活期由 start/spawn 与 stop/join 的先后次序覆盖，
// 写入面限于 VM safepoint 契约下采样线程独占的 `interrupt` 单槽。
unsafe impl Send for SamplerCallbacks {}
