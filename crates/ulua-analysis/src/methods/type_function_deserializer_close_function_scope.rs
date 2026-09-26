use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  type_function_deserializer::TypeFunctionDeserializer,
  type_function_function_type::TypeFunctionFunctionType,
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证 `f` 非空、对齐，指向类型函数反序列化期间由 arena/持有者保活、地址稳定的
  /// `TypeFunctionFunctionType`；本函数只读取其 `generics`/`generic_packs` 的长度以弹出登记。
  /// cpp `Analysis/src/TypeFunctionRuntimeBuilder.cpp:703`。单线程。
  pub unsafe fn close_function_scope(&mut self, f: *mut TypeFunctionFunctionType) {
    // Safety: `f` 为类型函数反序列化期间存活的函数类型节点（由 deserializer 的 arena/持有者
    // 保活），此处仅只读取 `generics` 的 `Vec::len()`，不解引用可变状态。
    let generics_len = unsafe { (*f).generics.len() };
    if generics_len > 0 {
      let generics_start = self.generic_types.len() - generics_len;
      LUAU_ASSERT!(self.generic_types.len() >= generics_len);
      self.generic_types.drain(generics_start..);
    }

    // Safety: 同上——`f` 指向存活函数类型节点，此处仅只读取 `generic_packs` 的 `Vec::len()`。
    let generic_packs_len = unsafe { (*f).generic_packs.len() };
    if generic_packs_len > 0 {
      let generic_packs_start = self.generic_packs.len() - generic_packs_len;
      LUAU_ASSERT!(self.generic_packs.len() >= generic_packs_len);
      self.generic_packs.drain(generic_packs_start..);
    }
  }
}
