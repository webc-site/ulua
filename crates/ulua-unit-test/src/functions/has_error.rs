use ulua_analysis::{
  records::{check_result::CheckResult, type_error::TypeError},
  type_aliases::type_error_data::TypeErrorDataMember,
};

pub fn has_error<T: TypeErrorDataMember>(result: &CheckResult) -> bool {
  result.errors.iter().any(|error| get::<T>(error).is_some())
}

fn get<T: TypeErrorDataMember>(error: &TypeError) -> Option<&T> {
  T::get_if(&error.data)
}
