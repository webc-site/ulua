//! 用例发现与过滤：目录扫描、按 id 过滤、目录定位。

use std::{
  env, fs, io,
  path::{Path, PathBuf},
};

/// 基准测试用例元数据（id 即文件 stem，源码一次性读入内存）。
pub struct BenchMeta {
  pub id: String,
  pub src: String,
}

/// 扫描 `cases_dir` 下指定扩展名（如 `lua`/`luau`）的用例，按 id 排序返回。
fn scan_cases(cases_dir: &Path, ext: &str) -> Result<Vec<BenchMeta>, io::Error> {
  let mut cases = Vec::new();
  for entry in fs::read_dir(cases_dir)? {
    let path = entry?.path();
    if path.extension().is_none_or(|e| e != ext) {
      continue;
    }
    let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
      continue;
    };
    // 读不进的用例（权限/竞态删除）跳过，但留一行说明，不静默吞掉。
    match fs::read_to_string(&path) {
      Ok(src) => cases.push(BenchMeta {
        id: stem.to_owned(),
        src,
      }),
      Err(err) => eprintln!("跳过无法读取的用例 {}: {err}", path.display()),
    }
  }
  cases.sort_unstable_by(|a, b| a.id.cmp(&b.id));
  Ok(cases)
}

/// 扫描用例目录并保证非空：读取失败带目录上下文；空目录直接报错（含扩展名），
/// 不静默产出空表。
pub fn scan_group_cases(cases_dir: &Path, ext: &str) -> Result<Vec<BenchMeta>, io::Error> {
  let cases = scan_cases(cases_dir, ext).inspect_err(|err| match err.kind() {
    io::ErrorKind::NotFound => eprintln!("错误: 找不到用例目录 {cases_dir:?}"),
    _ => eprintln!("错误: 无法读取用例目录 {cases_dir:?}: {err}"),
  })?;
  if cases.is_empty() {
    return Err(io::Error::other(format!(
      "未在 {cases_dir:?} 下找到任何 .{ext} 测试用例"
    )));
  }
  Ok(cases)
}

/// 按用例 id 过滤；空过滤器 = 全部，保持扫描顺序。
pub fn filter_cases<'a>(all: &'a [BenchMeta], filters: &[String]) -> Vec<&'a BenchMeta> {
  if filters.is_empty() {
    all.iter().collect()
  } else {
    all
      .iter()
      .filter(|c| filters.iter().any(|f| f == &c.id))
      .collect()
  }
}

/// 定位用例目录：依次尝试候选相对路径（先仓库根视角、后 runner 目录内视角），
/// 全部落空时回退第一个候选，让 `scan_cases` 报出真实错误。
pub fn locate_dir(candidates: &[&str]) -> PathBuf {
  let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
  candidates
    .iter()
    .map(|candidate| current_dir.join(candidate))
    .find(|dir| dir.is_dir())
    .unwrap_or_else(|| PathBuf::from(candidates.first().copied().unwrap_or("benchmarks/cases")))
}
