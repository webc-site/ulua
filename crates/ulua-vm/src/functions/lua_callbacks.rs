use crate::{records::lua_callbacks::LuaCallbacks, type_aliases::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_callbacks(l: *mut lua_State) -> *mut LuaCallbacks {
  unsafe { &mut (*(*l).global).cb as *mut LuaCallbacks }
}
