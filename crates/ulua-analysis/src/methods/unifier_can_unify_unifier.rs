use crate::{
  records::unifier::Unifier,
  type_aliases::{error_vec::ErrorVec, type_id::TypeId},
};

impl Unifier {
  pub fn can_unify_type_id_type_id(&mut self, sub_ty: TypeId, super_ty: TypeId) -> ErrorVec {
    let mut s = self.unifier_make_child_unifier();
    s.try_unify_type_id_type_id_bool_bool_literal_properties(sub_ty, super_ty, false, false, None);
    s.errors
  }
}
