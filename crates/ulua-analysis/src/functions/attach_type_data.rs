use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_expr_local::AstExprLocal, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
  },
  visit::AstVisitable,
};

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{module::Module, source_module::SourceModule, type_attacher::TypeAttacher},
};
/// C++ `TypeAttacher : public AstVisitor`. The five overridden `visit`
/// overloads are landed as inherent methods on `TypeAttacher`; this impl
/// bridges them to the `AstVisitor` trait dispatch used by `root->visit(&ta)`.
impl AstVisitor for TypeAttacher {
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    self.visit_ast_stat_local(from_mut(node))
  }

  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    self.visit_ast_expr_local(from_mut(node))
  }

  fn visit_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    self.visit_ast_stat_for(from_mut(node))
  }

  fn visit_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    self.visit_ast_stat_for_in(from_mut(node))
  }

  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    // SAFETY: 满足 cpp 契约——node 指向 visit 遍历中的存活 AstExprFunction。
    unsafe { self.visit_ast_expr_function(from_mut(node)) }
  }
}

pub fn attach_type_data(source: &mut SourceModule, result: &mut Module) {
  // C++ `TypeAttacher ta(result, source.allocator.get()); source.root->visit(&ta);`
  let mut ta =
    TypeAttacher::type_attacher_type_attacher(result as *mut Module, arc_as_mut(&source.allocator));
  // Safety: root 是 parser 成功产物、恒非空（C++ `source.root->visit(&ta)` 同款裸解引用），
  // 指向 source.allocator arena 页中的存活节点（地址不移动）；ta 持有的 *mut Module 来自本
  // 函数独占的 &mut result、*mut Allocator 来自 arc_as_mut 的独占可变借用，遍历期间二者除 ta
  // 外无其他借用者，单线程串行下重建借用无别名冲突。
  unsafe {
    (*source.root).visit(&mut ta);
  }
}
