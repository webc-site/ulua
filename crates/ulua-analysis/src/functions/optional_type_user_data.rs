

use crate::{functions::get_type_user_data::get_type_user_data, type_aliases::{type_function_type_id::TypeFunctionTypeId}};
use ulua_vm::records::lua_state::LuaState;
pub fn optional_type_user_data(l: &mut LuaState, idx: i32) -> Option<TypeFunctionTypeId> {
  if l.is_none_or_nil(idx) {
    None
  } else {
    Some(get_type_user_data(l, idx))
  }
}
