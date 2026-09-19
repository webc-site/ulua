use crate::records::{ast_attr::AstAttr, printer::Printer, writer::Writer};

impl<'a, W: Writer> Printer<'a, W> {
  /// cpp `Printer::visit(AstAttr*)` 的打印：节点只读，写入只发生在 `Writer`。
  pub fn visualize_attribute(&mut self, attribute: &AstAttr) {
    self.advance(attribute.base.location.begin);
    self.writer.symbol("@");
    self.writer.identifier(attribute.name.as_bytes());
  }
}
