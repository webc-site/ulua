use alloc::vec::Vec;

use ulua_analysis::{
  functions::to_string_error::to_string_type_error, records::type_error::TypeError,
};

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn validate_errors(&mut self, errors: &Vec<TypeError>) {
    for error in errors {
      let _ = to_string_type_error(error);
    }
  }
}
