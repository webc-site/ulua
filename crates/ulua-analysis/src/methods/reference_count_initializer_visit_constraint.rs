use crate::{
  records::{free_type::FreeType, reference_count_initializer::ReferenceCountInitializer},
  type_aliases::type_id::TypeId,
};

impl ReferenceCountInitializer {
  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const FreeType&)`
  /// (Constraint.cpp:26-30).
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, _free_type: &FreeType) -> bool {
    unsafe {
      (*self.mutated_types).insert_type_id(ty);
    }
    false
  }
}
