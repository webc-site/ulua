use alloc::vec::Vec;

/// 按 '/' 或 '\' 切分；与 cpp `splitPath` 一致，保留空段（如前导分隔符产生 ""）
pub fn split_path(path: &str) -> Vec<&str> {
  path.split(['\\', '/']).collect()
}
