use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{location::Location, printer::Printer, writer::Writer},
};

impl<'a, W: Writer> Printer<'a, W> {
  /// 表访问关键字（read/write）：CST 有 access_location 时校准前进并写
  /// （cpp `visualizeTypeTable` 内三处同款合并）。
  pub(crate) fn write_table_access(
    &mut self,
    access: AstTableAccess,
    access_location: Option<Location>,
  ) {
    if let Some(loc) = access_location {
      self.advance(loc.begin);
      self.writer.keyword(if access == AstTableAccess::Read {
        "read"
      } else {
        "write"
      });
    }
  }
}
