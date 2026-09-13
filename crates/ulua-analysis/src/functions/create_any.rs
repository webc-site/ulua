use core::ffi::c_int;

use crate::{
  functions::alloc_type_user_data::alloc_type_user_data,
  records::type_function_any_type::TypeFunctionAnyType,
  type_aliases::{lua_state::LuaState, type_function_type_variant::TypeFunctionTypeVariant},
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_any(l: *mut LuaState) -> c_int {
  unsafe {
    alloc_type_user_data(
      l,
      TypeFunctionTypeVariant::Any(TypeFunctionAnyType::default()),
      false,
    )
  };
  1
}
