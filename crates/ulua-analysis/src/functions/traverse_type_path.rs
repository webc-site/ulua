use crate::{
  functions::{follow_type, follow_type_pack},
  records::{
    builtin_types::BuiltinTypes, path::Path, traversal_state::TraversalState, type_arena::TypeArena,
  },
  type_aliases::{
    component::Component, type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId,
  },
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

pub(crate) fn traverse_type_pack_root(
  root: TypePackId,
  path: &Path,
  builtin_types: &BuiltinTypes,
  arena: &mut TypeArena,
) -> Option<TypeOrPack> {
  let mut state = TraversalState::traversal_state_type_pack_id_not_null_builtin_types_type_arena(
    follow_type_pack::follow(root),
    builtin_types,
    arena,
  );
  if traverse(&mut state, path) {
    if state.encountered_error_suppression {
      return Some(TypeOrPack::V0(builtin_types.error_type));
    }
    Some(state.current)
  } else {
    None
  }
}

pub fn traverse_type_root(
  root: TypeId,
  path: &Path,
  builtin_types: &BuiltinTypes,
  arena: &mut TypeArena,
) -> Option<TypeOrPack> {
  let mut state = TraversalState::traversal_state_type_id_not_null_builtin_types_type_arena(
    follow_type::follow(root),
    builtin_types,
    arena,
  );
  if traverse(&mut state, path) {
    Some(state.current)
  } else {
    None
  }
}
