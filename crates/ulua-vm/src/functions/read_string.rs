use core::ptr::null_mut;

use crate::{
  functions::read_var_int::read_var_int,
  records::{t_string::tstring, temp_buffer::TempBuffer},
};

/// `readString` 读到的 id 来自不可信字节码，cpp 侧两种坏情况的成因不同
/// （`lvmload.cpp:169` 的 `strings[id - 1]` 只有 `TempBuffer::operator[]` 里的
/// `LUAU_ASSERT`，release 编译掉就是越界读；id 本身读不完则是 blob 截断），
/// `loadsafe` 也要据此报不同的错误消息，故分开建模。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadStringError {
  /// id 超出已装载的字符串表
  OutOfRange,
  /// blob 已截断，连 id 都没读完
  Truncated,
}

/// cpp `lvmload.cpp:169-173` `readString` 对应：`id == 0 ? NULL : strings[id - 1]`。
///
/// 成功时返回的裸指针可以为 `null`，语义与 cpp 一致：id 0 表示「无字符串」，
/// 由调用方按各自字段决定 nullptr 是否合法（debugname/upvalue 名允许，
/// 字符串常量走 `setsvalue` 同理）。
///
/// 与 cpp 的差异：cpp 只靠 `LUAU_ASSERT` 兜底（release 直接编译掉，损坏字节码
/// 就是越界读），Rust 侧必须硬校验，故失败返回 [`ReadStringError`] 交由
/// `loadsafe` 转成「损坏字节码」错误，不 panic。以 `&[u8]` 为入参，本身无 `unsafe`。
pub fn read_string(
  strings: &TempBuffer<*mut tstring>,
  data: &[u8],
  offset: &mut usize,
) -> Result<*mut tstring, ReadStringError> {
  let Some(id) = read_var_int(data, offset) else {
    return Err(ReadStringError::Truncated);
  };

  if id == 0 {
    return Ok(null_mut());
  }

  let index = (id - 1) as usize;

  if index >= strings.count {
    return Err(ReadStringError::OutOfRange);
  }

  // 上面已硬校验过边界，`Index` 内的 LUAU_ASSERT 恒真
  Ok(strings[index])
}
