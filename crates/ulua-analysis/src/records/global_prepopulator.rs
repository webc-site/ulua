use core::ptr::NonNull;

use ulua_ast::records::{
  ast_expr_global::AstExprGlobal, ast_name::AstName, ast_stat_assign::AstStatAssign,
  ast_stat_function::AstStatFunction, ast_type::AstType, ast_type_pack::AstTypePack,
  ast_visitor::AstVisitor,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{data_flow_graph::DataFlowGraph, scope::Scope, type_arena::TypeArena};
#[derive(Debug, Clone)]
pub struct GlobalPrepopulator {
  pub global_scope: NonNull<Scope>,
  pub arena: NonNull<TypeArena>,
  pub dfg: NonNull<DataFlowGraph>,
  pub uninitialized_globals: DenseHashSet<AstName>,
}

impl GlobalPrepopulator {
  /// §2 裸指针收口单点：三个 NonNull 字段由 prepopulate 装配点以存活句柄经
  /// `NonNull::from` 注入（prepopulate 期指向对象恒存活、单线程串行独占写
  /// scope/arena），解引用只发生在本模块这三个访问器内。
  pub(crate) fn scope_mut(&mut self) -> &'static mut Scope {
    // SAFETY: 构造契约见上——`global_scope` 指向本次预填充期间存活的 Scope，
    // 本 pass 为其唯一写者；拷出 NonNull 后解引用（借用不锚定 &mut self，
    // 与原裸指针语义同构）。
    let mut p = self.global_scope;
    unsafe { p.as_mut() }
  }

  pub(crate) fn arena_mut(&mut self) -> &'static mut TypeArena {
    // SAFETY: 同上——`arena` 为会话存活的类型 arena，add_type 独占追加。
    let mut p = self.arena;
    unsafe { p.as_mut() }
  }

  pub(crate) fn dfg(&self) -> &'static DataFlowGraph {
    // SAFETY: 同上——`dfg` 为构造期注入的只读 DataFlowGraph 句柄。
    let p = self.dfg;
    unsafe { p.as_ref() }
  }
}

impl AstVisitor for GlobalPrepopulator {
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_ast_expr_global(node)
  }

  fn visit_type(&mut self, _node: &mut AstType) -> bool {
    true
  }

  fn visit_type_pack(&mut self, _node: &mut AstTypePack) -> bool {
    true
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.visit_ast_stat_assign(node)
  }
  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_ast_stat_function(node)
  }
}
