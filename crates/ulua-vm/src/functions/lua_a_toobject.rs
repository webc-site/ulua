use core::{
  ffi::c_int,
  ptr::{eq, null},
};

pub use crate::macros::lua_o_nilobject::luaO_nilobject;
use crate::{
  functions::index_2_addr::index_2_addr,
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_a_toobject(l: *mut lua_State, idx: c_int) -> *const TValue {
  unsafe {
    let p: StkId = index_2_addr(l, idx);

    if eq(p, luaO_nilobject) {
      null()
    } else {
      p as *const TValue
    }
  }
}

pub use lua_a_toobject as luaA_toobject;
