use alloc::string::String;

use ulua_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone, Default)]
pub struct TypeRehydrationOptions {
  pub(crate) banned_names: DenseHashSet<String>,
  pub(crate) expand_extern_type_props: bool,
}

impl TypeRehydrationOptions {
  pub fn banned_names(&self) -> &DenseHashSet<String> {
    &self.banned_names
  }

  pub fn expand_extern_type_props(&self) -> bool {
    self.expand_extern_type_props
  }
}
