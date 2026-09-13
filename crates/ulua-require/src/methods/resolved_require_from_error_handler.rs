use alloc::string::String;

use crate::{
  enums::status_require_impl::Status,
  records::{resolved_require::ResolvedRequire, runtime_error_handler::RuntimeErrorHandler},
};

impl ResolvedRequire {
  pub fn from_error_handler(error_handler: &RuntimeErrorHandler) -> ResolvedRequire {
    ResolvedRequire {
      status: Status::ErrorReported,
      chunkname: String::new(),
      loadname: String::new(),
      cache_key: String::new(),
      error: String::from(error_handler.get_reported_error()),
    }
  }
}
