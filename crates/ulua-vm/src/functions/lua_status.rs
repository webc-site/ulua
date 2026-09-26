use crate::records::lua_state::LuaState;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_status(l: *mut LuaState) -> i32 {
  unsafe { (*l).status as i32 }
}
