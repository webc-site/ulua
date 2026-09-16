use crate::{
  records::{
    extern_type::ExternType, instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
  },
  type_aliases::type_id::TypeId,
};

impl InstantiationQueuerDeprecated {
  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
