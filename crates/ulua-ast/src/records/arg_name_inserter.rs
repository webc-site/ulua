use crate::{
  records::{position::Position, writer::Writer},
  type_aliases::ast_argument_name::AstArgumentName,
};

/// names 与 colon_positions 为切片引用（cpp `AstArray` 裸指针的安全对应）。
/// writer 不再长期借用持有，改在 `write` 调用时传入，避免生命周期冲突。
pub struct ArgNameInserter<'n> {
  pub(crate) names: &'n [Option<AstArgumentName>],
  pub(crate) colon_positions: &'n [Position],
  pub(crate) idx: usize,
}

impl<'n> ArgNameInserter<'n> {
  pub(crate) fn new(names: &'n [Option<AstArgumentName>], colon_positions: &'n [Position]) -> Self {
    Self {
      names,
      colon_positions,
      idx: 0,
    }
  }

  /// 写入当前参数名及其后的可选冒号，并推进游标。
  pub fn write<W: Writer>(&mut self, writer: &mut W) {
    if let Some(name_val) = self.names.get(self.idx).copied().flatten() {
      writer.advance(&name_val.1.begin);
      writer.identifier(name_val.0.as_bytes());

      if let Some(colon) = self.colon_positions.get(self.idx) {
        writer.advance(colon);
      }

      writer.symbol(":");
    }
    self.idx += 1;
  }
}
