use alloc::vec::Vec;

use crate::records::runtime_navigation_context::{
  INITIAL_FILE_BUFFER_SIZE, RuntimeNavigationContext,
};

impl RuntimeNavigationContext<'_> {
  /// 配置文件原始字节（cpp `getConfig()` 返回 `std::string`）：
  /// 交给 ulua-config 解析前才在边界做 UTF-8 转换。
  pub fn get_config(&self) -> Option<Vec<u8>> {
    let config = unsafe { self.config.as_ref() }?;
    let writer = config.get_config?;
    self.get_string_from_c_writer(writer, INITIAL_FILE_BUFFER_SIZE)
  }
}
