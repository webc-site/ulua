use ulua_ast::records::ast_type_function::AstTypeFunction;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `f` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_ast_type_function(&mut self, f: *mut AstTypeFunction) {
    unsafe {
      let f_ref = &*f;

      self.visit_generics(f_ref.generics);
      self.visit_generic_packs(f_ref.generic_packs);
      self.visit_type_list(f_ref.arg_types);
      self.visit_type_pack_ast_type_pack(f_ref.return_types);
    }
  }
}
