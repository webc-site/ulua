use alloc::vec::Vec;

use crate::{
  enums::status_require_impl::Status, records::runtime_error_handler::RuntimeErrorHandler,
};

/// 对应 cpp `struct ResolvedRequire`：成员在 cpp 里都是 `std::string`（字节串），
/// 故此处用 `Vec<u8>` 保字节语义，只在推入 Lua 栈的 FFI 边界补 NUL。
pub struct ResolvedRequire {
  pub status: Status,
  pub chunkname: Vec<u8>,
  pub loadname: Vec<u8>,
  pub cache_key: Vec<u8>,
  pub error: Vec<u8>,
}

impl ResolvedRequire {
  /// 单载荷结果骨架：仅 `status` 与 `error` 有值，其余字节串成员恒空
  /// （cpp `fromErrorMessage` / `fromErrorHandler` / 缓存命中 Cached 返回三处共用）。
  pub(crate) fn with_error(status: Status, error: Vec<u8>) -> Self {
    ResolvedRequire {
      status,
      chunkname: Vec::new(),
      loadname: Vec::new(),
      cache_key: Vec::new(),
      error,
    }
  }

  /// 对应 cpp `ResolvedRequire::fromErrorMessage`：消息为固定文案字节。
  pub fn from_error_message(message: &[u8]) -> ResolvedRequire {
    Self::with_error(Status::ErrorReported, message.to_vec())
  }

  /// 对应 cpp `ResolvedRequire::fromErrorHandler`：仅 error 成员携带报告内容。
  pub fn from_error_handler(error_handler: &RuntimeErrorHandler) -> ResolvedRequire {
    Self::with_error(
      Status::ErrorReported,
      error_handler.get_reported_error().to_vec(),
    )
  }
}
