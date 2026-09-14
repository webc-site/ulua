use crate::{records::path::Path, type_aliases::component::Component};

impl Path {
  pub fn path_component(component: Component) -> Self {
    Path::path_vector_component(alloc::vec![component])
  }
}
