use crate::{
  records::{blocked_type::BlockedType, reference_count_initializer::ReferenceCountInitializer},
  type_aliases::type_id::TypeId,
};

impl ReferenceCountInitializer {
  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const BlockedType&)`
  /// (Constraint.cpp:32-36).
  pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _blocked_type: &BlockedType) -> bool {
    unsafe {
      (*self.mutated_types).insert_type_id(ty);
    }
    false
  }
}
