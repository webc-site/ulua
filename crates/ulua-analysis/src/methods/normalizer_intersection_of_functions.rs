use crate::{
  functions::get_type,
  records::{function_type::FunctionType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};
impl Normalizer {
  pub fn intersection_of_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    let hftv = get_type::get::<FunctionType>(here)?;
    let tftv = get_type::get::<FunctionType>(there)?;

    if hftv.generics != tftv.generics {
      return None;
    }
    if hftv.generic_packs != tftv.generic_packs {
      return None;
    }

    let (arg_types, ret_types) = if hftv.ret_types == tftv.ret_types {
      let arg_types = self.union_of_type_packs(hftv.arg_types, tftv.arg_types)?;
      (arg_types, hftv.ret_types)
    } else if hftv.arg_types == tftv.arg_types {
      let ret_types = self.intersection_of_type_packs_internal(hftv.arg_types, tftv.arg_types)?;
      (hftv.arg_types, ret_types)
    } else {
      return None;
    };

    if arg_types == hftv.arg_types && ret_types == hftv.ret_types {
      return Some(here);
    }
    if arg_types == tftv.arg_types && ret_types == tftv.ret_types {
      return Some(there);
    }

    let mut result = FunctionType::function_type_new(arg_types, ret_types, None, false);
    result.generics = hftv.generics.clone();
    result.generic_packs = hftv.generic_packs.clone();

    // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程驱动无并存别名。
    Some(self.wired_arena_mut().add_type(result))
  }
}
