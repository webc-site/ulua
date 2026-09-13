use crate::records::lua_state::lua_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_mainthread(l: *mut lua_State) -> *mut lua_State {
  unsafe { (*(*l).global).mainthread }
}
