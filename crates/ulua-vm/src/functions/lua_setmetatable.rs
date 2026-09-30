use core::ptr::{eq, null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{index_2_addr::index_2_addr, lua_g_readonlyerror::check_writable},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_c_objbarrier::lua_c_objbarrier,
    lua_o_nilobject::LUA_O_NILOBJECT, ttype::ttype,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈顶留有 1 个元表或 nil 值（`api_checknelems 1`；非 nil 时须为 table，
/// `api_check` 校验）；`objindex` 经 `index_2_addr` 解析出的槽须非 `LUA_O_NILOBJECT`；目标为只读 table 时
/// `luaG_readonlyerror` 抛错；写 `metatable` 字段并 `luaC_objbarrier`（可 GC），须受保护帧。cpp `lapi.cpp:1067`。
pub unsafe fn lua_setmetatable(l: *mut LuaState, objindex: i32) -> i32 {
  unsafe {
    api_checknelems!(l, 1);

    let obj: StkId = index_2_addr(l, objindex);
    api_check!(l, !eq(obj, LUA_O_NILOBJECT));

    let mut mt: *mut LuaTable = null_mut();
    if !(*(*l).top.sub(1)).is_nil() {
      api_check!(l, (*(*l).top.sub(1)).is_table());
      mt = (*(*l).top.sub(1)).as_table_ptr();
    }

    match ttype!(obj) {
      x if x == LuaType::Table as u32 => {
        let h = (*obj).as_table_ptr();
        check_writable(l, h);
        (*h).metatable = mt;
        if !mt.is_null() {
          lua_c_objbarrier!(l, h, mt);
        }
      }
      x if x == LuaType::UserData as u32 => {
        let u = (*obj).as_userdata_ptr().cast_mut();
        (*u).metatable = mt;
        if !mt.is_null() {
          lua_c_objbarrier!(l, u, mt);
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
