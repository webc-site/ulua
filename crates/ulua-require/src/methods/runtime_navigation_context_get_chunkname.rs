use alloc::vec::Vec;

use crate::records::runtime_navigation_context::{
  INITIAL_IDENTIFIER_BUFFER_SIZE, RuntimeNavigationContext,
};

impl RuntimeNavigationContext<'_> {
  /// 当前上下文的 chunkname 字节串（cpp `getChunkname()`）。
  pub fn get_chunkname(&self) -> Option<Vec<u8>> {
    let config = unsafe { self.config.as_ref() }?;
    let writer = config.get_chunkname?;
    self.get_string_from_c_writer(writer, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }
}
