use ulua_ast::{records::ast_node::AstNode, visit::ast_node_visit};

use crate::records::find_nth_occurence_of::FindNthOccurenceOf;
impl FindNthOccurenceOf {
  pub(crate) fn visit_ast_node(&mut self, n: *mut AstNode) -> bool {
    // Safety: n 为查询入口存活 AST 节点指针（调用方自 fixture root 传入）；self 是遍历独占持有的访问器，ast_node_visit 按 cpp visit(AstVisitor*) 非 const 契约以 &mut 走全树，单线程串行无第二借用，遍历期间 visitor 不重入。
    unsafe {
      ast_node_visit(n, self);
    }
    !self.the_node.is_null()
  }
}
