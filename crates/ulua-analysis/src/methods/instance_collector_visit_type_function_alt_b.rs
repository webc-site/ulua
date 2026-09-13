use crate::{
  records::{extern_type::ExternType, instance_collector::InstanceCollector},
  type_aliases::type_id::TypeId,
};

impl InstanceCollector {
  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern_type: &ExternType) -> bool {
    false
  }
}
