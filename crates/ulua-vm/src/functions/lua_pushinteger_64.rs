use crate::{
  enums::lua_type::LuaType,
  macros::{api_incr_top::api_incr_top, setlvalue::setlvalue},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_pushinteger_64")]
pub(crate) unsafe fn lua_pushinteger_64(l: *mut lua_State, n: i64) {
  unsafe {
    let _ = LuaType::Integer;
    setlvalue!((*l).top, n);
    api_incr_top!(l);
  }
}
