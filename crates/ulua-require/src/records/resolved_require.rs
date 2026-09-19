use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};

use crate::{enums::status_require_impl::Status, functions::display::lossy};

/// 对应 cpp `struct ResolvedRequire`：成员在 cpp 里都是 `std::string`（字节串），
/// 故此处用 `Vec<u8>` 保字节语义，只在推入 Lua 栈的 FFI 边界补 NUL。
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResolvedRequire {
  pub(crate) status: Status,
  pub(crate) chunkname: Vec<u8>,
  pub(crate) loadname: Vec<u8>,
  pub(crate) cache_key: Vec<u8>,
  pub(crate) error: Vec<u8>,
}

/// Debug 输出走 lossy 文本视图（展示用途），字段本体仍是原始字节。
impl Debug for ResolvedRequire {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("ResolvedRequire")
      .field("status", &self.status)
      .field("chunkname", &lossy(&self.chunkname))
      .field("loadname", &lossy(&self.loadname))
      .field("cache_key", &lossy(&self.cache_key))
      .field("error", &lossy(&self.error))
      .finish()
  }
}
