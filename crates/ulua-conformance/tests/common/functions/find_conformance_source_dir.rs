use std::{
  env::{current_dir, var_os},
  path::Path,
};

/// conformance fixture 目录的定位，对应 cpp `tests/Conformance.test.cpp:296-304`。
///
/// 上游顺序：`findConformanceSourceDir()`（从 cwd 逐级向上找 Roblox 内部布局的
/// `Client/content`）→ 环境变量 `LUAU_CONFORMANCE_SOURCE_DIR` → 字面量
/// `Client/Luau/tests/conformance`。
///
/// 本端口的差异只有一处：fixture 随 crate vendored（`crates/ulua-conformance/conformance`），
/// 上游那段 cwd 向上遍历在本仓布局下永不命中（仓库里没有 `Client/content`），因此不移植；
/// 环境变量相应提到最前，成为“指向另一份 fixture 集合”的唯一逃生门（CI 或本地想换一套
/// 脚本时使用）。找不到目录时不做二次猜测 —— 与上游一样由后续读取失败给出报错。
pub fn find_conformance_source_dir() -> String {
  // 与上游 `std::getenv` 等价：变量存在即生效；非 UTF-8 字节按 lossy 处理（路径只用于
  // 打开文件与拼报错信息）。
  if let Some(dir) = var_os("LUAU_CONFORMANCE_SOURCE_DIR") {
    return dir.to_string_lossy().into_owned();
  }

  let vendored = concat!(env!("CARGO_MANIFEST_DIR"), "/conformance");
  if Path::new(vendored).is_dir() {
    return vendored.to_owned();
  }
  if let Ok(cwd) = current_dir() {
    let candidate = cwd.join("crates/ulua-conformance/conformance");
    if candidate.is_dir() {
      return candidate.to_string_lossy().into_owned();
    }
    let candidate2 = cwd.join("conformance");
    if candidate2.is_dir() {
      return candidate2.to_string_lossy().into_owned();
    }
  }
  vendored.to_owned()
}
