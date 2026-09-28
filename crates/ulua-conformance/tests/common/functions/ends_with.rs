/// 与标准库 `str::ends_with` 等价（原手写字节比较已收敛到标准实现）。
pub fn ends_with(str: &str, suffix: &str) -> bool {
  str.ends_with(suffix)
}
