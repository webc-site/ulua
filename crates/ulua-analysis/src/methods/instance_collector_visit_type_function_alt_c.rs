use core::ffi::c_void;

use ulua_common::DFInt;

use crate::{
  records::{
    generic_type_visitor::GenericTypeVisitorTrait, instance_collector::InstanceCollector,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl InstanceCollector {
  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.type_function_instance_stack.push(tp as *const c_void);

    let guess_depth = DFInt::LuauTypeFamilyUseGuesserDepth.get();
    if guess_depth >= 0 && self.type_function_instance_stack.len() as i32 > guess_depth {
      self.should_guess.insert(tp as *const c_void);
    }

    if !self.recorded_tps.contains(&tp) {
      self.recorded_tps.insert(tp);
      self.tps.push_front(tp);
    }

    for &p in &tfitp.type_arguments {
      self.traverse_type_id(p);
    }

    for &p in &tfitp.pack_arguments {
      self.traverse_type_pack_id(p);
    }

    self.type_function_instance_stack.pop();

    false
  }
}
