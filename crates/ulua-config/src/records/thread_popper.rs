use ulua_vm::{macros::lua_pop::lua_pop, records::lua_state::LuaState};

/// RAII 弹栈守卫：离开作用域时弹出 `l` 栈顶一个值
/// （对应 cpp oracle 的手动 `lua_pop(L, 1)` 配对）。
#[derive(Debug)]
pub(crate) struct ThreadPopper {
  /// 构造契约：必须是有效 VM 状态，且本守卫存活期间栈顶有可弹出的值。
  pub(crate) l: *mut LuaState,
}

impl Drop for ThreadPopper {
  fn drop(&mut self) {
    // Safety: 按构造契约，l 为有效 VM 状态且其栈顶持有待弹值
    unsafe {
      lua_pop(self.l, 1);
    }
  }
}
