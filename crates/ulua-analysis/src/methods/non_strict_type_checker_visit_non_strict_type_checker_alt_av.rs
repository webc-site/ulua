use ulua_ast::records::ast_type_table::AstTypeTable;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `table` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_table(&mut self, table: *mut AstTypeTable) {
    unsafe {
      if !(*table).indexer.is_null() {
        let indexer = (*table).indexer;
        self.visit_ast_type((*indexer).index_type);
        self.visit_ast_type((*indexer).result_type);
      }

      let props = &(*table).props;
      for prop in props.as_slice() {
        self.visit_ast_type(prop.r#type);
      }
    }
  }
}
