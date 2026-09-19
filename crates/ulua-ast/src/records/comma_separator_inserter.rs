use crate::records::position::Position;

/// C++ `CommaSeparatorInserter`：writer 引用在移植后不再持有（调用方逐次传入），
/// 逗号位置游标改为切片引用（cpp `begin()` 裸指针逐个 `add(1)` 推进的安全对应），
/// 可跨 `visualize_*` 调用持有。
pub struct CommaSeparatorInserter<'x> {
  pub(crate) first: bool,
  pub(crate) comma_positions: &'x [Position],
}

impl<'x> CommaSeparatorInserter<'x> {
  pub fn new(comma_positions: &'x [Position]) -> Self {
    Self {
      first: true,
      comma_positions,
    }
  }
}
