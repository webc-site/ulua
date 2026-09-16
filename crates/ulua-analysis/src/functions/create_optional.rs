use alloc::vec::Vec;
use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l},
  records::lua_state,
};

use crate::{
  enums::type_type_function_runtime::Type,
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    allocate_type_function_type::allocate_type_function_type,
    get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
  },
  records::{
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_union_type::TypeFunctionUnionType,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_optional(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "types.optional: expected 1 argument, but got {}",
          argument_count
        ),
      );
    }

    let argument: TypeFunctionTypeId = get_type_user_data(l, 1);

    let mut components: Vec<TypeFunctionTypeId> = Vec::new();

    let union_ty = get_type_function_type_id::<TypeFunctionUnionType>(argument);
    if !union_ty.is_null() {
      components.reserve((*union_ty).components.len() + 1);
      components.extend((*union_ty).components.iter().copied());
    } else {
      components.push(argument);
    }

    let nil_type = TypeFunctionPrimitiveType::new(Type::NilType);
    let nil_variant = TypeFunctionTypeVariant::Primitive(nil_type);
    let nil_id = allocate_type_function_type(l, nil_variant);
    components.push(nil_id);

    let union_type = TypeFunctionUnionType { components };
    let union_variant = TypeFunctionTypeVariant::Union(union_type);
    alloc_type_user_data(l, union_variant, false);

    1
  }
}
