//! Faithful port of `Luau::detail::drain`
//! (`Analysis/src/TopoSortStatements.cpp:404-497`).
//!
//! Drain `Q` until the target's `depends` arcs are satisfied. `target` is always
//! added to the result.
use alloc::{
  collections::{BTreeMap, BTreeSet},
  vec::Vec,
};
use core::ptr::null_mut;

use ulua_ast::records::ast_stat::AstStat;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{is_block_terminator::is_block_terminator, prune::prune},
  records::{arcs::Arcs, node::Node},
  type_aliases::node_list::NodeList,
};
/// # Safety
/// 调用方须保证 `target` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn drain(q: &mut NodeList, result: &mut Vec<*mut AstStat>, target: *mut Node) {
  // Trying to toposort a subgraph is a pretty big hassle. :(
  // Some of the nodes in .depends and .provides aren't present in our subgraph

  // std::map<Node*, Arcs> allArcs;
  let mut all_arcs: BTreeMap<*mut Node, Arcs> = BTreeMap::new();

  // `DenseHashSet<Node*> elements` — the set of nodes present in Q. In C++ it
  // is (redundantly) rebuilt once per outer iteration; it is constant, so we
  // build it a single time.
  let elements: BTreeSet<*mut Node> = q.iter().copied().collect();

  // for (auto& node : Q) { ... copy connectivity filtered to Q ... }
  for &node_ptr in q.iter() {
    let mut arcs = Arcs::new();
    let node = unsafe { &*node_ptr };

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

    all_arcs.insert(node_ptr, arcs);
  }

  // while (!Q.empty())
  while !q.is_empty() {
    // if (target && target->depends.empty()) { prune(target); push; return; }
    if !target.is_null() && unsafe { (*target).depends.is_empty() } {
      unsafe { prune(target) };
      result.push(unsafe { (*target).element });
      return;
    }

    let mut next_node: *mut Node = null_mut();

    // Find the first non-terminator node whose (filtered) depends are empty.
    // 注：NodeList 底层是 ulua-common 的 VecDeque，未提供 iter()，暂用索引遍历
    for i in 0..q.len() {
      let candidate = q[i];
      if is_block_terminator(unsafe { &*(*candidate).element }) {
        continue;
      }

      LUAU_ASSERT!(all_arcs.contains_key(&candidate));

      if all_arcs[&candidate].depends.is_empty() {
        next_node = candidate;
        q.remove(i);
        break;
      }
    }

    // if (!nextNode) { nextNode = std::move(Q.front()); Q.pop_front(); }
    if next_node.is_null() {
      // We've hit a cycle or a terminator. Pick an arbitrary node.
      next_node = *q.front().unwrap();
      q.pop_front();
    }

    // Remove nextNode from the filtered arcs of its neighbours.
    let provides: Vec<*mut Node> = unsafe { (*next_node).provides.iter().copied().collect() };
    let depends: Vec<*mut Node> = unsafe { (*next_node).depends.iter().copied().collect() };

    for node in provides {
      if let Some(arcs) = all_arcs.get_mut(&node) {
        let removed = arcs.depends.remove(&next_node);
        LUAU_ASSERT!(removed);
      }
    }

    for node in depends {
      if let Some(arcs) = all_arcs.get_mut(&node) {
        let removed = arcs.provides.remove(&next_node);
        LUAU_ASSERT!(removed);
      }
    }

    unsafe { prune(next_node) };
    result.push(unsafe { (*next_node).element });
  }

  // if (target) { prune(target); result.push_back(target->element); }
  if !target.is_null() {
    unsafe { prune(target) };
    result.push(unsafe { (*target).element });
  }
}
