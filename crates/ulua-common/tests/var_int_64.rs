//! `try_read_var_int_64`（LEB128 变长整数）的截断/溢出契约测试。
//!
//! 依据：字节码 blob 是不可信输入（cpp `Bytecode.cpp` 的 readVarInt 读的是
//! 外部序列）。截断必须返回可恢复的 `None`，绝不能 panic 打挂宿主进程。

use ulua_common::functions::read_var_int_64::try_read_var_int_64;

fn varint(bytes: &[u8], offset: &mut usize) -> Option<u64> {
  try_read_var_int_64(bytes, offset)
}

#[test]
fn single_and_multi_byte_values() {
  let mut offset = 0;
  assert_eq!(varint(&[0x00], &mut offset), Some(0));
  assert_eq!(offset, 1);

  let mut offset = 0;
  assert_eq!(varint(&[0x7f], &mut offset), Some(0x7f));
  assert_eq!(offset, 1);

  // 0xFF 0x00 -> 低 7 位 0x7f，第二字节提供高位 0
  let mut offset = 0;
  assert_eq!(varint(&[0xff, 0x00], &mut offset), Some(0x7f));
  assert_eq!(offset, 2);
}

/// 截断（含「续字节一路读到 blob 末尾」）必须是可恢复的 `None`，
/// 不得 panic（否则损坏字节码会打挂宿主进程）。
#[test]
fn truncated_and_unterminated_streams_are_reported() {
  let mut offset = 0;
  assert_eq!(varint(&[], &mut offset), None);
  assert_eq!(offset, 0);

  // 末字节仍是续字节：下一次读落在 blob 外 -> 截断
  let mut offset = 2;
  assert_eq!(varint(&[0x80, 0x80, 0x80], &mut offset), None);
  assert_eq!(offset, 3);

  let mut offset = 0;
  assert_eq!(varint(&[0x80; 12], &mut offset), None);
  assert_eq!(offset, 12);
}

/// 续字节多到 `shift >= 64` 时高位丢弃：cpp 那里是 UB，这里保证低位仍
/// 按 LEB128 累加且一定能收尾
#[test]
fn oversized_stream_keeps_low_bits_and_terminates() {
  let mut bytes = [0xff; 12];
  bytes[11] = 0x00; // 第 12 字节才结束：第 11 字节的移位已达 70
  let mut offset = 0;

  assert_eq!(varint(&bytes, &mut offset), Some(u64::MAX));
  assert_eq!(offset, 12);
}
