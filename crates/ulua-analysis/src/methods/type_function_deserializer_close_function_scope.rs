use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  type_function_deserializer::TypeFunctionDeserializer,
  type_function_function_type::TypeFunctionFunctionType,
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证 `f` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn close_function_scope(&mut self, f: *mut TypeFunctionFunctionType) {
    let generics_len = unsafe { (*f).generics.len() };
    if generics_len > 0 {
      let generics_start = self.generic_types.len() - generics_len;
      LUAU_ASSERT!(self.generic_types.len() >= generics_len);
      self.generic_types.drain(generics_start..);
    }

    let generic_packs_len = unsafe { (*f).generic_packs.len() };
    if generic_packs_len > 0 {
      let generic_packs_start = self.generic_packs.len() - generic_packs_len;
      LUAU_ASSERT!(self.generic_packs.len() >= generic_packs_len);
      self.generic_packs.drain(generic_packs_start..);
    }
  }
}
