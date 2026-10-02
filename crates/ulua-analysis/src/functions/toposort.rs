//! Faithful port of `Luau::toposort`
//! (`Analysis/src/TopoSortStatements.cpp:520-583`).
//!
//! Decide the order in which to typecheck a block of statements: build a
//! dependency graph (uses → declarations, plus a chain through imperative
//! statements), then walk it Kahn-style, deferring function/type definitions
//! into a queue `Q` that is toposorted on demand.
// Wire `ArcCollector` (a C++ `AstVisitor` subclass) into the Rust `AstVisitor`
// trait so `ast_stat_visit_ref` dispatches to its overrides. Each trait method
// just forwards to the inherent `visit_ast_*` method that carries the ported
// body; the un-overridden methods keep the base `AstVisitor` defaults — exactly
// the set the C++ class overrides (the `AstExpr*`/`AstStat*`/`AstType*` ones).
use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_stat::AstStat, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_type_alias::AstStatTypeAlias,
    ast_type::AstType, ast_type_pack::AstTypePack, ast_type_reference::AstTypeReference,
    ast_type_typeof::AstTypeTypeof, ast_visitor::AstVisitor, node_handle::Node as StatHandle,
  },
  visit::ast_stat_visit_ref,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    contains_function_call::contains_function_call, drain::drain,
    is_block_terminator::is_block_terminator, is_toposortable_node::is_toposortable_node,
    mk_name_topo_sort_statements::mk_name_ast_stat, prune::prune,
  },
  records::{
    arc_collector::ArcCollector,
    identifier::Identifier,
    identifier_hash::IdentifierHash,
    node::{Node, NodeId},
  },
  type_aliases::{node_list::NodeList, node_queue::NodeQueue},
};
impl AstVisitor for ArcCollector<'_> {
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_ast_expr_global(node)
  }
  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    self.visit_ast_expr_local(node)
  }
  fn visit_expr_index_name(&mut self, node: &mut AstExprIndexName) -> bool {
    self.visit_ast_expr_index_name(node)
  }
  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_ast_stat_function(node)
  }
  fn visit_stat_local_function(&mut self, node: &mut AstStatLocalFunction) -> bool {
    self.visit_ast_stat_local_function(node)
  }
  fn visit_stat_type_alias(&mut self, node: &mut AstStatTypeAlias) -> bool {
    self.visit_ast_stat_type_alias(node)
  }
  fn visit_type(&mut self, _node: &mut AstType) -> bool {
    self.visit_ast_type()
  }
  fn visit_type_reference(&mut self, node: &mut AstTypeReference) -> bool {
    self.visit_ast_type_reference(node)
  }
  fn visit_type_typeof(&mut self, node: &mut AstTypeTypeof) -> bool {
    self.visit_ast_type_typeof(node)
  }
  fn visit_type_pack(&mut self, _node: &mut AstTypePack) -> bool {
    self.visit_ast_type_pack()
  }
}

/// 返回排序后的语句序列；cpp 的早退（不可排序时保持 `stats` 原样）在此
/// 折为「返回原序拷贝」，末尾 `std::swap(stats, result)` 折为返回 `result`。
/// 载荷为 arena 句柄 `Node<AstStat>`（原 `*mut AstStat`），身份判等仍按指针地址，
/// 与 cpp 逐格同构。
pub fn toposort(stats: &[StatHandle<AstStat>]) -> Vec<StatHandle<AstStat>> {
  // if (stats.empty()) return;
  if stats.is_empty() {
    return Vec::new();
  }

  // if (!containsToposortableNode(stats)) return;
  // (Inlined: the helper's signature does not match the handle slice.)
  if !stats.iter().any(|stat| is_toposortable_node(stat.get())) {
    return stats.to_vec();
  }

  // std::vector<AstStat*> result; result.reserve(stats.size());
  let mut result: Vec<StatHandle<AstStat>> = Vec::with_capacity(stats.len());

  // The dependency graph now lives in a dense `Vec<Node>` arena addressed by
  // index (`NodeId`), replacing the C++ `unique_ptr<Node>`s that `nodes`/`Q`
  // owned. `nodes` is the initial work queue of every arena index; it is
  // consumed head-first (Kahn order) so the index order matches the C++ push
  // order. `q` is the deferred-node queue drained on demand.
  let mut arena: Vec<Node> = Vec::with_capacity(stats.len());
  let mut nodes: NodeQueue = NodeQueue::new();
  let mut q: NodeList = NodeList::new();

  // for (AstStat* stat : stats) nodes.push_back(new Node(mkName(stat), stat));
  // `elements` is a copy of the arena handle payload so the visitor loop never
  // borrows `arena` while `collector` mutably holds it.
  let mut elements: Vec<StatHandle<AstStat>> = Vec::with_capacity(stats.len());
  for &stat in stats.iter() {
    let id: NodeId = arena.len();
    arena.push(Node::new(mk_name_ast_stat(Some(stat.get())), stat));
    elements.push(stat);
    nodes.push_back(id);
  }

  // ArcCollector collector{nodes}; for (node : nodes) { collector.currentArc =
  // node; node->element->visit(&collector); }
  // Scoped so the collector's mutable borrow of `arena` ends before the graph is
  // mutated below.
  {
    let mut collector = ArcCollector {
      arena: &mut arena,
      map: DenseHashMap::<Identifier, NodeId, IdentifierHash>::default(),
      current_arc: None,
    };
    collector.populate_map(nodes.iter().copied());
    // `elements` 持有 arena 句柄的独立拷贝（与 collector 的 `arena` 借用不相交），
    // 逐槽 `get_mut` 交出 `&mut AstStat` 喂给安全引用形态的 `ast_stat_visit_ref`
    // ——对应 cpp `node->element->visit(&collector)` 的非 const 写穿语义；独占性
    // 由 TopoSort 阶段 AST 定稿不变量保证（visitor 只读 AST、不改写节点，collector
    // 只写与 arena 句柄目标不相交的 Node 图 arena 与局部 map）。原裸指针 `unsafe`
    // 门面就此收敛为句柄安全借用,本函数不再需要 `unsafe`。
    for (id, element) in elements.iter_mut().enumerate() {
      collector.current_arc = Some(id);
      ast_stat_visit_ref(element.get_mut(), &mut collector);
    }
  }

  // Prev-statement edges: chain each non-toposortable statement to the
  // previous non-toposortable statement. `prev` lags one non-toposortable index
  // behind the scan cursor `it`.
  {
    let mut prev: usize = 0;
    for (it, &it_id) in nodes.iter().enumerate() {
      if it != prev && !is_toposortable_node(arena[it_id].element.get()) {
        let prev_id: NodeId = *nodes.at(prev);
        arena[it_id].depends.insert(prev_id);
        arena[prev_id].provides.insert(it_id);
        prev = it;
      }
    }
  }

  // while (!nodes.empty()) { ... }
  while !nodes.empty() {
    let next: NodeId = *nodes.front();

    if arena[next].depends.is_empty() && !is_block_terminator(arena[next].element.get()) {
      prune(&mut arena, next);
      result.push(arena[next].element);
    } else if !contains_function_call(arena[next].element.get()) {
      q.push_back(next);
    } else {
      result.extend(drain(&mut arena, &mut q, Some(next)));
    }

    nodes.pop_front();
  }

  // drain(Q, result, nullptr);
  result.extend(drain(&mut arena, &mut q, None));

  result
}
