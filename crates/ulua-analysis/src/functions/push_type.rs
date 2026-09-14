// kTypeUserdataTag is a constant used for Luau Type Function userdata.
use core::mem::size_of;

use ulua_vm::{
  functions::{
    lua_l_checkstack::lua_l_checkstack, lua_newuserdatatagged::lua_newuserdatatagged,
    lua_setmetatable::lua_setmetatable,
  },
  macros::lua_l_getmetatable::lua_l_getmetatable,
  records::lua_state,
};

use crate::type_aliases::{lua_state::LuaState, type_function_type_id::TypeFunctionTypeId};
const K_TYPE_USERDATA_TAG: i32 = 42;

pub fn push_type(l: *mut LuaState, r#type: TypeFunctionTypeId) {
  unsafe {
    lua_l_checkstack(l as *mut lua_state::LuaState, 2, "allocating type");

    let ptr = lua_newuserdatatagged(
      l as *mut lua_state::LuaState,
      size_of::<TypeFunctionTypeId>(),
      K_TYPE_USERDATA_TAG,
    ) as *mut TypeFunctionTypeId;

    *ptr = r#type;

    // set the new userdata's metatable to type metatable
    lua_l_getmetatable(l as *mut lua_state::LuaState, c"type".as_ptr());
    lua_setmetatable(l as *mut lua_state::LuaState, -2);
  }
}
