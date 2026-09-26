use alloc::{
  collections::{BTreeMap, BTreeSet},
  sync::Arc,
  vec::Vec,
};
use core::cmp::Reverse;

use crate::{
  records::{frontend_module_resolver::FrontendModuleResolver, source_node::SourceNode},
  type_aliases::{error_vec::ErrorVec, module_name_type::ModuleName},
};
pub fn accumulate_errors(
  source_nodes: &BTreeMap<ModuleName, Arc<SourceNode>>,
  module_resolver: &FrontendModuleResolver,
  name: &ModuleName,
) -> ErrorVec {
  let mut seen: BTreeSet<ModuleName> = BTreeSet::new();
  let mut queue: Vec<ModuleName> = vec![name.clone()];
  let mut result = ErrorVec::new();

  while let Some(next) = queue.pop() {
    if seen.contains(&next) {
      continue;
    }
    seen.insert(next.clone());

    let Some(source_node) = source_nodes.get(&next) else {
      continue;
    };

    for dependency in source_node.require_set.iter() {
      queue.push(dependency.clone());
    }

    let module = {
      // parking_lot 互斥量无投毒语义，与 cpp `std::lock_guard` 完全一致。
      let _lock = module_resolver.module_mutex.lock();
      module_resolver.modules.get(&next).cloned()
    };
    let Some(module) = module else {
      continue;
    };

    let prev_size = result.len();
    result.extend(module.errors.iter().rev().cloned());
    result[prev_size..].sort_by_key(|e| Reverse(e.location.begin));
  }

  result.reverse();
  result
}
