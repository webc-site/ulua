use crate::{
  records::{extern_type::ExternType, reference_count_initializer::ReferenceCountInitializer},
  type_aliases::type_id::TypeId,
};

impl ReferenceCountInitializer {
  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern_type: &ExternType) -> bool {
    false
  }
}
