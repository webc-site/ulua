use alloc::vec::Vec;

use crate::{enums::status_require_impl::Status, records::resolved_require::ResolvedRequire};

impl ResolvedRequire {
  /// 对应 cpp `ResolvedRequire::fromErrorMessage`：消息为固定文案字节。
  pub fn from_error_message(message: &[u8]) -> ResolvedRequire {
    ResolvedRequire {
      status: Status::ErrorReported,
      chunkname: Vec::new(),
      loadname: Vec::new(),
      cache_key: Vec::new(),
      error: message.to_vec(),
    }
  }
}
