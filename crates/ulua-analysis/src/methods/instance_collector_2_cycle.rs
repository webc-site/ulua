use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    instance_collector_2::InstanceCollector2, type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

impl InstanceCollector2 {
  pub fn cycle(&mut self, ty: TypeId) {
    let t = follow_type_id(ty);
    let it = get_type_id::<TypeFunctionInstanceType>(t);
    if !it.is_none() {
      self.cyclic_instance.insert(t);
    }
  }
}
