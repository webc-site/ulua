use ulua_vm::functions::{
  lua_l_typeerror_l::lua_l_typeerror_l, lua_touserdatatagged::lua_touserdatatagged,
};
// kTypeUserdataTag is a constant used for Luau Type Function userdata.
use ulua_vm::records::lua_state;

use crate::type_aliases::{lua_state::LuaState, type_function_type_id::TypeFunctionTypeId};
const K_TYPE_USERDATA_TAG: i32 = 42;

pub fn get_type_user_data(l: *mut LuaState, idx: i32) -> TypeFunctionTypeId {
  unsafe {
    let typ = lua_touserdatatagged(l as *mut lua_state::LuaState, idx, K_TYPE_USERDATA_TAG)
      as *mut TypeFunctionTypeId;

    if !typ.is_null() {
      return *typ;
    }

    lua_l_typeerror_l(l as *mut lua_state::LuaState, idx, "type");
  }
}
