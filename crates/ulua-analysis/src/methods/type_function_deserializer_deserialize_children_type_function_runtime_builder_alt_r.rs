//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:1095-1098`
//!
//! ```cpp
//! void deserializeChildren(TypeFunctionVariadicTypePack* v2, VariadicTypePack* v1)
//! {
//!     v1->ty = shallowDeserialize(v2->type);
//! }
//! ```
use crate::records::{
  type_function_deserializer::TypeFunctionDeserializer,
  type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  variadic_type_pack::VariadicTypePack,
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn deserialize_children_type_function_variadic_type_pack_variadic_type_pack(
    &mut self,
    v2: *mut TypeFunctionVariadicTypePack,
    v1: *mut VariadicTypePack,
  ) {
    unsafe {
      let deserialized = self.shallow_deserialize_type_function_type_id((*v2).type_id);
      (*v1).ty = deserialized;
    }
  }
}
