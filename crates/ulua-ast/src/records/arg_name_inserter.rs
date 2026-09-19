use crate::{
  records::{position::Position, writer::Writer},
  type_aliases::ast_argument_name::AstArgumentName,
};

/// `W: Writer` 泛型静态分发（实现者仅本 crate 内的 StringWriter）；names 与
/// colon_positions 为切片引用（cpp `AstArray` 裸指针的安全对应）。
pub struct ArgNameInserter<'w, 'n, W: Writer> {
  pub(crate) writer: &'w mut W,
  pub(crate) names: &'n [Option<AstArgumentName>],
  pub(crate) colon_positions: &'n [Position],
  pub(crate) idx: usize,
}

impl<'w, 'n, W: Writer> ArgNameInserter<'w, 'n, W> {
  pub(crate) fn new(
    writer: &'w mut W,
    names: &'n [Option<AstArgumentName>],
    colon_positions: &'n [Position],
  ) -> Self {
    Self {
      writer,
      names,
      colon_positions,
      idx: 0,
    }
  }
}
