use alloc::vec::Vec;

use crate::{
  enums::status_require_impl::Status,
  records::{resolved_require::ResolvedRequire, runtime_error_handler::RuntimeErrorHandler},
};

impl ResolvedRequire {
  /// 对应 cpp `ResolvedRequire::fromErrorHandler`：仅 error 成员携带报告内容。
  pub fn from_error_handler(error_handler: &RuntimeErrorHandler) -> ResolvedRequire {
    ResolvedRequire {
      status: Status::ErrorReported,
      chunkname: Vec::new(),
      loadname: Vec::new(),
      cache_key: Vec::new(),
      error: error_handler.get_reported_error().to_vec(),
    }
  }
}
