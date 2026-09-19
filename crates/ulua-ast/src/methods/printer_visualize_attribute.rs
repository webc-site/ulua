use crate::records::{ast_attr::AstAttr, printer::Printer, writer::Writer};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_attribute(&mut self, attribute: &mut AstAttr) {
    self.advance(attribute.base.location.begin);
    self.writer.symbol("@");
    self.writer.identifier(attribute.name.as_bytes());
  }
}
