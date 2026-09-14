use crate::functions::atoi::atoi;

/// 镜像 cpp 各 CLI 中 `-O<n>`/`-g<n>`/`-t<n>` 的级别解析:
/// `atoi(argv[i] + 2)` 后校验 `[low, high]`, 越界时打印 cpp 同款错误消息。
/// 返回 `None` 表示越界 (调用方按 cpp 返回 1)。
pub fn parse_level_arg(level_str: &str, low: i32, high: i32, label: &str) -> Option<i32> {
  let level = atoi(level_str);
  if (low..=high).contains(&level) {
    Some(level)
  } else {
    eprintln!("Error: {label} level must be between {low} and {high} inclusive.");
    None
  }
}
