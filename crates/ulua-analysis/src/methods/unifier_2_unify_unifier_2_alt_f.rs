use crate::{
  enums::unify_result::UnifyResult,
  functions::{are_compatible::are_compatible, follow_type::follow_type_id},
  records::{unifier_2::Unifier2, union_type::UnionType},
  type_aliases::type_id::TypeId,
};
impl Unifier2 {
  pub fn unify_type_id_union_type(
    &mut self,
    sub_ty: TypeId,
    super_union: &UnionType,
  ) -> UnifyResult {
    let sub_ty = follow_type_id(sub_ty);

    // T <: T | U1 | U2 | ... | Un is trivially true, so we don't gain any information by unifying
    for super_option in super_union.options.iter() {
      let followed_super_option = follow_type_id(*super_option);
      if sub_ty == followed_super_option {
        return UnifyResult::Ok;
      }
    }

    let mut result = UnifyResult::Ok;

    // if the occurs check fails for any option, it fails overall
    for super_option in super_union.options.iter() {
      if unsafe { are_compatible(sub_ty, *super_option) } {
        result &= self.unify_type_id_type_id(sub_ty, *super_option);
      }
    }

    result
  }
}
