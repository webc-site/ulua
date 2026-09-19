use ulua_ast::{records::ast_node::AstNode, visit::ast_node_visit};

use crate::records::type_map_visitor::{TypeMapVisitor, TypeMapVisitorArgs};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn build_type_map(root: *mut AstNode, args: TypeMapVisitorArgs<'_>) {
  let mut visitor = TypeMapVisitor::new(args);

  unsafe {
    ast_node_visit(root, &mut visitor);
  }
}
