//! Node of the toposort dependency graph, addressed by index into the arena
//! (`Vec<Node>`) rather than by raw pointer. See [`NodeId`].

use alloc::collections::BTreeSet;

use ulua_ast::records::ast_stat::AstStat;

use crate::records::identifier::Identifier;

/// A node handle: index into the `Vec<Node>` arena owned by [`toposort`].
///
/// Replaces the C++ `Node*` used as graph identity. `usize` gives a stable,
/// ordered (so `BTreeSet`-compatible) handle without leaking raw pointers into
/// the graph logic.
///
/// [`toposort`]: crate::functions::toposort::toposort
pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Node {
  // Adjacency, by arena index. C++ used `std::set<Node*>`; ordering by index is
  // the deterministic analogue of ordering by address.
  pub(crate) provides: BTreeSet<NodeId>,
  pub(crate) depends: BTreeSet<NodeId>,

  pub(crate) name: Option<Identifier>,
  // The statement itself lives in the AST arena; we only ever pass it around as
  // an opaque identity, so the raw pointer stays (it is graph *payload*, not a
  // graph edge).
  pub(crate) element: *mut AstStat,
}

impl Node {
  pub fn new(name: Option<Identifier>, el: *mut AstStat) -> Self {
    Self {
      provides: BTreeSet::new(),
      depends: BTreeSet::new(),
      name,
      element: el,
    }
  }
}
