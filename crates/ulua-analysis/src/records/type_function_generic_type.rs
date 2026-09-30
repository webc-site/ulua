use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TypeFunctionGenericType {
  pub(crate) is_named: bool,
  pub(crate) is_pack: bool,
  pub(crate) name: String,
}

impl TypeFunctionGenericType {
  pub fn is_named(&self) -> bool {
    self.is_named
  }

  pub fn is_pack(&self) -> bool {
    self.is_pack
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}
