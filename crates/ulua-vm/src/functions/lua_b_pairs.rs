use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
  },
  macros::lua_upvalueindex::lua_upvalueindex,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_b_pairs"))]
pub(crate) unsafe extern "C-unwind" fn lua_b_pairs(l: *mut lua_State) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_pushvalue(l, lua_upvalueindex(1));
    lua_pushvalue(l, 1);
    lua_pushnil(l);
    3
  }
}
