use core::{
  ffi::c_int,
  ptr::{eq, null_mut},
};

use crate::{
  enums::lua_type::LuaType,
  functions::{index_2_addr::index2addr, lua_g_readonlyerror::lua_g_readonlyerror},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, hvalue::hvalue,
    lua_c_objbarrier::luaC_objbarrier, lua_o_nilobject::luaO_nilobject, ttisnil::ttisnil,
    ttistable::ttistable, ttype::ttype, uvalue::uvalue,
  },
  records::{lua_state::lua_State, lua_table::LuaTable, udata::Udata},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setmetatable(l: *mut lua_State, objindex: c_int) -> c_int {
  unsafe {
    api_checknelems!(l, 1);

    let obj: StkId = index2addr(l, objindex);
    api_check!(l, !eq(obj, luaO_nilobject));

    let mut mt: *mut LuaTable = null_mut();
    if !ttisnil!((*l).top.sub(1)) {
      api_check!(l, ttistable!((*l).top.sub(1)));
      mt = hvalue!((*l).top.sub(1));
    }

    match ttype!(obj) {
      x if x == LuaType::Table as c_int => {
        let h = hvalue!(obj);
        if (*h).readonly != 0 {
          lua_g_readonlyerror(l);
        }
        (*h).metatable = mt;
        if !mt.is_null() {
          luaC_objbarrier!(l, h, mt);
        }
      }
      x if x == LuaType::UserData as c_int => {
        let u = uvalue!(obj) as *const _ as *mut Udata;
        (*u).metatable = mt;
        if !mt.is_null() {
          luaC_objbarrier!(l, u, mt);
        }
      }
      _ => {
        (*(*l).global).mt[ttype!(obj) as usize] = mt;
      }
    }

    (*l).top = (*l).top.sub(1);
    1
  }
}
