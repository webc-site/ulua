use ulua_common::dfint;

use crate::{
  records::{
    extern_type::ExternType, generic_type_visitor::GenericTypeVisitorTrait,
    instance_collector::InstanceCollector, type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl InstanceCollector {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    self.type_function_instance_stack.push(ty as *const ());

    let guess_depth = dfint::LuauTypeFamilyUseGuesserDepth.get();
    if guess_depth >= 0 && self.type_function_instance_stack.len() as i32 > guess_depth {
      self.should_guess.insert(ty as *const ());
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

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern_type: &ExternType) -> bool {
    false
  }

  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.type_function_instance_stack.push(tp as *const ());

    let guess_depth = dfint::LuauTypeFamilyUseGuesserDepth.get();
    if guess_depth >= 0 && self.type_function_instance_stack.len() as i32 > guess_depth {
      self.should_guess.insert(tp as *const ());
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
