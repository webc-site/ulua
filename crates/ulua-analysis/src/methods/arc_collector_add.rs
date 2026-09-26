//! Adds a dependency arc from the node currently being visited to the node that
//! declares `name`. Mirrors `Luau::detail::ArcCollector::add`
//! (`Analysis/src/TopoSortStatements.cpp:220-233`), now free of raw pointers:
//! both endpoints are arena indices.
use crate::records::{arc_collector::ArcCollector, identifier::Identifier};

impl ArcCollector<'_> {
  pub fn add(&mut self, name: &Identifier) {
    // `map.find(name)` — unknown identifier: nothing to link.
    let Some(&to) = self.map.find(name) else {
      return;
    };
    // `currentArc` is always positioned by `toposort` before any visit runs.
    let Some(from) = self.current_arc else {
      return;
    };
    // Self-dependency arcs are meaningless (and would pin the node).
    if to == from {
      return;
    }

    self.arena[to].provides.insert(from);
    self.arena[from].depends.insert(to);
  }
}
