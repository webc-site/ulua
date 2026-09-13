use alloc::vec::Vec;
use core::ffi::c_int;

use ulua_vm::{functions::lua_gettop::lua_gettop, records::lua_state};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data, push_type::push_type,
  },
  records::{
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_unknown_type::TypeFunctionUnknownType,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_intersection(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let arg_size = lua_gettop(vm_l);

    let mut components: Vec<TypeFunctionTypeId> = Vec::with_capacity(arg_size as usize);

    for i in 1..=arg_size {
      let component = get_type_user_data(l, i);

      if let Some(intersection_component) =
        get_type_function_type_id::<TypeFunctionIntersectionType>(component).as_ref()
      {
        components.extend(intersection_component.components.iter().copied());
      } else if !get_type_function_type_id::<TypeFunctionUnknownType>(component).is_null() {
        continue;
      } else {
        components.push(component);
      }
    }

    if components.is_empty() {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Unknown(TypeFunctionUnknownType::default()),
        false,
      );
    } else if components.len() == 1 {
      push_type(l, components[0]);
    } else {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Intersection(TypeFunctionIntersectionType { components }),
        false,
      );
    }

    1
  }
}
