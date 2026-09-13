use core::ptr::null;
use std::ptr::eq;
extern crate alloc;

use alloc::{sync::Arc, vec::Vec};
use std::collections::HashMap;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  records::{file_resolver::FileResolver, require_cycle::RequireCycle, source_node::SourceNode},
  type_aliases::module_name_type::ModuleName,
};

pub fn get_require_cycles(
  _resolver: &FileResolver,
  source_nodes: &HashMap<ModuleName, Arc<SourceNode>>,
  start: &SourceNode,
) -> Vec<RequireCycle> {
  let mut result: Vec<RequireCycle> = Vec::new();

  let mut seen: DenseHashSet<*const SourceNode> = DenseHashSet::new(null());
  let mut stack: Vec<*const SourceNode> = Vec::new();
  let mut path: Vec<*const SourceNode> = Vec::new();

  for (dep_name, dep_location) in &start.require_locations {
    let mut cycle: Vec<ModuleName> = Vec::new();

    let dit = source_nodes.get(dep_name);
    if dit.is_none() {
      continue;
    }

    stack.push(dit.unwrap().as_ref() as *const SourceNode);

    while let Some(top_ptr) = stack.pop() {
      let top: *const SourceNode = top_ptr;

      if top.is_null() {
        // special marker for post-order processing
        LUAU_ASSERT!(!path.is_empty());
        let last_path = *path.last().unwrap();
        path.pop();

        // we reached the node! path must form a cycle now
        if eq(last_path, start) {
          for &node in &path {
            // Safety: nodes in path are real pointers (never null)
            cycle.push(unsafe { (*node).name.clone() });
          }

          // Safety: last_path is part of the cycle, so it's non-null
          cycle.push(unsafe { (*last_path).name.clone() });
          break;
        }
      } else if !seen.contains(&top) {
        // Safety: top is a real pointer (never null)
        seen.insert(top);

        // push marker for post-order processing
        path.push(top);
        stack.push(null());

        // note: we push require edges in the opposite order
        // because it's a stack, the last edge to be pushed gets processed first
        // this ensures that the cyclic path we report is the first one in DFS order
        let require_locations = unsafe { &(*top).require_locations };
        for i in (1..=require_locations.len()).rev() {
          let req_name = &require_locations[i - 1].0;

          let rit = source_nodes.get(req_name);
          if let Some(rit_node) = rit {
            stack.push(rit_node.as_ref() as *const SourceNode);
          }
        }
      }
    }

    path.clear();
    stack.clear();

    if !cycle.is_empty() {
      // Safety: dep_location is owned by start and cycle is built from valid nodes.
      result.push(RequireCycle {
        location: *dep_location,
        path: cycle,
      });

      // only clear seen vector when we find a cycle
      seen.clear();
    }
  }

  result
}
