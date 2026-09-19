//! Faithful port of `Luau::detail::ArcCollector::add`
//! (`Analysis/src/TopoSortStatements.cpp:220-233`).
//!
//! ```cpp
//! // Adds a dependency from the current node to the named node.
//! void add(const Identifier& name)
//! {
//!     Node** it = map.find(name);
//!     if (it == nullptr)
//!         return;
//!
//!     Node* n = *it;
//!
//!     if (n == currentArc)
//!         return;
//!
//!     n->provides.insert(currentArc);
//!     currentArc->depends.insert(n);
//! }
//! ```
use crate::records::{arc_collector::ArcCollector, identifier::Identifier, node::Node};

impl ArcCollector {
  pub fn add(&mut self, name: &Identifier) {
    // Node** it = map.find(name);
    // if (it == nullptr) return;
    let n: *mut Node = match self.map.find(name) {
      Some(&n) => n,
      None => return,
    };

    // if (n == currentArc) return;
    if n == self.current_arc {
      return;
    }

    // n->provides.insert(currentArc);
    // currentArc->depends.insert(n);
    unsafe {
      (*n).provides.insert(self.current_arc);
      (*self.current_arc).depends.insert(n);
    }
  }
}
