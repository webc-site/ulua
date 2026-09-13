pub struct ArgNameInserter<'a> {
  pub(crate) writer: &'a mut dyn Writer,
  pub(crate) names: AstArray<Option<AstArgumentName>>,
  pub(crate) colon_positions: AstArray<Position>,
  pub(crate) idx: usize,
}

impl<'a> ArgNameInserter<'a> {
  pub(crate) fn new(
    writer: &'a mut dyn Writer,
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
use crate::{
  records::{ast_array::AstArray, position::Position, writer::Writer},
  type_aliases::ast_argument_name::AstArgumentName,
};
