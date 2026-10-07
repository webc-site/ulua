//! 缓冲区字节切片安全追加（取代旧 C 风格 strlen 扫描与裸指针拷贝）。

/// 安全追加字节切片到缓冲区；写入区间为 `[offset, offset + copy)`。
/// `copy` 经裁剪不超过 `buf.len().saturating_sub(1) - offset`，为尾部留至少一字节放终止符。
pub(crate) fn append_bytes(buf: &mut [u8], offset: usize, data: &[u8]) -> usize {
  let cap = buf.len().saturating_sub(1);
  if offset >= cap {
    return offset;
  }
  let copy = data.len().min(cap - offset);
  buf[offset..offset + copy].copy_from_slice(&data[..copy]);
  offset + copy
}
