use alloc::string::String;
use std::env::current_dir;

pub fn get_current_working_directory() -> Option<String> {
  let cwd = current_dir().ok()?;
  // require/VfsNavigator 用 '/' 虚拟路径空间, Windows cwd 归一为 '/' 避免缓存 key 不一致导致递归堆栈溢出
  Some(cwd.to_string_lossy().replace('\\', "/"))
}
