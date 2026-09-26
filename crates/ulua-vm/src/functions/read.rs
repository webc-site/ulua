use core::mem::size_of;

use ulua_common::functions::read::BytecodeRead;

/// 从 `data[*offset..]` 读一个按内存布局（原生端序）解释的 `T` 并推进 `*offset`
/// （cpp `lvmload.cpp:125` 的 `read<T>` 就是一次 `memcpy`）。
///
/// 与 cpp 的差异：cpp 侧只有 `LUAU_ASSERT(size >= offset + sizeof(T))`，release
/// 编译掉就是越界读。字节码是宿主传进来的**不可信输入**，故这里既不用 `assert!`
/// /panic 收口（那会把本该返回 `LUA_ERRSYNTAX` 的 `luau_load` 升级成打挂宿主进程），
/// 也不放任越界读：剩余字节不足（含 `offset + size_of::<T>()` 自身回绕——回绕后会
/// 伪装成合法边界）一律返回 `None`，由 `loadsafe` 转成「损坏字节码」的 Lua 错误。
///
/// 失败时 `*offset` 保持不变，调用方可据此报告出错位置。以 `&[u8]` 为入参，
/// 边界检查交给切片，全程无 `unsafe`。
pub(crate) fn read<T: BytecodeRead>(data: &[u8], offset: &mut usize) -> Option<T> {
  let end = offset.checked_add(size_of::<T>())?;
  let value = T::from_bytes(data.get(*offset..end)?);
  *offset = end;

  Some(value)
}

// 留证：`read<T>` 对应 cpp lvmload.cpp:125 的文件内 `template read<T>`（非导出面）；
// 「失败不推进 *offset」与 `offset + size_of::<T>()` 回绕两条契约经公开 `luau_load`
// 不可达（offset 恒 ≤ blob 长度，tests/load_malformed.rs 只能覆盖尾部截断通道），
// 且迁出需把内部读取原语提升为 pub。
#[cfg(test)]
mod tests {
  use core::fmt;

  use super::*;

  /// 以 `bytes` 为 blob 从 `*offset` 读一个 `T`
  fn read_at<T: BytecodeRead + PartialEq + fmt::Debug>(
    bytes: &[u8],
    offset: &mut usize,
  ) -> Option<T> {
    read(bytes, offset)
  }

  #[test]
  fn fixed_width_reads_follow_native_layout() {
    let bytes = [0x11, 0x22, 0x33, 0x44];
    let mut offset = 0;

    assert_eq!(
      read_at::<u32>(&bytes, &mut offset),
      Some(u32::from_ne_bytes(bytes))
    );
    assert_eq!(offset, 4);
  }

  #[test]
  fn insufficient_tail_is_reported_not_panicked() {
    let bytes = [0x11, 0x22, 0x33, 0x44];
    let mut offset = 2;

    assert_eq!(read_at::<u32>(&bytes, &mut offset), None);
    // 失败不得推进 offset：调用方的错误消息要用它定位
    assert_eq!(offset, 2);
  }

  #[test]
  fn empty_buffer_is_reported() {
    let mut offset = 0;

    assert_eq!(read_at::<u8>(&[], &mut offset), None);
    assert_eq!(offset, 0);
  }

  /// `offset + size_of::<T>()` 回绕不得伪装成合法边界（cpp 的 `size_t` 同样会绕，
  /// 但绕过去就是野指针读）
  #[test]
  fn offset_overflow_is_reported() {
    let bytes = [0u8; 4];
    let mut offset = usize::MAX;

    assert_eq!(read_at::<u32>(&bytes, &mut offset), None);
    assert_eq!(offset, usize::MAX);
  }
}
