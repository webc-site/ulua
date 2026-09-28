use crate::{
  functions::{cstr_bytes, lua_typename::lua_typename},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_b_type(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_any(1);
    // resulting name doesn't differentiate between userdata types
    let t = (*l).type_of(1);
    let name = lua_typename(l, t as i32);
    (*l).push_bytes(cstr_bytes(name));
    1
  }
}

lua_lib_fn!(pub fn lua_b_type, lua_b_type_arm);
