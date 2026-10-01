use ulua_vm::records::lua_state;

use crate::{
  functions::get_type_user_data::get_type_user_data,
  records::arena_handle::alias_ref,
  type_aliases::{lua_state::LuaState, type_function_type_id::TypeFunctionTypeId},
};
pub fn optional_type_user_data(l: *mut LuaState, idx: i32) -> Option<TypeFunctionTypeId> {
  if alias_ref(l as *mut lua_state::LuaState).is_none_or_nil(idx) {
    None
  } else {
    Some(get_type_user_data(l, idx))
  }
}
