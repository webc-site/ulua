use crate::records::position::Position;

/// C++ `CommaSeparatorInserter`：writer 引用在移植后不再持有（调用方逐次传入），
/// 仅保留首个标志与 CST 逗号位置游标。
pub struct CommaSeparatorInserter {
  pub(crate) first: bool,
  pub(crate) comma_position: *const Position,
}

impl CommaSeparatorInserter {
  pub fn new(comma_position: *const Position) -> Self {
    Self {
      first: true,
      comma_position,
    }
  }
}
