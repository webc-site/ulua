use core::ffi::c_void;

use ulua_common::DFInt;

use crate::{
  records::{
    generic_type_visitor::GenericTypeVisitorTrait, instance_collector::InstanceCollector,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};
impl InstanceCollector {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    self.type_function_instance_stack.push(ty as *const c_void);

    let guess_depth = DFInt::LuauTypeFamilyUseGuesserDepth.get();
    if guess_depth >= 0 && self.type_function_instance_stack.len() as i32 > guess_depth {
      self.should_guess.insert(ty as *const c_void);
    }

    if !self.recorded_tys.contains(&ty) {
      self.recorded_tys.insert(ty);
      self.tys.push_front(ty);
    }

    for &p in &tfit.type_arguments {
      self.traverse_type_id(p);
    }

    for &p in &tfit.pack_arguments {
      self.traverse_type_pack_id(p);
    }

    self.type_function_instance_stack.pop();

    false
  }
}
