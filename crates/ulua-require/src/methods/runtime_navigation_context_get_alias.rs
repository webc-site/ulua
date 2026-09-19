use alloc::vec::Vec;

use crate::records::runtime_navigation_context::{
  INITIAL_IDENTIFIER_BUFFER_SIZE, RuntimeNavigationContext,
};

impl RuntimeNavigationContext<'_> {
  /// 按别名字节查询其映射路径（cpp `getAlias(alias)`）：入参与返回值都是字节串。
  pub fn get_alias(&self, alias: &[u8]) -> Option<Vec<u8>> {
    let config = unsafe { self.config.as_ref() }?;
    let writer = config.get_alias?;
    self.get_string_from_c_writer_with_input(writer, alias, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }
}
