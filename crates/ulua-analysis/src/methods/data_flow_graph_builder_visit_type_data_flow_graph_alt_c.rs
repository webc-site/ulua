use ulua_ast::records::ast_type_table::AstTypeTable;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_ast_type_table(&mut self, t: *mut AstTypeTable) {
    unsafe {
      let table = &*t;
      for prop in table.props.as_slice() {
        self.visit_type_ast_type(prop.r#type);
      }

      if !table.indexer.is_null() {
        let indexer = &*table.indexer;
        self.visit_type_ast_type(indexer.index_type);
        self.visit_type_ast_type(indexer.result_type);
      }
    }
  }
}
