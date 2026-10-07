use alloc::string::String;

use crate::{
  functions::get_module_path::strip_module_suffix, records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub(crate) fn get_config_path(&self, filename: &str) -> String {
    // 命中模块后缀时取同目录，否则就在当前路径下找配置文件
    [strip_module_suffix(&self.real_path), "/", filename].concat()
  }
}
