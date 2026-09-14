use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, unapplied_type_function::UnappliedTypeFunction,
};

impl ErrorConverter {
  pub fn operator_call_56(&self, _e: &UnappliedTypeFunction) -> String {
    String::from("Type functions always require `<>` when referenced.")
  }
}
