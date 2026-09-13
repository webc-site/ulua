use core::ffi::c_int;

use crate::{
  enums::type_type_function_runtime::Type,
  functions::alloc_type_user_data::alloc_type_user_data,
  records::type_function_primitive_type::TypeFunctionPrimitiveType,
  type_aliases::{lua_state::LuaState, type_function_type_variant::TypeFunctionTypeVariant},
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_buffer(l: *mut LuaState) -> c_int {
  let primitive = TypeFunctionPrimitiveType::new(Type::Buffer);
  unsafe { alloc_type_user_data(l, TypeFunctionTypeVariant::Primitive(primitive), false) };

  1
}
