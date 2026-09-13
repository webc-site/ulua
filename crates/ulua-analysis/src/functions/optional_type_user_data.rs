use ulua_vm::{macros::lua_isnoneornil::lua_isnoneornil, records::lua_state};

use crate::{
  functions::get_type_user_data::get_type_user_data,
  type_aliases::{lua_state::LuaState, type_function_type_id::TypeFunctionTypeId},
};
pub fn optional_type_user_data(l: *mut LuaState, idx: i32) -> Option<TypeFunctionTypeId> {
  unsafe {
    if lua_isnoneornil!(l as *mut lua_state::LuaState, idx) {
      None
    } else {
      Some(get_type_user_data(l, idx))
    }
  }
}
