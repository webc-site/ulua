use alloc::{collections::BTreeSet, string::String};

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownPropButFoundLikeProp {
  pub(crate) table: TypeId,
  pub(crate) key: String,
  pub(crate) candidates: BTreeSet<String>,
}

impl UnknownPropButFoundLikeProp {
  pub fn table(&self) -> TypeId {
    self.table
  }

  pub fn key(&self) -> &str {
    &self.key
  }

  pub fn candidates(&self) -> &BTreeSet<String> {
    &self.candidates
  }
}
