use crate::{
  enums::unify_result::UnifyResult,
  records::{intersection_type::IntersectionType, unifier_2::Unifier2},
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn unify_type_id_intersection_type(
    &mut self,
    sub_ty: TypeId,
    super_intersection: &IntersectionType,
  ) -> UnifyResult {
    let mut result = UnifyResult::Ok;

    for super_part in super_intersection.parts.iter() {
      result &= self.unify_type_id_type_id(sub_ty, *super_part);
    }

    result
  }
}
