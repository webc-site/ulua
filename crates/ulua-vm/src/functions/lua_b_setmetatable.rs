//! Source: `VM/src/lbaselib.cpp:75-85` (hand-ported)

use crate::{
  enums::lua_type::LuaType,
  functions::lua_l_getmetafield::lua_l_getmetafield_bytes,
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位须为 table（`lua_l_checktype`）、2 号位须为 nil/table
/// （`arg_expected`），`lua_l_getmetafield` 检出 `__metatable` 时经 `lua_l_error_l` 抛错，否则
/// `lua_setmetatable` 写元表并可 GC。cpp/VM/src/lbaselib.cpp:100 luaB_setmetatable。
pub(crate) unsafe fn lua_b_setmetatable(l: *mut LuaState) -> i32 {
  unsafe {
    let t = (*l).type_of(2);
    (*l).check_type(1, LuaType::Table);
    (*l).arg_expected(t == LuaType::Nil || t == LuaType::Table, 2, "nil or table");
    if lua_l_getmetafield_bytes(l, 1, b"__metatable") != 0 {
      luaL_error!(l, "cannot change a protected metatable");
    }
    (*l).set_top(2);
    (*l).set_metatable(1);
    1
  }
}

lua_lib_fn!(pub(crate) fn lua_b_setmetatable, lua_b_setmetatable_arm);
