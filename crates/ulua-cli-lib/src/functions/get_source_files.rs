use alloc::vec::Vec;

use crate::functions::{
  get_module_path::K_SUFFIXES, is_directory::is_directory, normalize_path::normalize_path,
  traverse_directory::traverse_directory,
};

/// cpp `getSourceFiles(argc, argv)`: 收集非选项参数为源文件列表;
/// 目录参数递归收集 `.lua`/`.luau`; 遇 `--program-args`/`-a` 停止
pub fn get_source_files_from_slice(args: &[impl AsRef<str>]) -> Vec<String> {
  let mut files = Vec::new();

  for arg in args.iter().skip(1) {
    let arg = arg.as_ref();

    if arg == "--program-args" || arg == "-a" {
      return files;
    }

    // '-' 特殊文件, 源码读自 stdin; 其余 '-' 开头参数跳过
    if arg.starts_with('-') && arg.len() > 1 {
      continue;
    }

    let normalized = normalize_path(arg);

    if is_directory(&normalized) {
      traverse_directory(&normalized, |name: &str| {
        // cpp FileUtils.cpp:451-453 `ext == ".lua" || ext == ".luau"`：对本
        // 后缀集合与 getExtension 全等比较逐字节等价；合法扩展名集合单源于
        // VfsNavigator 的 kSuffixes（FileUtils.cpp:166-170 hasFileExtension
        // 的 ends_with 形态），免两处字面量各自漂移
        if K_SUFFIXES.iter().any(|&suffix| name.ends_with(suffix)) {
          files.push(name.to_owned()); // cpp FileUtils.cpp:454 直接 push，回调路径已归一
        }
      });
    } else {
      files.push(normalized);
    }
  }

  files
}
