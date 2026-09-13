use core::{
  ffi::{c_int, c_void},
  mem::transmute,
};

use crate::{
  functions::{index_2_addr::index2addr, lua_h_getn::lua_h_getn, lua_h_setnum::luaH_setnum},
  macros::{
    api_check::api_check, hvalue::hvalue, lua_c_barriert::luaC_barriert, lua_refnil::LUA_REFNIL,
    lua_registryindex::LUA_REGISTRYINDEX, nvalue::nvalue, registry::registry, setobj_2_t::setobj2t,
    ttisnil::ttisnil,
  },
  records::lua_state::lua_State,
  type_aliases::{lua_table::LuaTable, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_ref(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    api_check!(l, idx != LUA_REGISTRYINDEX);

    let mut ref_ = LUA_REFNIL;
    let g = (*l).global;
    let p: StkId = index2addr(l, idx);

    if !ttisnil!(p) {
      let reg: *mut LuaTable = hvalue!(registry!(l));

      if (*g).registryfree != 0 {
        ref_ = (*g).registryfree;
      } else {
        // The dependency card for lua_h_getn shows an empty signature: pub fn lua_h_getn();
        // In Luau VM, luaH_getn(t) returns int. We transmute to the real signature.
        let lua_h_getn_real: unsafe extern "C-unwind" fn(*mut LuaTable) -> c_int =
          transmute(lua_h_getn as *const c_void);
        ref_ = lua_h_getn_real(reg);
        ref_ += 1;
      }

      let slot: *mut TValue = luaH_setnum(l, reg, ref_);
      if (*g).registryfree != 0 {
        (*g).registryfree = nvalue!(slot) as c_int;
      }

      setobj2t!(l, slot, p);

      luaC_barriert!(l, reg, p);
    }

    ref_
  }
}
