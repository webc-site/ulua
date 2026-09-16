use crate::{
  records::{
    reference_count_initializer::ReferenceCountInitializer, type_ids::TypeIds,
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::type_pack_ids::TypePackIds,
};

impl ReferenceCountInitializer {
  pub fn reference_count_initializer_reference_count_initializer(
    mutated_types: *mut TypeIds,
    mutated_type_packs: *mut TypePackIds,
  ) -> Self {
    ReferenceCountInitializer {
      base: TypeOnceVisitor::new("ReferenceCountInitializer".to_string(), true),
      mutated_types,
      mutated_type_packs,
      traverse_into_type_functions: true,
    }
  }
}
