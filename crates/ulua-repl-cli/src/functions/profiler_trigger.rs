use alloc::{collections::BTreeMap, string::String};
use core::{
  cell::UnsafeCell,
  mem::zeroed,
  ptr::NonNull,
  sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use itoa::Buffer;
use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{
  functions::{lua_callbacks::lua_callbacks, lua_getinfo::lua_getinfo},
  records::{lua_callbacks::LuaCallbacks, lua_debug::LuaDebug, lua_state::LuaState},
};

// Trigger-side view of Profiler.cpp's file-static `gProfiler`. The sampling
// thread publishes `ticks` (atomic) and the VM-thread trigger consumes them,
// keeping `current_ticks`, the `stack_scratch` reuse Buffer, the accumulated
// per-stack `data` and the per-GC-state timing array — exactly the fields
// `profilerTrigger` reads and writes. (Profiler.cpp uses a
// DenseHashMap<string,uint64> for `data`; a BTreeMap captures the same
// stack→ticks accumulation.)
use crate::records::profiler::{GC_STATE_COUNT, Profiler};

/// cpp 文件静态量 `static Profiler gProfiler` 的跨线程共享容器：`UnsafeCell`
/// 包裹 + 下述字段分区契约。采样线程只碰原子字段，VM 线程侧的消费一律经
/// [`SharedProfiler::fields`] 取「按线程分区的字段视图」，业务逻辑不再直接
/// 解引用 `*mut Profiler`；[`SharedProfiler::get`] 仅保留给 start/stop/loop/dump
/// 这些同样按字段分区别名访问的兄弟入口。
pub(crate) struct SharedProfiler(UnsafeCell<Profiler>);

// Safety: 对内部 `Profiler` 的跨线程访问按字段分区，且由 happens-before 链保证
// 无数据竞争：
// - `frequency`/`callbacks`/`exit`/`thread` 由 `profiler_start`（主线程）写入，
//   写先于采样线程 spawn（spawn 建立同步边）；此后 `frequency`/`callbacks` 在
//   采样线程与 VM 线程只读，直到 `profiler_stop` join 采样线程后才有下一次
//   `profiler_start`（repl_main 为串行流程，start/stop/dump 均在主线程）；
// - `ticks`/`samples`/`exit` 仅经 Relaxed 原子读写访问（与 cpp 语义一致）；
// - `current_ticks`/`stack_scratch`/`data`/`gc` 只在 VM 线程（本 REPL 即主线程）
//   的 `profiler_trigger` 中触碰；`profiler_dump` 在 `profiler_stop` join 采样
//   线程、且 VM 已停止执行后于同线程读取，读与写同线程顺序化，无竞争。
unsafe impl Sync for SharedProfiler {}

/// [`Profiler`] 按线程分区后的字段视图（review.md §2「`(*p).field` 手工解引用 →
/// 具名字段引用」的收口形态）。
///
/// 视图里的引用互不相交（不同字段），故可在同一窗口内并存；但**构造**它们必须经
/// [`SharedProfiler::fields`] 这一处，因为「这些字段归属本线程、期间无人整体改写」
/// 的前提不在类型系统内，只能由契约表达。
///
/// 生命周期 `'p` 即 `G_PROFILER` 静态量的生存期（进程级）。
struct ProfilerFields<'p> {
  /// 采样线程唯一发布的累计 tick（原子字段，只读借用即与并发写者安全共存）。
  ticks: &'p AtomicU64,
  /// 上次触发时的 tick —— VM 线程独占。
  current_ticks: &'p mut u64,
  /// 复用的栈快照缓冲 —— VM 线程独占。
  stack_scratch: &'p mut String,
  /// 栈 → tick 累加表 —— VM 线程独占。
  data: &'p mut BTreeMap<String, u64>,
  /// 按 GC 状态计的 tick —— VM 线程独占。
  gc: &'p mut [u64; GC_STATE_COUNT],
  /// cpp `Profiler::callbacks`：`profiler_start` 写入后本线程只读（判空即 cpp 的
  /// nullptr 检查），其指向对象的 `interrupt` 槽写入另受 VM safepoint 契约约束。
  callbacks: &'p Option<NonNull<LuaCallbacks>>,
}

