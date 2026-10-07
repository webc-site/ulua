use core::{
  ffi::{c_int, c_void},
  fmt::{Debug, Formatter, Result},
  ptr::NonNull,
};

use ulua_vm::records::lua_state::LuaState;

/// 中断回调函数签名：对齐 C 侧 `lua_Callbacks::interrupt` 与 VM `LuaCallbacks.interrupt` 槽。
pub type InterruptFn = unsafe extern "C-unwind" fn(l: *mut LuaState, gc: c_int);

/// 配置执行回调对（对应 C++ `LuauConfigInterrupt` 的线程数据布线 + 中断回调对）。
#[derive(Clone, Copy, Default)]
pub struct InterruptCallbacks {
  /// VM lightuserdata 线程数据槽（`lua_setthreaddata`/`lua_getthreaddata` 语义）的
  /// 转手地址。
  ///
  /// 形如 `Option<NonNull<c_void>>`（review.md §2「可空指针 → Option，非空指针 →
  /// 带类型保证的句柄」）：`None` 即「本次配置执行没有配对上下文」，`Some` 恒为非空
  /// 地址，缺席态不再靠裸 null 表达，消费侧无需再判空。
  ///
  /// 真边界（review.md §2）：本枚 `NonNull<c_void>` 是 VM 线程数据槽的固有形态——
  /// [`crate::functions::extract_config`] 在进入配置执行窗口前原样挂接、不解引用，
  /// 中断回调在窗口内经 `lua_getthreaddata` 还原。契约：载荷必须指向配置执行
  /// 同步窗口内存活的数据（由各构造点的调用栈保证），窗口结束后随沙箱状态
  /// 一并关闭，不再被读取。
  pub thread_data: Option<NonNull<c_void>>,
  /// FFI 契约: 经 `extract_config` 直接写入 VM `LuaCallbacks.interrupt` 槽
  /// (`extern "C-unwind"`, 对齐 C 侧 `lua_Callbacks::interrupt`), 故保留 `c_int` 而非 `i32`。
  pub interrupt_callback: Option<InterruptFn>,
}

impl Debug for InterruptCallbacks {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("InterruptCallbacks")
      .field("thread_data", &self.thread_data)
      .field(
        "interrupt_callback",
        &self.interrupt_callback.map(|f| f as usize),
      )
      .finish()
  }
}
