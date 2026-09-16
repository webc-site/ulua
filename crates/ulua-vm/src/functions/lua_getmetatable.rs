use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi},
  macros::{
    api_incr_top::api_incr_top, hvalue::hvalue, objectvalue::objectvalue, sethvalue::sethvalue,
    ttype::ttype, uvalue::uvalue,
  },
  records::{lua_state::lua_State, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_getmetatable(l: *mut lua_State, objindex: c_int) -> c_int {
  unsafe {
    lua_c_threadbarrier_lapi(l);

    let obj: StkId = index2addr(l, objindex);

    let mt: *mut LuaTable = match ttype!(obj) {
      x if x == LuaType::Table as c_int => (*hvalue!(obj)).metatable,
      x if x == LuaType::UserData as c_int => uvalue!(obj).metatable,
      x if x == LuaType::Object as c_int => (*objectvalue!(obj).lclass).instancemetatable,
      _ => (*(*l).global).mt[ttype!(obj) as usize],
    };

    if !mt.is_null() {
      sethvalue!(l, (*l).top, mt);
      api_incr_top!(l);
    }

    (!mt.is_null()) as c_int
  }
}
