use crate::{
  records::{ast_array::AstArray, position::Position, writer::Writer},
  type_aliases::ast_argument_name::AstArgumentName,
};

/// `W: Writer` 泛型静态分发（实现者仅本 crate 内的 StringWriter）。
pub struct ArgNameInserter<'a, W: Writer> {
  pub(crate) writer: &'a mut W,
  pub(crate) names: AstArray<Option<AstArgumentName>>,
  pub(crate) colon_positions: AstArray<Position>,
  pub(crate) idx: usize,
}

impl<'a, W: Writer> ArgNameInserter<'a, W> {
  pub(crate) fn new(
    writer: &'a mut W,
    names: AstArray<Option<AstArgumentName>>,
    colon_positions: AstArray<Position>,
  ) -> Self {
    Self {
      writer,
      names,
      colon_positions,
      idx: 0,
    }
  }
}
