use crate::records::{position::Position, writer::Writer};

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

  /// 写入下一个元素前的逗号分隔符；首个元素跳过，其后在 CST 给出的逗号
  /// 位置写入 `,`。切片逐个消耗（与 cpp 游标 `add(1)` 推进同序）；耗尽时
  /// 直接写——解析器保证逗号数与元素数一致，cpp 裸指针同样依赖该不变式。
  pub fn write<W: Writer>(&mut self, writer: &mut W) {
    if self.first {
      self.first = false;
    } else {
      if let Some((pos, rest)) = self.comma_positions.split_first() {
        writer.advance(pos);
        self.comma_positions = rest;
      }
      writer.symbol(",");
    }
  }
}
