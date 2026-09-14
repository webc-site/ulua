use alloc::string::String;

use crate::{enums::status_require_impl::Status, records::resolved_require::ResolvedRequire};

impl ResolvedRequire {
  pub fn from_error_message(message: &str) -> ResolvedRequire {
    ResolvedRequire {
      status: Status::ErrorReported,
      chunkname: String::new(),
      loadname: String::new(),
      cache_key: String::new(),
      error: String::from(message),
    }
  }
}
