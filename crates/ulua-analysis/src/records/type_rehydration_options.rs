use alloc::string::String;

use ulua_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct TypeRehydrationOptions {
  pub(crate) banned_names: DenseHashSet<String>,
  pub(crate) expand_extern_type_props: bool,
}

impl Default for TypeRehydrationOptions {
  fn default() -> Self {
    Self {
      banned_names: DenseHashSet::new(String::new()),
      expand_extern_type_props: false,
    }
  }
}

impl TypeRehydrationOptions {
  pub fn banned_names(&self) -> &DenseHashSet<String> {
    &self.banned_names
  }

  pub fn expand_extern_type_props(&self) -> bool {
    self.expand_extern_type_props
  }
}
