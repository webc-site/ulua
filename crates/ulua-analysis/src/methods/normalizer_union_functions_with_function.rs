use crate::{
  records::{
    normalized_function_type::NormalizedFunctionType, normalizer::Normalizer, type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_functions_with_function(
    &mut self,
    heres: &mut NormalizedFunctionType,
    there: TypeId,
  ) {
    self.consume_fuel();

    if heres.is_never() {
      let mut tmps = TypeIds::new();
      tmps.insert_type_id(there);
      heres.parts = tmps;
      return;
    }

    let mut tmps = TypeIds::new();
    let parts = heres.parts.clone();
    for here in parts.order {
      if let Some(fun) = self.union_of_functions(here, there) {
        tmps.insert_type_id(fun);
      } else {
        let builtin_types = unsafe { &*self.builtin_types };
        tmps.insert_type_id(builtin_types.error_recovery_type(there));
      }
    }
    heres.parts = tmps;
  }
}
