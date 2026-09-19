use ulua_analysis::{
  records::type_error::TypeError, type_aliases::type_error_data::TypeErrorDataMember,
};

pub fn type_error_data_ref<T: TypeErrorDataMember>(error: &TypeError) -> Option<&T> {
  T::get_if(&error.data)
}
