use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{function_type::FunctionType, normalizer::Normalizer},
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

impl Normalizer {
  pub fn union_of_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    if get_type_id::<ErrorType>(here).is_some() {
      return Some(here);
    }

    if get_type_id::<ErrorType>(there).is_some() {
      return Some(there);
    }

    let hftv = get_type_id::<FunctionType>(here);
    LUAU_ASSERT!(hftv.is_some());
    let hftv = hftv.unwrap();
    let tftv = get_type_id::<FunctionType>(there);
    LUAU_ASSERT!(tftv.is_some());
    let tftv = tftv.unwrap();

    let h_generics = hftv.generics.clone();
    let t_generics = tftv.generics.clone();
    if h_generics != t_generics {
      return None;
    }

    let h_generic_packs = hftv.generic_packs.clone();
    let t_generic_packs = tftv.generic_packs.clone();
    if h_generic_packs != t_generic_packs {
      return None;
    }

    let arg_types = self.intersection_of_type_packs_internal(hftv.arg_types, tftv.arg_types);
    arg_types?;

    let ret_types = self.union_of_type_packs(hftv.ret_types, tftv.ret_types);
    ret_types?;

    let arg_types_val = arg_types.unwrap();
    let ret_types_val = ret_types.unwrap();

    if arg_types_val == hftv.arg_types && ret_types_val == hftv.ret_types {
      return Some(here);
    }

    if arg_types_val == tftv.arg_types && ret_types_val == tftv.ret_types {
      return Some(there);
    }

    let mut result = FunctionType::function_type_new(arg_types_val, ret_types_val, None, false);
    result.generics = h_generics;
    result.generic_packs = h_generic_packs;

    // SAFETY: self.arena 指向 Normalizer 常驻的类型 arena
    Some(unsafe { (*self.arena).add_type(result) })
  }
}
