use alloc::vec::Vec;

use crate::{records::path::Path, type_aliases::component::Component};

impl Path {
  pub fn new() -> Self {
    Self {
      components: Vec::new(),
    }
  }

  pub fn path_vector_component(components: Vec<Component>) -> Self {
    Self { components }
  }

  pub fn path_component(component: Component) -> Self {
    Path::path_vector_component(alloc::vec![component])
  }
}
