use alloc::string::String;
use std::path::PathBuf;

/// `PathBuf` → `String` 的唯一收口：本 crate 的路径全部源自 `&str`/UTF-8 环境，
/// 回转恒成功；平台编码异常时退化为 lossy（与 cpp 字节串透传语义等价）。
pub(crate) fn into_string(path: PathBuf) -> String {
  path
    .into_os_string()
    .into_string()
    .unwrap_or_else(|os| os.to_string_lossy().into_owned())
}
