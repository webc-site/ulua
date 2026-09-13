use crate::records::{ast_local::AstLocal, position::Position, printer::Printer};

impl<'a> Printer<'a> {
  pub fn visualize_ast_local_position(&mut self, local: &AstLocal, colon_position: Position) {
    self.advance(local.location.begin);

    self.writer.identifier(local.name.as_str_or_empty());
    if self.write_types && !local.annotation.is_null() {
      self.maybe_advance_and_write(&colon_position, ":", true);
      unsafe {
        self.visualize_type_annotation(&mut *local.annotation);
      }
    }
  }
}
