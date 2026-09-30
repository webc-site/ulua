//! `metatable_type` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;

use crate::{records::metatable_type::MetatableType, type_aliases::type_id::TypeId};

impl MetatableType {
  pub fn new(table: TypeId, metatable: TypeId) -> Self {
    Self {
      table,
      metatable,
      synthetic_name: None,
    }
  }
}

impl MetatableType {
  pub fn new_named(table: TypeId, metatable: TypeId, synthetic_name: String) -> Self {
    Self {
      table,
      metatable,
      synthetic_name: Some(synthetic_name),
    }
  }
}
