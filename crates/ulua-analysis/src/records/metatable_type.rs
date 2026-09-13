use alloc::string::String;
use core::option::Option;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetatableType {
  /// Should always be a TableType.
  pub(crate) table: TypeId,
  /// Should almost always either be a TableType or another MetatableType,
  /// though it is possible for other types (like AnyType and ErrorType) to
  /// find their way here sometimes.
  pub(crate) metatable: TypeId,
  pub(crate) synthetic_name: Option<String>,
}

impl MetatableType {
  pub fn table(&self) -> TypeId {
    self.table
  }

  pub fn metatable(&self) -> TypeId {
    self.metatable
  }

  pub fn synthetic_name(&self) -> Option<&str> {
    self.synthetic_name.as_deref()
  }
}

unsafe impl Send for MetatableType {}
unsafe impl Sync for MetatableType {}
