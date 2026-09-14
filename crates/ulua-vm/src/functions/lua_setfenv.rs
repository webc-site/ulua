use core::{ffi::c_int, ptr::eq};

use crate::{
  enums::lua_type::LuaType,
  functions::index_2_addr::index_2_addr,
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, gcvalue::gcvalue, hvalue::hvalue,
    lua_c_objbarrier::luaC_objbarrier, lua_o_nilobject::luaO_nilobject, ttistable::ttistable,
    ttype::ttype,
  },
  records::{closure::Closure, lua_state::lua_State},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setfenv(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    let mut res: i32 = 1;
    api_checknelems!(l, 1);
    let o: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(o, luaO_nilobject));
    api_check!(l, ttistable!((*l).top.sub(1)));
    match ttype!(o) {
      x if x == LuaType::Function as c_int => {
        let cl = core::ptr::addr_of_mut!((*(*o).value.gc).cl) as *mut Closure;
        (*cl).env = hvalue!((*l).top.sub(1));
      }
      x if x == LuaType::Thread as c_int => {
        let th = core::ptr::addr_of_mut!((*(*o).value.gc).th) as *mut lua_State;
        (*th).gt = hvalue!((*l).top.sub(1));
      }
      _ => {
        res = 0;
      }
    }
    if res != 0 {
      luaC_objbarrier!(l, gcvalue!(o), hvalue!((*l).top.sub(1)));
    }
    (*l).top = (*l).top.sub(1);
    res
  }
}
