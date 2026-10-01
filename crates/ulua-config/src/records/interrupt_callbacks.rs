use core::{
  ffi::{c_int, c_void},
  fmt::{Debug, Formatter, Result},
};

use ulua_vm::records::lua_state::LuaState;

/// 初始化回调函数签名：在配置执行窗口内被调用一次，转手 `userdata`。
pub type ConfigInitFn = unsafe fn(l: *mut LuaState, userdata: *mut c_void);

/// 中断回调函数签名：对齐 C 侧 `lua_Callbacks::interrupt` 与 VM `LuaCallbacks.interrupt` 槽。
pub type InterruptFn = unsafe extern "C-unwind" fn(l: *mut LuaState, gc: c_int);

/// 配置执行前的初始化回调对（函数指针 + 转手用户数据），静态分派、零分配。
///
/// `callback` 只应在 `extract_config` 执行配置的同步窗口内被调用一次；
/// `userdata` 是该窗口内存活的回调携带数据的裸指针地址，具体指向何处由
/// 各 `callback` 实现方的 Safety 契约约束（Rust 侧调用，非 C ABI 边界，
/// 故取 Rust ABI 的 `unsafe fn`）。
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
/// DELIBERATE DEVIATION（review.md §9.3）：`ConfigInitCallback` 静态分派对成员，
/// 解引用 `*mut LuaState` 调 `set_thread_data`（lightuserdata 转手槽），`userdata`
/// 只转手不解引用——均系 VM 线程数据边界固有形态，非纯 Rust 逻辑。
///
/// # Safety
/// `l` 必须是执行配置的 VM 提供的存活 `LuaState`；`userdata` 只作转手存储
/// 的 lightuserdata 挂入线程数据槽，本函数不解引用（其存活期由挂接方的
/// 配置执行窗口契约保证）。
pub unsafe fn attach_threaddata_init(l: *mut LuaState, userdata: *mut c_void) {
  // Safety: 契约保证 l 为存活且本窗口内独占驱动的 `LuaState`，`&mut *l` 排他引用
  // 重建前提成立；userdata 只转手存储，与 cpp 原版同义。
  unsafe { (*l).set_thread_data(userdata) };
}

/// 配置执行回调对（对应 C++ `LuauConfigInterrupt` 的 init/中断函数对）。
#[derive(Clone, Copy, Default)]
pub struct InterruptCallbacks {
  /// 执行配置前的一次性线程数据初始化回调（静态分派对，见 [`ConfigInitCallback`]）。
  pub init_callback: Option<ConfigInitCallback>,
  /// FFI 契约: 经 `extract_config` 直接写入 VM `LuaCallbacks.interrupt` 槽
  /// (`extern "C-unwind"`, 对齐 C 侧 `lua_Callbacks::interrupt`), 故保留 `c_int` 而非 `i32`。
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
