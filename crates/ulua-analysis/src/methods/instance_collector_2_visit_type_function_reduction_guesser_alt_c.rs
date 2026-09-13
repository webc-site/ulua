use crate::{
  records::{
    instance_collector_2::InstanceCollector2,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl InstanceCollector2 {
  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    _it: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.tps.push_front(tp);
    true
  }
}
