//! Source: `Analysis/src/Unifier2.cpp:407-438` — `Unifier2::unify_(TypeId, const FunctionType*)`.

use crate::{
  enums::unify_result::UnifyResult,
  functions::{
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
  },
  records::{
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    unifier_2::Unifier2,
  },
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn unify_type_id_function_type(
    &mut self,
    sub_ty: TypeId,
    super_fn: &FunctionType,
  ) -> UnifyResult {
    // C++ Unifier2.cpp:408 get<FunctionType>(subTy) 无空检查，调用方保证 subTy 为函数类型
    let sub_fn = get_type_id::<FunctionType>(sub_ty).unwrap();

    let should_instantiate = (super_fn.generics.is_empty() && !sub_fn.generics.is_empty())
      || (super_fn.generic_packs.is_empty() && !sub_fn.generic_packs.is_empty());

    if should_instantiate {
      for &generic in sub_fn.generics.iter() {
        let generic = follow_type_id(generic);
        if let Some(r#gen) = get_type_id::<GenericType>(generic) {
          let fresh = self.fresh_type(self.scope, r#gen.polarity);
          *self.generic_substitutions.get_or_insert(generic) = fresh;
        }
      }

      for &generic_pack in sub_fn.generic_packs.iter() {
        let generic_pack = unsafe { follow_type_pack_id(generic_pack) };
        if let Some(r#gen) = get_type_pack_id::<GenericTypePack>(generic_pack) {
          let fresh = self.fresh_type_pack(self.scope, r#gen.polarity);
          *self.generic_pack_substitutions.get_or_insert(generic_pack) = fresh;
        }
      }
    }

    let arg_result = self.unify_type_pack_id_type_pack_id(super_fn.arg_types, sub_fn.arg_types);
    let ret_result = self.unify_type_pack_id_type_pack_id(sub_fn.ret_types, super_fn.ret_types);
    arg_result & ret_result
  }
}
