use core::ffi::c_void;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    instance_collector::InstanceCollector, type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};
impl InstanceCollector {
  pub fn cycle(&mut self, ty: TypeId) {
    let t = follow_type_id(ty);

    let it = get_type_id::<TypeFunctionInstanceType>(t);
    if !it.is_none() {
      // If we see a type a second time and it's in the type function stack, it's a real cycle
      if self
        .type_function_instance_stack
        .contains(&(t as *const c_void))
      {
        self.cyclic_instance.push(t);
      }
    }
  }
}
