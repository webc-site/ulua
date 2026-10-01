//! buffer_readbits/buffer_writebits 共享的"字节区间装入 u64"辅助函数。

/// 将 `bytes`（已按界校验的 `buf[startbyte..endbyte]` 区间，至多 8 字节）装入 u64。
///
/// cpp（lstrlib.cpp 位操作装载）分两条路径：大端逐字节高位先入、小端整块 memcpy；
/// 两条路径对同一区间产出的整数值完全相同——低位字节恒落在 `startbyte`——即
/// `from_le_bytes` 零扩展，故此处以单一安全实现收敛，无端序分支。
///
/// 长度上限由调用方契约保证：`bitcount <= 32` 且 `subbyteoffset <= 7`，
/// 区间不超 5 字节；违例即调用方违约，panic 兜底不留静默越界。
pub(crate) fn load_bits_u64(bytes: &[u8]) -> u64 {
  let mut buf = [0u8; 8];
  buf[..bytes.len()].copy_from_slice(bytes);
  u64::from_le_bytes(buf)
}
