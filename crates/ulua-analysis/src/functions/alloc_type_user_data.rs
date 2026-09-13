use core::mem::size_of;

use ulua_vm::{
  functions::{
    lua_l_checkstack::lua_l_checkstack, lua_newuserdatatagged::lua_newuserdatatagged,
    lua_setmetatable::lua_setmetatable,
  },
  macros::lua_l_getmetatable::lua_l_getmetatable,
  records::lua_state,
};

use crate::{
  functions::allocate_type_function_type::allocate_type_function_type,
  records::type_function_type::TypeFunctionType,
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
const K_TYPE_USERDATA_TAG: i32 = 42;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn alloc_type_user_data(
  l: *mut LuaState,
  type_variant: TypeFunctionTypeVariant,
  frozen: bool,
) {
  unsafe {
    lua_l_checkstack(l as *mut lua_state::LuaState, 2, "allocating type");

    let ptr = lua_newuserdatatagged(
      l as *mut lua_state::LuaState,
      size_of::<TypeFunctionTypeId>(),
      K_TYPE_USERDATA_TAG,
    ) as *mut TypeFunctionTypeId;

    let type_id = allocate_type_function_type(l, type_variant);
    *ptr = type_id;

    let type_ptr = *ptr as *mut TypeFunctionType;
    (*type_ptr).frozen = frozen;

    lua_l_getmetatable(l as *mut lua_state::LuaState, c"type".as_ptr());
    lua_setmetatable(l as *mut lua_state::LuaState, -2);
  }
}
