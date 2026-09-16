use crate::{
  functions::contains_parse_error_name::contains_parse_error_name,
  records::type_checker::TypeChecker,
  type_aliases::{error_vec::ErrorVec, type_error_data::TypeErrorData},
};

impl TypeChecker {
  pub fn prepare_errors_for_display(&mut self, err_vec: &mut ErrorVec) {
    err_vec.retain(|err| !contains_parse_error_name(err));

    for err in err_vec.iter_mut() {
      if let TypeErrorData::UnknownProperty(utk) = err.data.clone() {
        self.diagnose_missing_table_key(&utk, &mut err.data);
      }
    }
  }
}
