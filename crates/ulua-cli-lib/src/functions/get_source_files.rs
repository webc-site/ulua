use alloc::vec::Vec;

use crate::functions::{
  get_extension::get_extension, is_directory::is_directory, normalize_path::normalize_path,
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
        let ext = get_extension(name);
        if ext == ".lua" || ext == ".luau" {
          files.push(name.to_string());
        }
      });
    } else {
      files.push(normalized);
    }
  }

  files
}
