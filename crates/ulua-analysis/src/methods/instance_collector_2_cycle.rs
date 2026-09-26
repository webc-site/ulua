use crate::{
  functions::{follow_type, get_type},
  records::{
    instance_collector_2::InstanceCollector2, type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

impl InstanceCollector2 {
  pub fn cycle(&mut self, ty: TypeId) {
    let t = follow_type::follow(ty);
    let it = get_type::get::<TypeFunctionInstanceType>(t);
    if it.is_some() {
      self.cyclic_instance.insert(t);
    }
  }
}
