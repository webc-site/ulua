use core::ptr::{eq, null};

pub use crate::macros::lua_o_nilobject::LUA_O_NILOBJECT;
use crate::{
  functions::index_2_addr::index_2_addr,
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_a_toobject(l: *mut LuaState, idx: i32) -> *const TValue {
  unsafe {
    let p: StkId = index_2_addr(l, idx);

    if eq(p, LUA_O_NILOBJECT) {
      null()
    } else {
      p as *const TValue
    }
  }
}
