use core::ffi::{c_int, c_void};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_pcall::luaD_pcall, lua_gettop::lua_gettop},
  macros::{savestack::savestack, setnilvalue::setnilvalue},
  records::resolve_import::ResolveImport,
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, t_value::TValue},
};

pub(crate) unsafe fn resolve_import_safe(
  l: *mut lua_State,
  _env: *mut LuaTable,
  k: *mut TValue,
  id: u32,
) {
  unsafe {
    let mut ri = ResolveImport { k, id };

    if (*(*l).gt).safeenv != 0 {
      let old_top = lua_gettop(l);
      let status = luaD_pcall(
        l,
        Some(ResolveImport::run),
        &mut ri as *mut _ as *mut c_void,
        savestack!(l, (*l).top) as isize,
        0,
      );

      LUAU_ASSERT!(old_top + 1 == lua_gettop(l));

      if status != LuaStatus::Ok as c_int {
        setnilvalue!((*l).top.sub(1));
      }
    } else {
      setnilvalue!((*l).top);
      (*l).top = (*l).top.add(1);
    }
  }
}
