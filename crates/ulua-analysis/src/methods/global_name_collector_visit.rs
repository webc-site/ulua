use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::records::global_name_collector::GlobalNameCollector;

impl GlobalNameCollector {
  /// # Safety
  /// `{node}` 须指向本次遍历期间存活的 parse-arena 节点：非空、对齐，地址在该 arena 释放前不
  /// 移动；调用方（AstVisitor 遍历驱动）单线程串行访问，函数体内不产生与之重叠的可变借用。
  /// 对应 C++ `bool GlobalNameCollector::visit(AstExprGlobal* node)` (`cpp/Analysis/src/ConstraintGenerator.cpp:262`)。
  pub unsafe fn visit(&mut self, node: *mut AstExprGlobal) -> bool {
    let node_ref = unsafe { &*node };
    self.names.insert(node_ref.name);
    true
  }
}
