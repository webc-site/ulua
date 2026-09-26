//! Source: `Common/src/BytecodeWire.cpp`（`readVarInt64`，`BytecodeWire.cpp:9`）

/// LEB128 varint64 的唯一实现：从 `data[*offset..]` 逐字节累加，直到某字节的
/// 续位（`& 0x80`）为 0。
///
/// 与 cpp 的差异（两处都是收紧而非放宽）：
/// - 上游 `read<uint8_t>` 只有 `LUAU_ASSERT` 边界（release 编译掉就是越界读），
///   前提是调用方保证流合法；这里任何一次字节读失败都立刻返回 `None`，供
///   不可信输入路径（如 VM 的 `luau_load`）转成「损坏字节码」错误。
/// - 上游对 `unsigned int shift` 无守卫，损坏流的续字节让 `shift ≥ 32/64` 后
///   是移位 UB；这里 `shift ≥ 64` 时丢弃高位（u64 的 varint 最多 10 字节）。
///
/// 截断流时 `*offset` 停在最后一个成功读出的字节之后、不回退：上游对截断
/// 没有定义行为（断言即死），故调用方一律以返回 `None` 为准判定，不得依赖
/// offset 回退。
pub fn try_read_var_int_64(data: &[u8], offset: &mut usize) -> Option<u64> {
  let mut result: u64 = 0;
  let mut shift: u32 = 0;

  loop {
    // `get` 失败（含 offset 已达/越过末尾、offset 自身回绕后仍越界）不推进 offset
    let byte = *data.get(*offset)?;
    // 上一行保证 `*offset < data.len() ≤ isize::MAX`，+1 不会溢出
    *offset += 1;

    if shift < 64 {
      result |= u64::from(byte & 0x7f) << shift;
    }
    shift += 7;

    if byte & 0x80 == 0 {
      return Some(result);
    }
  }
}

/// [`try_read_var_int_64`] 的不可失败版本：越界即 panic（对应 C++
/// `BytecodeWire.cpp:9` 里 `read<uint8_t>` 的 `LUAU_ASSERT`，供已预先核对长度
/// 的可信流使用，如 `read_var_int` 的委托路径）。
pub fn read_var_int_64(data: &[u8], offset: &mut usize) -> u64 {
  try_read_var_int_64(data, offset).expect("read out of bounds")
}
