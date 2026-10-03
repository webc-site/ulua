// kTypeUserdataTag is a constant used for Luau Type Function userdata.

use core::mem::size_of;
use ulua_vm::{functions::{lua_l_checkstack::lua_l_checkstack, lua_newuserdatatagged::lua_newuserdatatagged}, records::lua_state};
use crate::{functions::lua_names::TYPE, type_aliases::{type_function_type_id::TypeFunctionTypeId}};
use ulua_vm::records::lua_state::LuaState;
const K_TYPE_USERDATA_TAG: i32 = 42;

pub fn push_type(l: &mut LuaState, r#type: TypeFunctionTypeId) {
  unsafe {
    lua_l_checkstack(l.as_mut_ptr(), 2, "allocating type");

    let ptr = lua_newuserdatatagged(l.as_mut_ptr(),
      size_of::<TypeFunctionTypeId>(),
      K_TYPE_USERDATA_TAG,
    ) as *mut TypeFunctionTypeId;

    *ptr = r#type;

    // set the new userdata's metatable to type metatable
    (*(l as *mut lua_state::LuaState)).get_metatable_by_bytes(TYPE);
    (*(l as *mut lua_state::LuaState)).set_metatable(-2);
  }
}
