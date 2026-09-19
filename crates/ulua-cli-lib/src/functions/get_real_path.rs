use alloc::{format, string::String};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    get_module_path::{K_INIT_SUFFIXES, K_SUFFIXES},
    is_directory::is_directory,
    is_file::is_file,
  },
  records::resolved_real_path::ResolvedRealPath,
};

/// 扫描候选后缀（镜像 cpp `kSuffixes`/`kInitSuffixes` 两段循环）:
/// `Ok(Some(suffix))` 唯一命中、`Ok(None)` 未命中、`Err(Ambiguous)` 多义。
/// cpp 循环不提前退出, 命中后续扫以暴露二次命中, 此处一致。
fn scan_suffixes<'a>(
  module_path: &str,
  potential_suffixes: &[&'a str],
) -> Result<Option<&'a str>, NavigationStatus> {
  let mut hit: Option<&'a str> = None;
  for &potential_suffix in potential_suffixes {
    if !is_file(&format!("{module_path}{potential_suffix}")) {
      continue;
    }
    if hit.is_some() {
      return Err(NavigationStatus::Ambiguous);
    }
    hit = Some(potential_suffix);
  }
  Ok(hit)
}

pub fn get_real_path(module_path: String) -> ResolvedRealPath {
  let last_slash = module_path.rfind('/');
  LUAU_ASSERT!(last_slash.is_some());

  let last_component = match last_slash {
    Some(idx) => &module_path[idx + 1..],
    None => "",
  };

  // 常规模块后缀（`init` 末段交给下方目录分支）
  let mut suffix = if last_component == "init" {
    None
  } else {
    match scan_suffixes(&module_path, K_SUFFIXES) {
      Err(status) => return ResolvedRealPath::new(status, String::new()),
      Ok(hit) => hit,
    }
  };

  if is_directory(&module_path) {
    if suffix.is_some() {
      return ResolvedRealPath::new(NavigationStatus::Ambiguous, String::new());
    }

    match scan_suffixes(&module_path, K_INIT_SUFFIXES) {
      Err(status) => return ResolvedRealPath::new(status, String::new()),
      // 目录自身即模块：未命中 init 后缀时以空后缀指向目录
      Ok(hit) => suffix = Some(hit.unwrap_or_default()),
    }
  }

  let Some(suffix) = suffix else {
    return ResolvedRealPath::new(NavigationStatus::NotFound, String::new());
  };

  let mut result_path = module_path;
  result_path.push_str(suffix);
  ResolvedRealPath::new(NavigationStatus::Success, result_path)
}
