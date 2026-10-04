use core::{
  ffi::{c_int, c_void},
  fmt::{Debug, Formatter, Result},
};

use ulua_vm::records::lua_state::LuaState;

/// 初始化回调签名：在配置执行窗口内被调用一次，以独占借用交入沙箱 VM 状态，
/// 转手 `userdata`。
///
/// 保留 `unsafe` 的裁定（review.md §2 判定 1）：本别名不为 C ABI 调用约定服务
/// （`extern` 形只出现在 [`InterruptFn`]），`unsafe` 源于部分实现体会解引用
/// `userdata`——它是经 VM lightuserdata 槽转手的裸地址，存活期/类型前提无法由
/// 类型承载。能降的是实现本体而非别名：只转手不解引用的实现（如
/// [`attach_threaddata_init`]）就是安全 `fn`，coerce 进本别名合法（safe fn 指针
/// 是 unsafe fn 指针的子形态）。
pub type ConfigInitFn = unsafe fn(l: &mut LuaState, userdata: *mut c_void);

/// 中断回调签名：对齐 C 侧 `lua_Callbacks::interrupt` 与 VM `LuaCallbacks.interrupt` 槽。
///
/// 调用约定契约（必须保持 C 形态，非本区门面选择）：
/// - 谁调用：指针由 `extract_config` 写入 VM `LuaCallbacks.interrupt` 槽
///   （`lua_CInterrupt` 约定），由 VM 在配置线程执行期的中断检查点回调，
///   不在 Rust 侧任何具名调用栈上，故 ABI 必须是 `extern` 形且与槽类型逐字一致。
/// - 栈帧与存活期：回调在配置脚本自身栈帧之上同线程直接触发，只在配置执行
///   同步窗口内被调用；槽随沙箱 `LuaState` 存亡。
/// - 是否可抛错/unwind：`-unwind` 是必须的——实现方可经 `luaL_error` 发散
///   （超时中断即此形态），展开须能穿越 VM 的中断触发帧抵达 `lua_pcall`
///   保护点；首参 `*mut LuaState` 与 `gc: c_int`（Lua GC 阶段占位）按槽约定
///   由 VM 交回裸地址，被调体就地物化借用。
///
/// 消费面降级判定：受本别名钉死的是 fn **本体**的 `unsafe extern "C-unwind"`
/// 形（判定：真 C 侧回调约定边界，保留），而各门面返回值（如
/// `NavigationContext::luau_config_interrupt`）直投本别名、不再逐处重拼裸签名。
pub type InterruptFn = unsafe extern "C-unwind" fn(l: *mut LuaState, gc: c_int);

/// 配置执行前的初始化回调对（函数指针 + 转手用户数据），静态分派、零分配。
///
/// `callback` 只应在 `extract_config` 执行配置的同步窗口内被调用一次；
/// `userdata` 是该窗口内存活的回调携带数据的裸指针地址，具体指向何处由
/// 各 `callback` 实现方的 Safety 契约约束（Rust 侧调用，非 C ABI 边界，
/// VM 状态以 `&mut` 独占借用交入，见 [`ConfigInitFn`]）。
#[derive(Clone, Copy)]
pub struct ConfigInitCallback {
  pub callback: ConfigInitFn,
  pub userdata: *mut c_void,
}

impl ConfigInitCallback {
  /// 构造初始化回调对。
  #[inline]
  pub const fn new(callback: ConfigInitFn, userdata: *mut c_void) -> Self {
    Self { callback, userdata }
  }
}

impl Debug for ConfigInitCallback {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("ConfigInitCallback")
      .field("callback", &(self.callback as usize))
      .field("userdata", &(self.userdata as usize))
      .finish()
  }
}

/// 通用初始化回调：把 `userdata` 原样挂进 VM 线程数据槽。
///
/// 对应 cpp `Analyze.cpp` 的 `[&info](LuaState* l) { lua_setthreaddata(l, &info); }`
/// 布线形态（`info` 地址经裸指针转手，仅在配置执行窗口内被中断回调读取）。
///
/// DELIBERATE DEVIATION（review.md §9.3）：cpp 用函数指针 + `LuaState*` 裸参，
/// Rust 侧首参收编为 `&mut LuaState` 后解引用消失——挂接经 vm 的安全门面
/// `set_thread_data`（lightuserdata 转手槽）完成；`userdata` 只转手、本函数
/// 不解引用，无任何裸指针操作，故降为安全 `fn`（review.md §2 判定 2）。
///
/// 调用序契约（正确性，非内存安全）：`userdata` 须指向配置执行同步窗口内
/// 存活的数据——它被挂入线程数据槽后只会被同窗口的中断回调读回。
pub fn attach_threaddata_init(l: &mut LuaState, userdata: *mut c_void) {
  l.set_thread_data(userdata);
}

/// 配置执行回调对（对应 C++ `LuauConfigInterrupt` 的 init/中断函数对）。
#[derive(Clone, Copy, Default)]
pub struct InterruptCallbacks {
  /// 执行配置前的一次性线程数据初始化回调（静态分派对，见 [`ConfigInitCallback`]）。
  pub init_callback: Option<ConfigInitCallback>,
  /// 配置执行期中断回调：直投 [`InterruptFn`]，调用约定契约（谁调用/栈帧/
  /// 存活期/unwind）见该别名文档——经 `extract_config` 写入 VM
  /// `LuaCallbacks.interrupt` 槽，由 VM 在配置线程执行期回调。
  pub interrupt_callback: Option<InterruptFn>,
}

impl Debug for InterruptCallbacks {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("InterruptCallbacks")
      .field("init_callback", &self.init_callback)
      .field(
        "interrupt_callback",
        &self.interrupt_callback.map(|f| f as usize),
      )
      .finish()
  }
}
