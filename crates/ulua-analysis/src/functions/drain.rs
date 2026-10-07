//! Drain `Q` until the target's `depends` arcs are satisfied. `target` is always
//! added to the result. Mirrors `Luau::detail::drain`
//! (`Analysis/src/TopoSortStatements.cpp:404-497`).
//!
//! The graph is the shared `arena: Vec<Node>`; nodes are named by index. The
//! only arena payload is the `Node<AstStat>` handle (`node.element`), which
//! lives in the AST arena and is copied into `result` by value.
use alloc::{
  collections::{BTreeMap, BTreeSet},
  vec::Vec,
};

use ulua_ast::records::{ast_stat::AstStat, node_handle::Node as StatHandle};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{is_block_terminator::is_block_terminator, prune::prune},
  records::{
    arcs::Arcs,
    node::{Node, NodeId},
  },
  type_aliases::node_list::NodeList,
};

fn stat_of(node: &Node) -> &AstStat {
  node.element.get()
}

/// 返回本轮摘除并加入结果的语句序列（cpp 的 `result` 出参折为返回值），
/// 顺序与 cpp 逐次 `result.push_back` 一致。载荷为 arena 句柄，身份判等按地址。
pub fn drain(
  arena: &mut [Node],
  q: &mut NodeList,
  target: Option<NodeId>,
) -> Vec<StatHandle<AstStat>> {
  // cpp 中调用方传入的 `result`：本函数只追加，故收拢为局部返回值。
  let mut result: Vec<StatHandle<AstStat>> = Vec::new();

  // Connectivity of the subgraph induced by the nodes currently in `Q`. In C++
  // `elements` was redundantly rebuilt each outer iteration; it is loop-invariant.
  let elements: BTreeSet<NodeId> = q.iter().copied().collect();
  let mut all_arcs: BTreeMap<NodeId, Arcs> = BTreeMap::new();
  for &id in q.iter() {
    let mut arcs = Arcs::new();
    let node = &arena[id];
    for &dep in node.depends.iter() {
      if elements.contains(&dep) {
        arcs.depends.insert(dep);
      }
    }
    for &prov in node.provides.iter() {
      if elements.contains(&prov) {
        arcs.provides.insert(prov);
      }
    }
    all_arcs.insert(id, arcs);
  }

  while !q.is_empty() {
    // A ready target short-circuits the drain.
    if let Some(t) = target
      && arena[t].depends.is_empty()
    {
      prune(arena, t);
      result.push(arena[t].element);
      return result;
    }

    // First non-terminator whose (filtered) dependencies are all satisfied.
    let mut next: Option<NodeId> = None;
    let mut found_pos = None;
    for (i, &candidate) in q.iter().enumerate() {
      if is_block_terminator(stat_of(&arena[candidate])) {
        continue;
      }
      LUAU_ASSERT!(all_arcs.contains_key(&candidate));
      if all_arcs[&candidate].depends.is_empty() {
        next = Some(candidate);
        found_pos = Some(i);
        break;
      }
    }
    if let Some(i) = found_pos {
      q.remove(i);
    }

    // Otherwise a cycle or a terminator blocks progress: take an arbitrary node.
    let next = match next {
      Some(n) => n,
      None => {
        // 此臂 `next`/`found_pos` 成对为 None，本轮无 remove 摘除，while 头
        // `!q.is_empty()` 蕴含 front() 命中 Some。
        let n = *q
          .front()
          .expect("found_pos 同臂为 None 未摘除，循环头蕴含 q 非空");
        q.pop_front();
        n
      }
    };

    // Drop `next` from its neighbours' filtered connectivity.
    let provides: Vec<NodeId> = arena[next].provides.iter().copied().collect();
    let depends: Vec<NodeId> = arena[next].depends.iter().copied().collect();
    for node in provides {
      if let Some(arcs) = all_arcs.get_mut(&node) {
        let removed = arcs.depends.remove(&next);
        LUAU_ASSERT!(removed);
      }
    }
    for node in depends {
      if let Some(arcs) = all_arcs.get_mut(&node) {
        let removed = arcs.provides.remove(&next);
        LUAU_ASSERT!(removed);
      }
    }

    prune(arena, next);
    result.push(arena[next].element);
  }

  if let Some(t) = target {
    prune(arena, t);
    result.push(arena[t].element);
  }

  result
}
