use crate::{
  functions::has_unification_too_complex::has_unification_too_complex,
  records::unifier::Unifier,
  type_aliases::{error_vec::ErrorVec, type_id::TypeId},
};

impl Unifier {
  pub fn check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
    &mut self,
    inner_errors: &ErrorVec,
    prop: &str,
    wanted_type: TypeId,
    given_type: TypeId,
  ) {
    if let Some(e) = has_unification_too_complex(inner_errors) {
      self.report_error_type_error(e);
    } else if !inner_errors.is_empty() {
      // 手抄 TypeMismatch 构造收敛到 ext 骨架单点（context 仍先于 error 求值）
      self.unifier_report_type_mismatch_ext(
        wanted_type,
        given_type,
        format!("Property '{}' is not compatible.", prop),
        Some(inner_errors[0].clone()),
      );
    }
  }
}
