use crate::{
  functions::allows_no_return_values::allows_no_return_values,
  records::type_checker_2::TypeChecker2, type_aliases::type_pack_id::TypePackId,
};

impl TypeChecker2 {
  pub fn allows_no_return_values(&self, tp: TypePackId) -> bool {
    allows_no_return_values(tp)
  }
}
