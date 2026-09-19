use alloc::vec::Vec;

use crate::records::runtime_navigation_context::{
  INITIAL_IDENTIFIER_BUFFER_SIZE, RuntimeNavigationContext,
};

impl RuntimeNavigationContext<'_> {
  /// 当前上下文的缓存键字节串（cpp `getCacheKey()`）：可能含非 UTF-8 字节。
  pub fn get_cache_key(&self) -> Option<Vec<u8>> {
    let config = unsafe { self.config.as_ref() }?;
    let writer = config.get_cache_key?;
    self.get_string_from_c_writer(writer, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }
}