impl SharedProfiler {
  /// 进程级静态量的裸指针；start/stop/loop/dump 这些同样按字段分区的入口继续用它。
  ///
  /// 触发路径（`profiler_trigger`）不用此函数，改走 [`Self::fields`]。
  pub(crate) fn get(&self) -> *mut Profiler {
    self.0.get()
  }

  /// `UnsafeCell` 裸地址 → [`ProfilerFields`] 字段视图的单点收口。
  ///
  /// # Safety
  /// 调用方必须落在 [`SharedProfiler`] 的 `unsafe impl Sync` 字段分区契约内：
  /// 只在 VM 线程（`profiler_trigger` 的调用窗口）取用，且返回视图存活期间不得再
  /// 构造覆盖整个 `Profiler`（含原子字段）的引用——那会与采样线程对原子字段的写
  /// 别名，属确定 UB。
  unsafe fn fields(&self) -> ProfilerFields<'_> {
    let raw = self.0.get();
    // Safety: 逐字段各自成引用，字段互不相交，故不存在重叠借用；不构造整个
    // `Profiler` 的引用。前置条件由本 fn 的 # Safety 交给调用方。
    unsafe {
      ProfilerFields {
        ticks: &(*raw).ticks,
        current_ticks: &mut (*raw).current_ticks,
        stack_scratch: &mut (*raw).stack_scratch,
        data: &mut (*raw).data,
        gc: &mut (*raw).gc,
        callbacks: &(*raw).callbacks,
      }
    }
  }
}

pub(crate) static G_PROFILER: SharedProfiler = SharedProfiler(UnsafeCell::new(Profiler {
  // 未接线即 None（cpp 的文件静态指针零初始化），可空性由 Option 表达
  callbacks: None,
  frequency: 1000,
  thread: None,
  exit: AtomicBool::new(false),
  ticks: AtomicU64::new(0),
  samples: AtomicU64::new(0),
  current_ticks: 0,
  stack_scratch: String::new(),
  data: BTreeMap::new(),
  gc: [0; GC_STATE_COUNT],
}));

/// `lua_getinfo` 选项串「取 short_src+name」（NUL 结尾字节串，收口点转 C 指针）。
const GETINFO_SN_OPT: &[u8] = b"sn\0";

/// 采样栈快照：`lua_getinfo` 逐级上爬拼 `src,line,linedefined;…` 串（真 FFI 边
/// 界的遍历循环，(c) 保留）。拼好的串写入复用的 `stack_scratch`。
///
/// # Safety
///
/// `l` 为 VM 线程当前有效状态；`stack` 必须是 `G_PROFILER` 的 `stack_scratch`
/// 字段在本帧的独占借用（SharedProfiler 字段分区契约），回调期间无其它别名。
unsafe fn collect_stack(l: *mut LuaState, gc: i32, stack: &mut String) {
  stack.clear();
  if gc > 0 {
    stack.push_str("GC,GC,");
  }

  // C++ `LuaDebug ar = {}`：纯 POD，全零是合法初值。
  // Safety: zeroed() 对 POD LuaDebug 合法。
  let mut ar: LuaDebug = unsafe { zeroed() };
  // cpp `for (level = 0; lua_getinfo(...); level++)`：open-ended range 即同形，
  // getinfo 返回 0 即 break。
  for level in 0.. {
    // Safety: l 存活；`&mut ar` 以 `&mut T → *mut T` 隐式转换交出本地独占出参，
    // getinfo 成功时把 short_src/name 填为 NUL 结尾串（串缓冲由调用帧持有，本循环
    // 窗口内有效）。
    if unsafe { lua_getinfo(l, level, GETINFO_SN_OPT.as_ptr().cast(), &mut ar) } == 0 {
      break;
    }

    if !stack.is_empty() {
      stack.push(';');
    }

    // 「判空 + CStr::from_ptr」样板收敛到 cstr_cow 门面：null 译空串，push 空串
    // 与原判空跳过的观察行为一致。
    // Safety: short_src/name 为 null 或 NUL 结尾串（上一行 getinfo 契约）。
    stack.push_str(&unsafe { cstr_cow(ar.short_src) });
    stack.push(',');
    // Safety: 同上。
    stack.push_str(&unsafe { cstr_cow(ar.name) });
    stack.push(',');
    if ar.linedefined > 0 {
      // 数字转串走 itoa 栈缓冲（采样热路径，免 core::fmt 开销），产物逐字节一致
      stack.push_str(Buffer::new().format(ar.linedefined));
    }
  }
}

