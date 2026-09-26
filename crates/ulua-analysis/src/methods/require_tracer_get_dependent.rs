use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_group::AstExprGroup,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode,
    ast_type_group::AstTypeGroup, ast_type_typeof::AstTypeTypeof,
  },
  rtti::{AstNodePtr, ast_node_try_as},
};

use crate::records::require_tracer::RequireTracer;
impl RequireTracer<'_> {
  /// # Safety
  /// `node` 须指向本次 require 追踪期间存活的 parse-arena `AstNode`：非空、对齐，地址在 arena
  /// 释放前不移动；函数全程只读，单线程串行。对应 C++ `AstNode* getDependent(AstNode* node)`
  /// (`cpp/Analysis/src/RequireTracer.cpp:74`)。
  pub unsafe fn get_dependent(&self, node: *mut AstNode) -> *mut AstNode {
    // SAFETY: node 由调用方契约保证非空且指向存活节点；本函数全程只读，
    // 一次裸解引用换得 &AstNode，后续各分支下转与字段读取全部走安全引用。
    let node_ref: &AstNode = unsafe { &*node };

    if let Some(expr) = ast_node_try_as::<AstExprLocal>(node_ref) {
      // local 槽已句柄化恒非空；locals 键值为既有裸指针形态，经 as_ptr 桥接。
      if let Some(&val) = self.locals.find(&(expr.local.as_ptr())) {
        return val.as_ast_node();
      }
      return null_mut();
    } else if let Some(expr) = ast_node_try_as::<AstExprIndexName>(node_ref) {
      return expr.expr.as_ast_node();
    } else if let Some(expr) = ast_node_try_as::<AstExprIndexExpr>(node_ref) {
      return expr.expr.as_ast_node();
    } else if let Some(expr) = ast_node_try_as::<AstExprCall>(node_ref) {
      if expr.self_ {
        // 对照 C++ `static_cast<AstExprIndexName*>(func)`：self-call 不变式保证
        // func 实为 AstExprIndexName，无 is<> 判定。SAFETY: 同上，arena 存活。
        let func: &AstExprIndexName = unsafe { &*expr.func.cast::<AstExprIndexName>() };
        return func.expr.as_ast_node();
      }
    } else if let Some(expr) = ast_node_try_as::<AstExprGroup>(node_ref) {
      return expr.expr.as_ast_node();
    } else if let Some(expr) = ast_node_try_as::<AstExprTypeAssertion>(node_ref) {
      return expr.annotation.as_ast_node();
    } else if let Some(expr) = ast_node_try_as::<AstTypeGroup>(node_ref) {
      return expr.type_.as_ast_node();
    } else if let Some(expr) = ast_node_try_as::<AstTypeTypeof>(node_ref) {
      return expr.expr.as_ast_node();
    }

    null_mut()
  }
}
