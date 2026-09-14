use alloc::string::String;

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
  CannotRead,
  CannotWrite,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyAccessViolation {
  pub(crate) table: TypeId,
  pub(crate) key: String,
  pub(crate) context: Context,
}

impl PropertyAccessViolation {
  pub fn table(&self) -> TypeId {
    self.table
  }

  pub fn key(&self) -> &str {
    &self.key
  }

  pub fn context(&self) -> Context {
    self.context
  }
}

pub use Context as PropertyAccessViolation_Context;
