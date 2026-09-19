use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{function_type::FunctionType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_saturated_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    let hftv = get_type_id::<FunctionType>(here)?;

    let tftv = get_type_id::<FunctionType>(there)?;

    if hftv.generics != tftv.generics {
      return None;
    }

    if hftv.generic_packs != tftv.generic_packs {
      return None;
    }

    let arg_types = self.union_of_type_packs(hftv.arg_types, tftv.arg_types)?;
    let ret_types = self.union_of_type_packs(hftv.ret_types, tftv.ret_types)?;

    let mut result = FunctionType::function_type_new(arg_types, ret_types, None, false);
    result.generics = hftv.generics.clone();
    result.generic_packs = hftv.generic_packs.clone();

    // SAFETY: self.arena 指向 Normalizer 常驻的类型 arena
    Some(unsafe { (*self.arena).add_type(result) })
  }
}
