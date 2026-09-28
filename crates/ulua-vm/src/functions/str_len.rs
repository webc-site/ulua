use crate::{
  functions::lua_l_checklstring::lua_l_checklstring_ref, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn str_len(l: *mut LuaState) -> i32 {
  unsafe {
    // 借用切片形态：出参 len 由切片长度直接承接
    let s = lua_l_checklstring_ref(l, 1);
    (*l).push_integer(s.len() as i32);
    1
  }
}

lua_lib_fn!(pub fn str_len, str_len_arm);
