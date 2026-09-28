//! Faithful port of `Luau::toposort`
//! (`Analysis/src/TopoSortStatements.cpp:520-583`).
//!
//! Decide the order in which to typecheck a block of statements: build a
//! dependency graph (uses → declarations, plus a chain through imperative
//! statements), then walk it Kahn-style, deferring function/type definitions
//! into a queue `Q` that is toposorted on demand.
// Wire `ArcCollector` (a C++ `AstVisitor` subclass) into the Rust `AstVisitor`
// trait so `ast_stat_visit` dispatches to its overrides. Each trait method just
// forwards to the inherent `visit_ast_*` method that carries the ported body;
// the un-overridden methods keep the base `AstVisitor` defaults — exactly the
// set the C++ class overrides (the `AstExpr*`/`AstStat*`/`AstType*` ones).
use alloc::vec::Vec;
use core::mem::swap;

use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_stat::AstStat, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_type_alias::AstStatTypeAlias,
    ast_type::AstType, ast_type_pack::AstTypePack, ast_type_reference::AstTypeReference,
    ast_type_typeof::AstTypeTypeof, ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit,
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

pub fn toposort(stats: &mut Vec<*mut AstStat>) {
  // if (stats.empty()) return;
  if stats.is_empty() {
    return;
  }

  // if (!containsToposortableNode(stats)) return;
  // (Inlined: the helper's signature does not match `Vec<*mut AstStat>`.)
  if !stats.iter().any(|&stat| {
    // Safety: `stats` 元素是 parser 写入 AST arena 的块语句指针——非空、指向
    // repr(C) AstStat 前缀节点且整个 toposort 期间存活；此处仅只读重建 `&`
    // 供 class_index 判定（TopoSort 阶段 AST 已定稿，无并存可变访问）。
    is_toposortable_node(unsafe { &*stat })
  }) {
    return;
  }

  // std::vector<AstStat*> result; result.reserve(stats.size());
  let mut result: Vec<*mut AstStat> = Vec::with_capacity(stats.len());

  // The dependency graph now lives in a dense `Vec<Node>` arena addressed by
  // index (`NodeId`), replacing the C++ `unique_ptr<Node>`s that `nodes`/`Q`
  // owned. `nodes` is the initial work queue of every arena index; it is
  // consumed head-first (Kahn order) so the index order matches the C++ push
  // order. `q` is the deferred-node queue drained on demand.
  let mut arena: Vec<Node> = Vec::with_capacity(stats.len());
  let mut nodes: NodeQueue = NodeQueue::new();
  let mut q: NodeList = NodeList::new();

  // for (AstStat* stat : stats) nodes.push_back(new Node(mkName(stat), stat));
  // `elements` is a copy of the raw `AstStat*` payload so the visitor loop never
  // borrows `arena` while `collector` mutably holds it.
  let mut elements: Vec<*mut AstStat> = Vec::with_capacity(stats.len());
  for &stat in stats.iter() {
    let id: NodeId = arena.len();
    // Safety: 满足 `mk_name_ast_stat` 的参数契约——`stat` 是 parser 写入 AST
    // arena 的存活 AstStat 家族节点（非 null），函数体仅按 class_index 只读
    // 下转取名字；TopoSort 阶段该 arena 已定稿，无并存的 AST 可变访问。
    arena.push(Node::new(unsafe { mk_name_ast_stat(stat) }, stat));
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
    for (id, &element) in elements.iter().enumerate() {
      collector.current_arc = Some(id);
      // Safety: `element` 即入参 `stats`（本函数独占的 &mut Vec）中的
      // arena 存活 AstStat 指针，满足 ast_stat_visit 的“null 或存活节点”
      // 契约；RTTI class index 分发到对应 visit 覆写。独占性由 `stats` 的
      // `&mut` 借用与 TopoSort 阶段 AST 定稿不变量共同保证；collector 只写
      // 与其不相交的 Node 图 arena 与局部 map。
      unsafe {
        ast_stat_visit(element, &mut collector);
      }
    }
  }

  // Prev-statement edges: chain each non-toposortable statement to the
  // previous non-toposortable statement. `prev` lags one non-toposortable index
  // behind the scan cursor `it`.
  {
    let mut prev: usize = 0;
    for (it, &it_id) in nodes.iter().enumerate() {
      // Safety: `arena[it_id].element` 是建表时从 `stats` 值拷贝的 arena 存活
      // AstStat 指针（从未被改写），只读重建 `&` 供 class_index 判定；该借用
      // 止于条件求值，与随后对 arena 的 insert 写入不重叠。
      if it != prev && !is_toposortable_node(unsafe { &*arena[it_id].element }) {
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

    // Safety: `arena[next].element` 是建表时值拷贝自 `stats` 的 arena 存活
    // AstStat 指针，仅只读重建 `&` 供 class_index 判定；借用止于条件求值。
    if arena[next].depends.is_empty() && !is_block_terminator(unsafe { &*arena[next].element }) {
      prune(&mut arena, next);
      result.push(arena[next].element);
    } else if !contains_function_call(
      // Safety: 同一元素指针、同一不变量——arena 存活非空，只读 `&` 重建
      // 供分类判定，与随后 drain/prune 对 Node 图 arena 的可变访问不相交。
      unsafe { &*arena[next].element },
    ) {
      q.push_back(next);
    } else {
      drain(&mut arena, &mut q, &mut result, Some(next));
    }

    nodes.pop_front();
  }

  // drain(Q, result, nullptr);
  drain(&mut arena, &mut q, &mut result, None);

  // std::swap(stats, result);
  swap(stats, &mut result);
}
