use crate::{
  enums::unify_result::UnifyResult,
  functions::follow_type::follow_type_id,
  records::{intersection_type::IntersectionType, unifier_2::Unifier2},
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn unify_intersection_type_type_id(
    &mut self,
    sub_intersection: &IntersectionType,
    super_ty: TypeId,
  ) -> UnifyResult {
    let super_ty = follow_type_id(super_ty);

    for &sub_option in sub_intersection.parts.iter() {
      let followed_sub_option = follow_type_id(sub_option);
      if super_ty == followed_sub_option {
        return UnifyResult::Ok;
      }
    }

    let mut result = UnifyResult::Ok;

    for sub_part in sub_intersection.parts.iter() {
      result &= self.unify_type_id_type_id(*sub_part, super_ty);
    }

    result
  }
}
