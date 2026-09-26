use alloc::string::String;

use crate::{
  functions::to_string_error::to_string_type_error,
  records::{dcr_logger::DcrLogger, error_snapshot::ErrorSnapshot, type_error::TypeError},
};

impl DcrLogger {
  pub fn capture_generation_error(&mut self, error: &TypeError) {
    let stringified_error: String = to_string_type_error(error);
    self.generation_log.errors.push(ErrorSnapshot {
      message: stringified_error,
      location: error.location,
    });
  }
}
