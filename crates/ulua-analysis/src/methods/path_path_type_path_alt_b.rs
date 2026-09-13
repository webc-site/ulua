use alloc::vec::Vec;

use crate::{records::path::Path, type_aliases::component::Component};
impl Path {
  pub fn path_vector_component(components: Vec<Component>) -> Self {
    Self { components }
  }
}
