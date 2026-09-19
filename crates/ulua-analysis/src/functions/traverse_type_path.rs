//! Source: `Analysis/src/TypePath.cpp:969-984` (hand-ported)
use crate::{
  records::{path::Path, traversal_state::TraversalState},
  type_aliases::component::Component,
};

pub fn traverse(state: &mut TraversalState, path: &Path) -> bool {
  for component in &path.components {
    // C++ `visit(step, component)` — dispatch each Component alternative
    // to the matching `TraversalState::traverse` overload.
    let step_success = match component {
      Component::Property(p) => state.traverse_type_path_property(p),
      Component::Index(i) => state.traverse_type_path_index(i),
      Component::TypeField(f) => state.traverse_type_path_type_field(*f),
      Component::PackField(f) => state.traverse_type_path_pack_field(*f),
      Component::PackSlice(s) => state.traverse_type_path_pack_slice(*s),
      Component::Reduction(r) => state.traverse_type_path_reduction(*r),
      Component::GenericPackMapping(m) => state.traverse_type_path_generic_pack_mapping(*m),
    };

    if !step_success {
      return false;
    }
  }

  true
}
