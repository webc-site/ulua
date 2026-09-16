//! @interface-stub
use crate::{
  records::{extern_type::ExternType, instance_collector_2::InstanceCollector2},
  type_aliases::type_id::TypeId,
};

impl InstanceCollector2 {
  pub fn visit_type_id_extern_type(&mut self, ty: TypeId, _extern: &ExternType) -> bool {
    let _ = ty;
    false
  }
}
