use alloc::{string::String, vec::Vec};

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
  Missing,
  Extra,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MissingProperties {
  pub(crate) super_type: TypeId,
  pub(crate) sub_type: TypeId,
  pub(crate) properties: Vec<String>,
  pub(crate) context: Context,
}

impl MissingProperties {
  pub fn super_type(&self) -> TypeId {
    self.super_type
  }

  pub fn sub_type(&self) -> TypeId {
    self.sub_type
  }

  pub fn properties(&self) -> &[String] {
    &self.properties
  }

  pub fn context(&self) -> Context {
    self.context
  }
}
