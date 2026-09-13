/// Global counter for assigning unique indices to `PendingExpansionType` instances.
/// Mirrors `int PendingExpansionType::nextIndex` in `Analysis/include/Luau/Type.h`.
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

use ulua_ast::records::ast_name::AstName;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};
static NEXT_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PendingExpansionType {
  pub prefix: Option<AstName>,
  pub name: AstName,
  pub type_arguments: Vec<TypeId>,
  pub pack_arguments: Vec<TypePackId>,
  pub index: usize,
}

impl PendingExpansionType {
  pub fn fresh_index() -> usize {
    NEXT_INDEX.fetch_add(1, Ordering::Relaxed)
  }
}
