use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::records::global_name_collector::GlobalNameCollector;

impl GlobalNameCollector {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit(&mut self, node: *mut AstExprGlobal) -> bool {
    let node_ref = unsafe { &*node };
    self.names.insert(node_ref.name);
    true
  }
}
