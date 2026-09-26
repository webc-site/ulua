//! Drain `Q` until the target's `depends` arcs are satisfied. `target` is always
//! added to the result. Mirrors `Luau::detail::drain`
//! (`Analysis/src/TopoSortStatements.cpp:404-497`).
//!
//! The graph is the shared `arena: Vec<Node>`; nodes are named by index. The
//! only raw pointers are the `AstStat*` payload (`node.element`), which live in
//! the AST arena and are copied into `result` by value.
use alloc::{
  collections::{BTreeMap, BTreeSet},
  vec::Vec,
};

use ulua_ast::records::ast_stat::AstStat;
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
  // Safety: element 是建图期从 parser arena 拷入的非空 *mut AstStat（TopoSortStatements
  // 全程只搬运该指针、从不置空），AST arena 块地址不移动且比图/SourceModule 同寿；
  // 此处仅重建共享只读借用，单线程遍历期间无人写 AST，无别名冲突。
  unsafe { &*node.element }
}

pub fn drain(
  arena: &mut [Node],
  q: &mut NodeList,
  result: &mut Vec<*mut AstStat>,
  target: Option<NodeId>,
) {
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
      return;
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
}
