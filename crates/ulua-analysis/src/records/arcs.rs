//! Faithful port of `Luau::detail::Arcs` (`Analysis/src/TopoSortStatements.cpp:72`).
//!
//! ```cpp
//! struct Arcs
//! {
//!     std::set<Node*> provides;
//!     std::set<Node*> depends;
//! };
//! ```
//!
//! `std::set<Node*>` is an ordered set keyed by the raw node pointer; the C++
//! code relies only on insert / erase / find / empty, all of which a Rust
//! `BTreeSet<*mut Node>` provides with identical semantics (raw pointers have a
//! total `Ord`).

use alloc::collections::BTreeSet;

use crate::records::node::Node;

#[derive(Debug, Clone, Default)]
pub struct Arcs {
  pub(crate) provides: BTreeSet<*mut Node>,
  pub(crate) depends: BTreeSet<*mut Node>,
}

impl Arcs {
  pub fn new() -> Self {
    Self {
      provides: BTreeSet::new(),
      depends: BTreeSet::new(),
    }
  }
}
