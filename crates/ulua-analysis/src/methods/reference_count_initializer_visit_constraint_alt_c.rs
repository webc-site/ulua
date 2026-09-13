use crate::{
  records::{
    pending_expansion_type::PendingExpansionType,
    reference_count_initializer::ReferenceCountInitializer,
  },
  type_aliases::type_id::TypeId,
};

impl ReferenceCountInitializer {
  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const PendingExpansionType&)`
  /// (Constraint.cpp:38-42).
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _pending_expansion_type: &PendingExpansionType,
  ) -> bool {
    unsafe {
      (*self.mutated_types).insert_type_id(ty);
    }
    false
  }
}