/// 把本次采样的栈快照累加进 `data`（cpp `data[stack] += elapsed` 的 Rust 化）。
///
/// 纯 Rust 集合运算：入参已是具名可变引用，无需 `unsafe`（原实现的 `(*profiler)`
/// 手工解引用由 [`SharedProfiler::fields`] 收口后，这里只剩业务语义）。空栈直接
/// 跳过，与 cpp `if (stack.empty()) return;` 一致。
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
/// # Safety
///
/// `l` 必须指向存活的 `LuaState` 且本函数在 VM 线程上调用。对 `G_PROFILER` 的跨线程
/// 共享仅靠 `SharedProfiler` 的字段分区契约保证 soundness（见其 `unsafe impl Sync` 的
/// Safety 说明）：采样线程只访问原子字段 `ticks`/`samples`/`exit` 及 `callbacks` 指向的
/// interrupt 槽；本函数只在 VM 线程经 `G_PROFILER.fields()` 取用非原子的
/// `current_ticks`/`stack_scratch`/`data`/`gc` 与只读的 `ticks`/`callbacks`，绝不构造
/// 覆盖整个 `Profiler`（含原子字段）的 `&mut`——那会与另一线程的原子写别名，属确定 UB。
pub(crate) unsafe fn profiler_trigger(l: *mut LuaState, gc: i32) {
  // Safety: G_PROFILER 是进程级 static（地址稳定），本 fn 的契约保证处于 VM 线程、
  // 且取用的字段全部按分区归本线程；视图各字段互不相交，并存无别名冲突。
  let ProfilerFields {
    ticks,
    current_ticks,
    stack_scratch,
    data,
    gc: gc_ticks,
    callbacks,
  } = unsafe { G_PROFILER.fields() };

  // ticks 是采样线程唯一发布的累计量：原子字段经 `&AtomicU64` 只读借用即可与并发
  // 写者安全共存（Relaxed，与 cpp 同语义）；current_ticks 归 VM 线程独占，
  // 这里的普通读写不需要 `unsafe`（分区契约已在上一行的 fields() 收口处论证）。
  let current = ticks.load(Ordering::Relaxed);
  let elapsed_ticks = current - *current_ticks;

  if elapsed_ticks != 0 {
    // Safety: l 为 VM 线程当前有效状态（本 fn 契约）；stack_scratch 是 VM 线程独占
    // 字段，本次借用即 collect_stack 所需的独占窗口。
    unsafe { collect_stack(l, gc, stack_scratch) };
    accumulate_sample(data, stack_scratch, elapsed_ticks);

    if gc > 0 {
      // cpp `gProfiler.gc[gc]` 无检查（越界即 UB）；Rust 侧用 get_mut 挡住超出
      // GC 状态数的异常入参，越界不入账，取安全方向。
      if let Some(slot) = gc_ticks.get_mut(gc as usize) {
        *slot += elapsed_ticks;
      }
    }
  }

  *current_ticks = current;

  // Safety: callbacks 在 profiler_start（采样线程 spawn 前）写入后本线程只读，
  // Option 判空即 cpp 的 nullptr 检查；interrupt 槽写沿用 VM safepoint 契约。
  // else 分支：采样线程 shim 未发布 callbacks 指针，直接清活跃状态的 interrupt 槽，
  // 等价 cpp `gProfiler.callbacks->interrupt = nullptr`。
  unsafe {
    // `Option<NonNull>` 是 Copy：按值读出后改写的是其指向对象的 interrupt 槽
    // （VM safepoint 契约），字段本体保持只读，与 cpp `gProfiler.callbacks` 的
    // 读取语义一致。
    if let Some(mut callbacks) = *callbacks {
      callbacks.as_mut().interrupt = None;
    } else {
      (*lua_callbacks(l)).interrupt = None;
    }
  }
}
