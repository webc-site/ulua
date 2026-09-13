use crate::{
  records::{
    arg_name_inserter::ArgNameInserter, ast_array::AstArray, position::Position, writer::Writer,
  },
  type_aliases::ast_argument_name::AstArgumentName,
};

impl<'a> ArgNameInserter<'a> {
  pub fn arg_name_inserter_arg_name_inserter(
    writer: &'a mut dyn Writer,
    names: AstArray<Option<AstArgumentName>>,
    colon_positions: AstArray<Position>,
  ) -> Self {
    ArgNameInserter::new(writer, names, colon_positions)
  }
}
