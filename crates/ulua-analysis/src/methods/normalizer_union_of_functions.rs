use crate::{
  functions::get_type,
  records::{function_type::FunctionType, normalizer::Normalizer},
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

impl Normalizer {
  pub fn union_of_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    if get_type::get::<ErrorType>(here).is_some() {
      return Some(here);
    }

    if get_type::get::<ErrorType>(there).is_some() {
      return Some(there);
    }

    // cpp 侧 unionOfFunctions 入口已由变体判定保证双方为 FunctionType（LUAU_ASSERT 同义）。
    let hftv = get_type::get::<FunctionType>(here).expect("调用方已证 here 为 FunctionType 变体");
    let tftv = get_type::get::<FunctionType>(there).expect("调用方已证 there 为 FunctionType 变体");

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

    let arg_types_val = arg_types.expect("上方 `arg_types?` 早退已排除 None");
    let ret_types_val = ret_types.expect("上方 `ret_types?` 早退已排除 None");

    if arg_types_val == hftv.arg_types && ret_types_val == hftv.ret_types {
      return Some(here);
    }

    if arg_types_val == tftv.arg_types && ret_types_val == tftv.ret_types {
      return Some(there);
    }

    let mut result = FunctionType::function_type_new(arg_types_val, ret_types_val, None, false);
    result.generics = h_generics;
    result.generic_packs = h_generic_packs;

    // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程驱动无并存别名。
    Some(self.wired_arena_mut().add_type(result))
  }
}
