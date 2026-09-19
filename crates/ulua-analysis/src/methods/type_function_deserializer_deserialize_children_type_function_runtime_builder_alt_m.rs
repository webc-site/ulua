//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:981-988`
//!
//! ```cpp
//! void deserializeChildren(TypeFunctionTableType* m2, MetatableType* m1)
//! {
//!     TypeFunctionTypeId temp = typeFunctionRuntime->type_arena.allocate(TypeFunctionTableType{m2->props, m2->indexer});
//!     m1->table = shallowDeserialize(temp);
//!
//!     if (m2->metatable.has_value())
//!         m1->metatable = shallowDeserialize(*m2->metatable);
//! }
//! ```
use crate::{
  records::{
    metatable_type::MetatableType, type_function_deserializer::TypeFunctionDeserializer,
    type_function_table_type::TypeFunctionTableType, type_function_type::TypeFunctionType,
  },
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_variant::TypeFunctionTypeVariant,
  },
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn deserialize_children_type_function_table_type_metatable_type(
    &mut self,
    m2: *mut TypeFunctionTableType,
    m1: *mut MetatableType,
  ) {
    unsafe {
      // `TypeFunctionTableType{m2->props, m2->indexer}` — the 2-arg constructor
      // (metatable defaults to std::nullopt).
      let table = TypeFunctionTableType {
        props: (*m2).props.clone(),
        indexer: (*m2).indexer.clone(),
        metatable: None,
      };
      let temp: TypeFunctionTypeId = (*self.type_function_runtime)
        .type_arena
        .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Table(table)));

      (*m1).table = self.shallow_deserialize_type_function_type_id(temp);

      if let Some(metatable) = (*m2).metatable {
        (*m1).metatable = self.shallow_deserialize_type_function_type_id(metatable);
      }
    }
  }
}
