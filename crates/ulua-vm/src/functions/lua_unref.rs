use crate::{
  functions::lua_h_getnum::lua_h_getnum,
  macros::{
    api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT, lua_refnil::LUA_REFNIL,
    registry::registry, setnvalue::setnvalue,
  },
  records::{global_state::global_State, lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global.registry` 为已建好的 table TValue（`hvalue!` 取表）；`ref_`
/// 须为 `> LUA_REFNIL` 的既有引用号，`luaH_getnum` 返回的槽须非 `LUA_O_NILOBJECT`（`api_check`）；写入的
/// 释放链值为 number 故免屏障。cpp `lapi.cpp:1896`。
pub unsafe fn lua_unref(l: *mut LuaState, ref_: i32) {
  unsafe {
    if ref_ <= LUA_REFNIL {
      return;
    }

    let g: *mut global_State = (*l).global;

    // hvalue! 宏接收 TValue 指针
    // registry!(l) returns &(*(*l).global).registry, which is a &TValue.
    let reg_tvalue_ptr: *const TValue = registry!(l);
    let reg: *mut LuaTable = (*reg_tvalue_ptr).as_table_ptr() as *const _ as *mut LuaTable;

    let slot: *const TValue = lua_h_getnum(reg, ref_);

    api_check!(l, slot != LUA_O_NILOBJECT);

    // similar to how 'lua_h_setnum' makes non-nil slot value mutable
    let mutable_slot = slot as *mut TValue;

    // NB: no barrier needed because value isn't collectable (it's a number)
    setnvalue!(mutable_slot, (*g).registryfree as f64);

    (*g).registryfree = ref_;
  }
}
