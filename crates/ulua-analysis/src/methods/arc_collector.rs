//! `arc_collector` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_ast::records::{
  ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
  ast_expr_local::AstExprLocal, ast_stat_function::AstStatFunction,
  ast_stat_local_function::AstStatLocalFunction, ast_stat_type_alias::AstStatTypeAlias,
  ast_type_reference::AstTypeReference, ast_type_typeof::AstTypeTypeof,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::mk_name_topo_sort_statements::{
    mk_name_ast_expr, mk_name_ast_expr_global, mk_name_ast_expr_index_name, mk_name_ast_expr_local,
    mk_name_ast_name, mk_name_ast_stat_function, mk_name_ast_stat_local_function,
    mk_name_ast_stat_type_alias,
  },
  records::{arc_collector::ArcCollector, identifier::Identifier, node::NodeId},
};

// Adds a dependency arc from the node currently being visited to the node that
// declares `name`. Mirrors `Luau::detail::ArcCollector::add`
// (`Analysis/src/TopoSortStatements.cpp:220-233`), now free of raw pointers:
// both endpoints are arena indices.

impl ArcCollector<'_> {
  pub fn add(&mut self, name: &Identifier) {
    // `map.find(name)` — unknown identifier: nothing to link.
    let Some(&to) = self.map.find(name) else {
      return;
    };
    // `currentArc` is always positioned by `toposort` before any visit runs.
    let Some(from) = self.current_arc else {
      return;
    };
    // Self-dependency arcs are meaningless (and would pin the node).
    if to == from {
      return;
    }

    self.arena[to].provides.insert(from);
    self.arena[from].depends.insert(to);
  }
}

// Populates the collector's declaration map and wires `IdentifierHash` into
// the container's `DenseHasher` trait. Mirrors the body of the C++
// `ArcCollector::ArcCollector` loop (`Analysis/src/TopoSortStatements.cpp:207-217`):
// for every node with a name, register the *first* node declaring that name.

impl ArcCollector<'_> {
  /// Build `map: name → first declaring node` over `nodes` (indices into the
  /// arena). `default()` 起步的空键占位为 `Identifier::default()`（`("", null)`，
  /// 与 cpp `map{}` 起步同构）；占用由位图判定，该占位不是保留键（见
  /// ulua-common `dense_hash_table` 模块文档）。
  pub fn populate_map(&mut self, nodes: impl Iterator<Item = NodeId>) {
    self.map = DenseHashMap::default();

    for index in nodes {
      let name = match &self.arena[index].name {
        Some(name) => name.clone(),
        None => continue,
      };
      if !self.map.contains(&name) {
        *self.map.get_or_insert(name) = index;
      }
    }
  }
}

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
