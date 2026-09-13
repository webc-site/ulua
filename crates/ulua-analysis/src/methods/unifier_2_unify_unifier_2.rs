use crate::{
  enums::unify_result::UnifyResult, records::unifier_2::Unifier2, type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn unify(&mut self, sub_ty: TypeId, super_ty: TypeId) -> UnifyResult {
    self.iteration_count = 0;
    self.unify_type_id_type_id(sub_ty, super_ty)
  }
}
