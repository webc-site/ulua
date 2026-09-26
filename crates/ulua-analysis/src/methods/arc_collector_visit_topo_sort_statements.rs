use ulua_ast::records::{
  ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
  ast_expr_local::AstExprLocal, ast_stat_function::AstStatFunction,
  ast_stat_local_function::AstStatLocalFunction, ast_stat_type_alias::AstStatTypeAlias,
  ast_type_reference::AstTypeReference, ast_type_typeof::AstTypeTypeof,
};

use crate::{
  functions::mk_name_topo_sort_statements::{
    mk_name_ast_expr, mk_name_ast_expr_global, mk_name_ast_expr_index_name, mk_name_ast_expr_local,
    mk_name_ast_name, mk_name_ast_stat_function, mk_name_ast_stat_local_function,
    mk_name_ast_stat_type_alias,
  },
  records::{arc_collector::ArcCollector, identifier::Identifier},
};

impl ArcCollector<'_> {
  pub fn visit_ast_expr_global(&mut self, node: &AstExprGlobal) -> bool {
    let name: Identifier = mk_name_ast_expr_global(node);
    self.add(&name);
    true
  }

  pub fn visit_ast_expr_local(&mut self, node: &AstExprLocal) -> bool {
    let name = mk_name_ast_expr_local(node);
    self.add(&name);
    true
  }

  pub fn visit_ast_expr_index_name(&mut self, node: &AstExprIndexName) -> bool {
    if let Some(name) = mk_name_ast_expr_index_name(node) {
      self.add(&name);
    }
    true
  }

  pub fn visit_ast_stat_function(&mut self, node: &AstStatFunction) -> bool {
    let name = mk_name_ast_stat_function(node);
    self.add(&name);
    true
  }

  pub fn visit_ast_stat_local_function(&mut self, node: &AstStatLocalFunction) -> bool {
    let name = mk_name_ast_stat_local_function(node);
    self.add(&name);
    true
  }

  pub fn visit_ast_stat_type_alias(&mut self, node: &AstStatTypeAlias) -> bool {
    let name = mk_name_ast_stat_type_alias(node);
    self.add(&name);
    true
  }

  /// cpp `visit(AstType*) { return true; }`：trait 默认 `false` 会跳过注解子树，
  /// 此覆写仅用于打开遍历，不读取节点本身。
  pub fn visit_ast_type(&mut self) -> bool {
    true
  }

  pub fn visit_ast_type_reference(&mut self, node: &AstTypeReference) -> bool {
    let name = node.name;
    let identifier = mk_name_ast_name(&name);
    self.add(&identifier);
    true
  }

  pub fn visit_ast_type_typeof(&mut self, node: &AstTypeTypeof) -> bool {
    let expr = node.expr;
    let name = unsafe { mk_name_ast_expr(&*expr) };
    if let Some(name) = name {
      self.add(&name);
    }
    true
  }

  /// cpp `visit(AstTypePack*) { return true; }`：同 `visit_ast_type`，
  /// 仅覆盖 trait 默认的 `false` 以进入 type pack 子树。
  pub fn visit_ast_type_pack(&mut self) -> bool {
    true
  }
}
