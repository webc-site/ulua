//! `has_free_type` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::{has_free_type::HasFreeType, type_once_visitor::TypeOnceVisitor};

pub fn has_free_type_has_free_type() {
  let mut _visitor = HasFreeType {
    base: TypeOnceVisitor::new("TypeOnceVisitor".to_string(), true),
    result: false,
  };

  _visitor.has_free_type_has_free_type();
}

impl HasFreeType {
  pub fn visit_type_id_extern_type(&mut self) {
    self.result = false;
  }

  pub fn visit_type_id_free_type(&mut self) {
    self.result = true;
  }

  pub fn visit_type_pack_id_free_type_pack(&mut self) {
    self.result = true;
  }
}
