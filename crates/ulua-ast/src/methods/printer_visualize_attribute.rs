use crate::records::{ast_attr::AstAttr, printer::Printer};

impl<'a> Printer<'a> {
  pub fn visualize_attribute(&mut self, attribute: &mut AstAttr) {
    self.advance(attribute.base.location.begin);
    self.writer.symbol("@");
    self.writer.identifier(attribute.name.as_str_or_empty());
  }
}
