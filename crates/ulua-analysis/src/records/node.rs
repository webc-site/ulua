//! Faithful port of `Luau::detail::Node` (`Analysis/src/TopoSortStatements.cpp:78`).
//!
//! ```cpp
//! struct Node : Arcs
//! {
//!     std::optional<Identifier> name;
//!     AstStat* element;
//!
//!     Node(const std::optional<Identifier>& name, AstStat* el)
//!         : name(name), element(el) {}
//! };
//! ```
//!
//! C++ `Node` publicly inherits `Arcs`, so `node->provides` / `node->depends`
//! name the base's sets directly. Rust has no struct inheritance, so the two
//! `Arcs` fields are flattened into `Node` (the only observable use of the base
//! is `node->provides` / `node->depends`). A separate [`Arcs`](super::arcs::Arcs)
//! value is still used by `drain` to build a filtered copy of the connectivity.

use alloc::collections::BTreeSet;

use ulua_ast::records::ast_stat::AstStat;

use crate::records::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct Node {
  // Flattened `Arcs` base (`struct Node : Arcs`).
  pub(crate) provides: BTreeSet<*mut Node>,
  pub(crate) depends: BTreeSet<*mut Node>,

  pub(crate) name: Option<Identifier>,
  pub(crate) element: *mut AstStat,
}

impl Node {
  /// `Node(const std::optional<Identifier>& name, AstStat* el)`.
  pub fn new(name: Option<Identifier>, el: *mut AstStat) -> Self {
    Self {
      provides: BTreeSet::new(),
      depends: BTreeSet::new(),
      name,
      element: el,
    }
  }
}
