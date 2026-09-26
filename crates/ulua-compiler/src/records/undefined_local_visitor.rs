//! Source: `Compiler/src/Compiler.cpp`

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_expr_local::AstExprLocal, ast_local::AstLocal,
  ast_visitor::AstVisitor,
};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::records::{compiler::Compiler, node::Node};

/// cpp `UndefinedLocalVisitor`：检测 continue 跳过的 local 随后被使用。
/// `compiler` 借用为只读（查询 `functions`），取代裸指针自引用。
#[derive(Debug)]
pub(crate) struct UndefinedLocalVisitor<'a> {
  pub(crate) compiler: &'a Compiler,
  /// 首个「被 continue 跳过且随后使用」的 local（cpp null 哨兵 → Option）
  pub(crate) undef: Option<Node<AstLocal>>,
  pub(crate) locals: DenseHashSet<Node<AstLocal>>,
}

impl UndefinedLocalVisitor<'_> {
  pub(crate) fn check(&mut self, local: Node<AstLocal>) {
    if self.undef.is_none() && self.locals.contains(&local) {
      self.undef = Some(local);
    }
  }
}

impl AstVisitor for UndefinedLocalVisitor<'_> {
  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    if !node.upvalue {
      self.check(node.local.into());
    }
    false
  }

  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    let f = self.compiler.functions.find(&Node::from_mut(node));
    LUAU_ASSERT!(f.is_some());
    let f = f.unwrap();

    // uv 句柄的存活契约见 `Node::borrow`（Function 建档的 arena local，编译期只读）。
    for uv in &f.upvals {
      LUAU_ASSERT!(uv.borrow().function_depth < node.function_depth);

      if uv.borrow().function_depth == node.function_depth - 1 {
        self.check(*uv);
      }
    }
    false
  }
}
