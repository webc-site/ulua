use crate::{
  functions::{follow_type, get_type},
  records::{
    instance_collector::InstanceCollector, type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};
impl InstanceCollector {
  pub fn cycle(&mut self, ty: TypeId) {
    let t = follow_type::follow(ty);

    let it = get_type::get::<TypeFunctionInstanceType>(t);
    if it.is_some() {
      // If we see a type a second time and it's in the type function stack, it's a real cycle
      if self
        .type_function_instance_stack
        .contains(&(t as *const ()))
      {
        self.cyclic_instance.push(t);
      }
    }
  }
}
