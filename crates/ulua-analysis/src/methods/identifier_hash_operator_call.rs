use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::records::{identifier::Identifier, identifier_hash::IdentifierHash};
impl IdentifierHash {
  pub fn identifier_hash_operator_call(ident: &Identifier) -> usize {
    let name = ident.name();
    let ctx = ident.ctx() as *const ();

    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let hash_name = hasher.finish() as usize;

    hasher = DefaultHasher::new();
    (ctx as usize).hash(&mut hasher);
    let hash_ctx = hasher.finish() as usize;

    hash_name ^ hash_ctx
  }
}
