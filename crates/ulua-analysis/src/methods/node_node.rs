//! Faithful port of `Luau::detail::Node::Node`
//! (`Analysis/src/TopoSortStatements.cpp:83-87`).
//!
//! ```cpp
//! Node(const std::optional<Identifier>& name, AstStat* el)
//!     : name(name)
//!     , element(el)
//! {
//! }
//! ```
//!
//! The constructor only member-initializes `name` and `element`; the inherited
//! `Arcs` sets (`provides` / `depends`) are value-initialized empty. This
//! in-place initializer assigns those two members; the empty sets are produced
//! by [`Node::new`](crate::records::node::Node::new) at allocation. Construction
//! in `toposort` goes through `Node::new`, which mirrors this body exactly.
use ulua_ast::records::ast_stat::AstStat;

use crate::records::{identifier::Identifier, node::Node};

impl Node {
  pub fn node(&mut self, name: Option<Identifier>, el: *mut AstStat) {
    self.name = name;
    self.element = el;
  }
}
