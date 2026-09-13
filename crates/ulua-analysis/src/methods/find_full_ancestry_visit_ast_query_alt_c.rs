use ulua_ast::records::ast_node::AstNode;

use crate::records::find_full_ancestry::FindFullAncestry;

impl FindFullAncestry {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_node(&mut self, node: *mut AstNode) -> bool {
    let node_ref = unsafe { &*node };

    if node_ref.location.contains(self.pos) {
      self.nodes.push(node);
      return true;
    }

    // Edge case: If we ask for the node at the position that is the very end of the document
    // return the innermost AST element that ends at that position.

    if node_ref.location.end == self.document_end && self.pos >= self.document_end {
      self.nodes.push(node);
      return true;
    }

    false
  }
}
