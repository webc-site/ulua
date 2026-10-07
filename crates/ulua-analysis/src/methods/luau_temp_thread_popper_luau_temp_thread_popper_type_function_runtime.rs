use ulua_vm::records::lua_state::LuaState;

use crate::records::{arena_handle::alias, luau_temp_thread_popper::LuauTempThreadPopper};

impl LuauTempThreadPopper {
  pub fn new(l: *mut LuaState) -> Self {
    Self { l }
  }
}

impl LuauTempThreadPopper {
  /// 归还临时线程：把 `l` 栈顶的一枚值弹出。
  ///
  /// Safety 前提由 `alias` 门面契约承载：`self.l` 是宿主 `StateRef` 保活的存活
  /// state，且本次弹出处于该 state 的独占窗口内。
  pub fn luau_temp_thread_popper(&mut self) {
    alias(self.l).pop(1);
  }
}
