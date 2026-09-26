//! 不可信字节码 `readString` 的三通道契约（cpp/VM/src/lvmload.cpp:169-173）。
//!
//! cpp 侧：`id == 0 ? NULL : strings[id - 1]`，越界只靠
//! `TempBuffer::operator[]` 的 `LUAU_ASSERT(index < count)`（lvmload.cpp:57-60，
//! release 编译掉即越界读）；Rust 侧必须硬校验，逐结局钉
//! 「命中 / 越界 OutOfRange / 截断 Truncated」三分，且不得与
//! `loadsafe` 的错误消息通道混淆（公开面的错误文案在 `tests/load_malformed.rs`）。
//!
//! id 的取值来自 `readVarInt`（lvmload.cpp:135-149，32 位累加并丢弃续位溢出），
//! 与 `readVarInt64` 截断 `as u32` 在低位一致：宽 varint 按低 32 位命中。

use core::{mem::ManuallyDrop, ptr::null_mut};
use std::result::Result;

use ulua_vm::{
  functions::read_string::{ReadStringError, read_string},
  records::{t_string::tstring, temp_buffer::TempBuffer},
};

/// 非解引用哨兵：槽内容对 `read_string` 透明，仅用于钉 `strings[id - 1]` 的
/// 索引映射（cpp lvmload.cpp:173），绝不在测试中被解引用。
fn slot(n: usize) -> *mut tstring {
  n as *mut tstring
}

/// 用 `count` 项、哨兵填充的字符串表驱动 `read_string`，返回 (结局, 读毕 offset)。
/// offset 一并钉住：cpp `readString` 以 `size_t& offset` 按引用推进 varint 字节数。
fn read_with_table(
  table: &[*mut tstring],
  bytes: &[u8],
) -> (Result<*mut tstring, ReadStringError>, usize) {
  // 借用栈数组的只读窗口：`read_string` 只经 `Index`（methods/
  // temp_buffer_operator_index.rs）读取，不写穿该指针。`ManuallyDrop` 阻止
  // `Drop`（methods/temp_buffer_temp_buffer_lvmload.rs）把栈指针交给 `lua_m_free`
  let strings = ManuallyDrop::new(TempBuffer {
    l: null_mut(),
    data: table.as_ptr().cast_mut(),
    count: table.len(),
  });
  let mut offset = 0;
  let result = read_string(&strings, bytes, &mut offset);
  (result, offset)
}

fn read(bytes: &[u8]) -> (Result<*mut tstring, ReadStringError>, usize) {
  read_with_table(&[slot(0xA0), slot(0xB0)], bytes)
}

#[test]
fn zero_id_reads_as_no_string() {
  // cpp lvmload.cpp:173 `id == 0 ? NULL`
  assert_eq!(read(&[0x00]), (Ok(null_mut()), 1));
}

/// varint 的多字节零编码（0x80 0x00 == 0）同样走 NULL 通道：cpp readVarInt
/// （lvmload.cpp:135-149）逐 7 位累加，不识别「最短编码」。
#[test]
fn multibyte_zero_id_still_reads_as_no_string() {
  assert_eq!(read(&[0x80, 0x00]), (Ok(null_mut()), 2));
}

/// 表内 id 必须命中且偏移是 `id - 1`（若写成 `id`，id=1 会拿到 slot 2、
/// id=2 会被误判越界；哨兵指针把这条映射钉死）。
#[test]
fn in_range_id_hits_the_table_at_id_minus_one() {
  assert_eq!(read(&[0x01]), (Ok(slot(0xA0)), 1));
  assert_eq!(read(&[0x02]), (Ok(slot(0xB0)), 1));
  // 双字节 varint 编码的 id=1（0x81 0x00）同样命中 slot 1，offset 吃掉两个字节
  assert_eq!(read(&[0x81, 0x00]), (Ok(slot(0xA0)), 2));
}

/// 越界只报 `OutOfRange`：不得升级成 panic（cpp release 是静默越界读，
/// lvmload.cpp:57-60 的 LUAU_ASSERT 编译掉），也不得与截断混淆。
#[test]
fn out_of_range_id_is_not_truncation() {
  assert_eq!(read(&[0x03]), (Err(ReadStringError::OutOfRange), 1));
  // id = u32::MAX：0xFF×4 + 0x0F 的 5 字节 varint
  assert_eq!(
    read(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]),
    (Err(ReadStringError::OutOfRange), 5)
  );
  // count=0 的表：任何非零 id 都是越界，不是截断
  assert_eq!(
    read_with_table(&[], &[0x01]),
    (Err(ReadStringError::OutOfRange), 1)
  );
}

/// 宽 varint 按 32 位截断（cpp `readVarInt` 的 `unsigned int` 累加器，
/// lvmload.cpp:136-145；Rust 侧 `readVarInt64 as u32`，read_var_int.rs 头注）：
/// 2^32 + 1 截断为 1 → 命中 slot 1，而非越界。
#[test]
fn wide_varint_truncates_to_u32_like_cpp() {
  // 0x81 0x80 0x80 0x80 0x10 == 1 | (0x10 << 28) == 2^32 + 1
  assert_eq!(read(&[0x81, 0x80, 0x80, 0x80, 0x10]), (Ok(slot(0xA0)), 5));
}

/// 截断只报 `Truncated`：不得升级成 panic，也不得与「id 越界」混淆。
/// offset 停在最后一个成功读出的字节之后、不回退（read_var_int.rs 头注契约）。
#[test]
fn truncated_id_is_reported() {
  assert_eq!(read(&[]), (Err(ReadStringError::Truncated), 0));
  // 续字节一路到 blob 末尾
  assert_eq!(read(&[0x80; 4]), (Err(ReadStringError::Truncated), 4));
  // 命中前截断：第一字节完好、第二续字节缺失
  assert_eq!(read(&[0x81]), (Err(ReadStringError::Truncated), 1));
}
