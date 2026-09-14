use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_pushinteger::lua_pushinteger,
    lua_pushvalue::lua_pushvalue,
  },
  macros::lua_upvalueindex::lua_upvalueindex,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_ipairs")]
pub(crate) unsafe extern "C-unwind" fn lua_b_ipairs(l: *mut lua_State) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_pushvalue(l, lua_upvalueindex(1));
    lua_pushvalue(l, 1);
    lua_pushinteger(l, 0);
    3
  }
}
