use core::mem::size_of;

use ulua_common::functions::{read::BytecodeRead, read_var_int_64::try_read_var_int_64};

/// 只读字节码游标。
///
/// [`ulua_common::functions::read::read`] 在越界时 panic（cpp 的 `read<T>` 只有
/// `LUAU_ASSERT`，release 下直接越界读，移植时收紧成 panic 以避免 UB）。反序列化
/// 入口的输入是不可信文件，契约是 `Option`，因此这里把同一套读法改成可失败版本：
/// 越界一律返回 `None`，由调用方作为"字节码损坏"处理。
#[derive(Debug, Clone, Copy)]
pub(crate) struct Cursor<'a> {
  data: &'a [u8],
  offset: usize,
}

impl<'a> Cursor<'a> {
  pub(crate) const fn new(data: &'a [u8]) -> Self {
    Self { data, offset: 0 }
  }

  /// 读一个按内存布局解释的 `T`；剩余字节不足时返回 `None`。
  pub(crate) fn read<T: BytecodeRead>(&mut self) -> Option<T> {
    let end = self.offset.checked_add(size_of::<T>())?;
    let bytes = self.data.get(self.offset..end)?;
    self.offset = end;
    Some(T::from_bytes(bytes))
  }

  pub(crate) fn var_int(&mut self) -> Option<u32> {
    Some(self.var_int_64()? as u32)
  }

  /// 与 cpp `readVarInt64` 一致：续字节过多时高位丢弃而非移位溢出。
  /// 逐字节读法与越界/截断语义委托 `ulua-common` 的唯一 LEB128 实现，
  /// 失败时 `offset` 停在最后成功读出的字节之后（不回退），由调用方判损坏。
  pub(crate) fn var_int_64(&mut self) -> Option<u64> {
    try_read_var_int_64(self.data, &mut self.offset)
  }

  /// 取走 `len` 个原始字节；越界返回 `None`。
  pub(crate) fn bytes(&mut self, len: usize) -> Option<&'a [u8]> {
    let end = self.offset.checked_add(len)?;
    let slice = self.data.get(self.offset..end)?;
    self.offset = end;
    Some(slice)
  }

  /// 由输入声明的条目数是否可信：`count` 项、每项至少 `min_item_size` 字节，
  /// 必须仍在剩余字节之内，否则损坏的计数会让调用方一次性申请天文数字的内存。
  pub(crate) fn has_room_for(&self, count: usize, min_item_size: usize) -> bool {
    count
      .checked_mul(min_item_size)
      .is_some_and(|need| need <= self.data.len() - self.offset)
  }
}
