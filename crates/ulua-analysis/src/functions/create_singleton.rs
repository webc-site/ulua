use core::ffi::{CStr, c_int};

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkboolean::lua_l_checkboolean, lua_l_error_l::lua_l_error_l, lua_type::lua_type,
    lua_typename::lua_typename,
  },
  macros::{
    lua_isboolean::lua_isboolean, lua_isnil::lua_isnil, lua_l_checkstring::luaL_checkstring,
  },
  records::lua_state,
};

use crate::{
  enums::type_type_function_runtime::Type,
  functions::alloc_type_user_data::alloc_type_user_data,
  records::{
    type_function_boolean_singleton::TypeFunctionBooleanSingleton,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
  },
  type_aliases::{
    lua_state::LuaState, type_function_singleton_variant::TypeFunctionSingletonVariant,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_singleton(l: *mut LuaState) -> c_int {
  let vm_l = l as *mut lua_state::LuaState;

  if lua_isboolean!(vm_l, 1) {
    let value = unsafe { lua_l_checkboolean(vm_l, 1) } != 0;
    unsafe {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
          variant: TypeFunctionSingletonVariant::V0(TypeFunctionBooleanSingleton { value }),
        }),
        false,
      )
    };

    return 1;
  }

  if unsafe { lua_type(vm_l, 1) } == LuaType::String as i32 {
    let value = luaL_checkstring!(vm_l, 1);
    unsafe {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
          variant: TypeFunctionSingletonVariant::V1(TypeFunctionStringSingleton {
            value: CStr::from_ptr(value).to_string_lossy().into_owned(),
          }),
        }),
        false,
      )
    };

    return 1;
  }

  if lua_isnil!(vm_l, 1) {
    unsafe {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType::new(Type::NilType)),
        false,
      )
    };

    return 1;
  }

  let type_name = unsafe { CStr::from_ptr(lua_typename(vm_l, 1)) }.to_string_lossy();
  unsafe {
    lua_l_error_l(
      vm_l,
      c"%s".as_ptr(),
      core::format_args!(
        "types.singleton: can't create singleton from `{}` type",
        type_name
      ),
    )
  };
  0
}
