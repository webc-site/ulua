use ulua_ast::records::{ast_node::AstNode, ast_type_pack::AstTypePack};

use crate::records::find_nth_occurence_of::FindNthOccurenceOf;

impl FindNthOccurenceOf {
  /// C++ `bool FindNthOccurenceOf::visit(AstTypePack* t) { return checkIt(t); }`
  /// (AstQueryDsl.cpp:40), matching the `AstNode`/`AstType` visit siblings.
  pub fn visit_ast_type_pack(&mut self, t: *mut AstTypePack) -> bool {
    self.check_it(t as *mut AstNode)
  }
}
