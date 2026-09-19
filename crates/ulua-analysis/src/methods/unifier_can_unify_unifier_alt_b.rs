use crate::{
  records::unifier::Unifier,
  type_aliases::{error_vec::ErrorVec, type_pack_id::TypePackId},
};

impl Unifier {
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
