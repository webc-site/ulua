use std::{
  ffi::OsString,
  path::{Path, PathBuf},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    get_module_path::{K_INIT_SUFFIXES, K_SUFFIXES},
    is_absolute_path::path_is_absolute,
    is_directory::is_directory_path,
    is_file::is_file_path,
  },
  records::resolved_real_path::ResolvedRealPath,
};

/// 检查路径是否需要前缀修补（如 Windows 下被剥除盘符的根绝对路径 "/Users/..."）
/// 若已含盘符（如 "C:/..."）或已带有 prefix，严禁重复拼接以防 "C:C:/..." 路径破坏。
#[inline]
fn needs_prefix(module_path: &Path, prefix: &str) -> bool {
  if prefix.is_empty() {
    return false;
  }
  let bytes = module_path.as_os_str().as_encoded_bytes();
  if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
    return false;
  }
  if bytes.starts_with(prefix.as_bytes()) {
    return false;
  }
  path_is_absolute(module_path)
}

/// 扫描候选后缀（镜像 cpp `kSuffixes`/`kInitSuffixes` 两段循环）:
/// `Ok(Some(suffix))` 唯一命中、`Ok(None)` 未命中、`Err(Ambiguous)` 多义。
/// 循环内复用单个 String 缓冲区，通过 truncate 实现 O(1) 零重复分配。
fn scan_suffixes<'a>(
  physical_path: &Path,
  potential_suffixes: &[&'a str],
) -> Result<Option<&'a str>, NavigationStatus> {
  let mut hit: Option<&'a str> = None;

  if let Some(base_str) = physical_path.to_str() {
    let mut buf = String::with_capacity(base_str.len() + 16);
    buf.push_str(base_str);
    for &potential_suffix in potential_suffixes {
      buf.push_str(potential_suffix);
      let exists = is_file_path(Path::new(&buf));
      buf.truncate(base_str.len());

      if !exists {
        continue;
      }
      if hit.is_some() {
        return Err(NavigationStatus::Ambiguous);
      }
      hit = Some(potential_suffix);
    }
  } else {
    // 异常编码防御性兜底
    for &potential_suffix in potential_suffixes {
      let mut candidate = physical_path.as_os_str().to_os_string();
      candidate.push(potential_suffix);
      if !is_file_path(Path::new(&candidate)) {
        continue;
      }
      if hit.is_some() {
        return Err(NavigationStatus::Ambiguous);
      }
      hit = Some(potential_suffix);
    }
  }

  Ok(hit)
}

/// 末段组件是否为 `init`（cpp `substr(lastSlash + 1) == "init"` 的字节语义）。
/// `as_encoded_bytes` 在 unix 下即原始字节；上游已把 `'\\'` 归一为 `'/'`，
/// 故仅按 `'/'` 切段与 cpp `find_last_of('/')` 一致。
fn ends_with_init_component(module_path: &Path) -> bool {
  let bytes = module_path.as_os_str().as_encoded_bytes();
  let last_slash = bytes.iter().rposition(|&b| b == b'/');
  LUAU_ASSERT!(last_slash.is_some());

  // cpp `modulePath.substr(lastSlash + 1)`：lastSlash == npos 时是 substr(0)，
  // 即整串（不是空串）。debug 下上面的断言先炸，release 下无斜杠输入走此分支。
  match last_slash {
    Some(idx) => &bytes[idx + 1..] == b"init",
    None => bytes == b"init",
  }
}

/// 解析真实文件路径（cpp 版按值收 `std::string`，此处以 `&Path` 借用零拷贝）。
/// 当 `module_path` 为绝对路径且指定了 `prefix`（如 Windows 盘符 `"C:"`）时，
/// 文件系统存在性检查需拼接 `prefix` 寻址，返回的虚拟路径仍保持原样，
/// 供上层 `update_real_paths` 按需组合。
pub(crate) fn get_real_path(module_path: &Path, prefix: &str) -> ResolvedRealPath {
  let physical_buf;
  let physical_path: &Path = if needs_prefix(module_path, prefix) {
    let mut os = OsString::with_capacity(prefix.len() + module_path.as_os_str().len());
    os.push(prefix);
    os.push(module_path.as_os_str());
    physical_buf = PathBuf::from(os);
    &physical_buf
  } else {
    module_path
  };

  // 常规模块后缀（`init` 末段交给下方目录分支）
  let mut suffix = if ends_with_init_component(module_path) {
    None
  } else {
    match scan_suffixes(physical_path, K_SUFFIXES) {
      Err(status) => return ResolvedRealPath::new(status, PathBuf::new()),
      Ok(hit) => hit,
    }
  };

  if is_directory_path(physical_path) {
    if suffix.is_some() {
      return ResolvedRealPath::new(NavigationStatus::Ambiguous, PathBuf::new());
    }

    match scan_suffixes(physical_path, K_INIT_SUFFIXES) {
      Err(status) => return ResolvedRealPath::new(status, PathBuf::new()),
      // 目录自身即模块：未命中 init 后缀时以空后缀指向目录
      Ok(hit) => suffix = Some(hit.unwrap_or_default()),
    }
  }

  let Some(suffix) = suffix else {
    return ResolvedRealPath::new(NavigationStatus::NotFound, PathBuf::new());
  };

  let mut real_path = module_path.as_os_str().to_os_string();
  real_path.push(suffix);
  ResolvedRealPath::new(NavigationStatus::Success, real_path.into())
}
