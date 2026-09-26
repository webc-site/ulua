use alloc::string::String;

/// 删除 `text` 中所有包含 `needle` 的行。
///
/// 子串查找交给 `str::find`（std 内置向量化搜索器），比 `windows + position`
/// 的逐窗口比较更快；命中后按行边界删除，无命中即提前结束（不再全扫）。
pub fn strip_lines_containing(text: &mut String, needle: &str) {
  // 空 needle 会永远命中，直接返回（同原实现）
  if needle.is_empty() {
    return;
  }

  let mut pos = 0;
  while let Some(found) = text[pos..].find(needle) {
    let actual_pos = pos + found;
    let bytes = text.as_bytes();

    let line_start = bytes[..actual_pos]
      .iter()
      .rposition(|&b| b == b'\n')
      .map_or(0, |idx| idx + 1);

    let line_end = bytes[actual_pos..].iter().position(|&b| b == b'\n');

    match line_end {
      // 命中行是最后一行（无换行符）：直接截断到行首
      None => text.truncate(line_start),
      Some(end_idx) => {
        text.drain(line_start..=actual_pos + end_idx);
      }
    }

    pos = line_start;
  }
}
