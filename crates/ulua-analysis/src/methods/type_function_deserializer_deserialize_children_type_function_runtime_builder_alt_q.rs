//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:1086-1093`
//!
//! ```cpp
//! void deserializeChildren(TypeFunctionTypePack* t2, TypePack* t1)
//! {
//!     for (TypeFunctionTypeId& ty : t2->head)
//!         t1->head.push_back(shallowDeserialize(ty));
//!
//!     if (t2->tail.has_value())
//!         t1->tail = shallowDeserialize(*t2->tail);
//! }
//! ```
use crate::records::{
  type_function_deserializer::TypeFunctionDeserializer,
  type_function_type_pack::TypeFunctionTypePack, type_pack::TypePack,
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn deserialize_children_type_function_type_pack_type_pack(
    &mut self,
    t2: *mut TypeFunctionTypePack,
    t1: *mut TypePack,
  ) {
    unsafe {
      let t2 = &*t2;
      for &ty in &t2.head {
        let deserialized = self.shallow_deserialize_type_function_type_id(ty);
        (*t1).head.push(deserialized);
      }

      if let Some(tail) = t2.tail {
        (*t1).tail = Some(self.shallow_deserialize_type_function_type_pack_id(tail));
      }
    }
  }
}
