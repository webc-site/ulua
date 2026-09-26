use crate::{
  records::unifier::Unifier,
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};

impl Unifier {
  pub fn can_unify_type_id_type_id(&mut self, sub_ty: TypeId, super_ty: TypeId) -> ErrorVec {
    let mut s = self.unifier_make_child_unifier();
    s.try_unify_type_id_type_id_bool_bool_literal_properties(sub_ty, super_ty, false, false, None);
    s.errors
  }

  pub fn can_unify_type_pack_id_type_pack_id_bool(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    is_function_call: bool,
  ) -> ErrorVec {
    let mut child = self.unifier_make_child_unifier();
    child.try_unify_type_pack_id_type_pack_id_bool(sub_tp, super_tp, is_function_call);
    child.errors
  }
}
