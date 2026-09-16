//! Source: `Analysis/include/Luau/TypePath.h:139` + `Analysis/src/TypePath.cpp:72-122` (hand-ported)
use alloc::vec::Vec;

use crate::type_aliases::component::Component;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Path {
  pub components: Vec<Component>,
}

impl Path {
  /// C++ `explicit Path(Component component)` — single-component path
  pub fn from_component(component: Component) -> Path {
    Path {
      components: alloc::vec![component],
    }
  }

  /// C++ `explicit Path(std::vector<Component> components)`
  pub fn from_components(components: Vec<Component>) -> Path {
    Path { components }
  }

  pub fn append(&self, suffix: &Path) -> Path {
    let mut joined = self.components.clone();
    joined.reserve(suffix.components.len());
    joined.extend(suffix.components.iter().cloned());
    Path { components: joined }
  }

  pub fn push(&self, component: Component) -> Path {
    let mut joined = self.components.clone();
    joined.push(component);
    Path { components: joined }
  }

  pub fn push_front(&self, component: Component) -> Path {
    let mut joined = Vec::with_capacity(self.components.len() + 1);
    joined.push(component);
    joined.extend(self.components.iter().cloned());
    Path { components: joined }
  }

  pub fn pop(&self) -> Path {
    if self.path_empty() {
      return Path::default(); // kEmpty
    }
    let mut popped = self.components.clone();
    popped.pop();
    Path { components: popped }
  }

  pub fn last(&self) -> Option<Component> {
    self.components.last().cloned()
  }

  // C++ Path::empty; `empty` collides with nothing but keep the pinned name
  pub fn path_empty(&self) -> bool {
    self.components.is_empty()
  }

  pub fn operator_eq(&self, other: &Path) -> bool {
    self.components == other.components
  }
}
