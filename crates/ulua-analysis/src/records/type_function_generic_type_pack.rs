use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TypeFunctionGenericTypePack {
  pub(crate) is_named: bool,
  pub(crate) name: String,
}

impl TypeFunctionGenericTypePack {
  pub fn is_named(&self) -> bool {
    self.is_named
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}
