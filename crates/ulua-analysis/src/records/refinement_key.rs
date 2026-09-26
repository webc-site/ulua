use alloc::string::String;
use core::ptr::null;

use crate::type_aliases::def_id_def::DefId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefinementKey {
  pub(crate) parent: *const RefinementKey,
  pub(crate) def: DefId,
  pub(crate) prop_name: Option<String>,
}

impl Default for RefinementKey {
  fn default() -> Self {
    Self {
      parent: null(),
      def: DefId::NULL,
      prop_name: None,
    }
  }
}

impl RefinementKey {
  pub fn parent(&self) -> *const RefinementKey {
    self.parent
  }

  pub fn def(&self) -> DefId {
    self.def
  }

  pub fn prop_name(&self) -> Option<&String> {
    self.prop_name.as_ref()
  }
}
