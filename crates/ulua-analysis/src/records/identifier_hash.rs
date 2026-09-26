use ulua_common::{collections::fast_hash, records::dense_hash_table::DenseHasher};

use crate::records::identifier::Identifier;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct IdentifierHash;

impl IdentifierHash {
  pub const fn new() -> Self {
    Self
  }

  #[inline]
  pub fn hash_identifier(ident: &Identifier) -> usize {
    let name = ident.name();
    let ctx = ident.ctx() as *const ();

    let hash_name = fast_hash(&name);
    let hash_ctx = fast_hash(&(ctx as usize));

    hash_name ^ hash_ctx
  }

  #[inline]
  pub fn identifier_hash_operator_call(ident: &Identifier) -> usize {
    Self::hash_identifier(ident)
  }

  #[inline]
  pub fn operator_call(&self, ident: &Identifier) -> usize {
    Self::hash_identifier(ident)
  }
}

impl DenseHasher<Identifier> for IdentifierHash {
  #[inline]
  fn hash(&self, key: &Identifier) -> usize {
    Self::hash_identifier(key)
  }
}
