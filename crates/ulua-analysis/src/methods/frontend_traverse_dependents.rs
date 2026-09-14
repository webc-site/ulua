use alloc::{sync::Arc, vec::Vec};

use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  records::{frontend::Frontend, source_node::SourceNode},
  type_aliases::module_name_type::ModuleName,
};
impl Frontend {
  pub fn traverse_dependents(
    &mut self,
    name: &ModuleName,
    process_subtree: Box<dyn Fn(&mut SourceNode) -> bool>,
  ) {
    LUAU_TIMETRACE_SCOPE!("Frontend::traverseDependents", "Frontend");

    if !self.source_nodes.contains_key(name) {
      return;
    }

    let mut queue = Vec::new();
    queue.push(name.clone());

    while let Some(next) = queue.pop() {
      debug_assert!(self.source_nodes.contains_key(&next));
      let source_node_arc = match self.source_nodes.get(&next) {
        Some(v) => v,
        None => continue,
      };

      // Clone to avoid borrowing `self.source_nodes` across callback and subsequent queue mutations.
      let source_node_ptr = Arc::as_ptr(source_node_arc).cast_mut();

      // SAFETY: `SourceNode` is owned by `Frontend.source_nodes` as an `Arc`. We do not free it here,
      // and callback is expected to mutate the node. We avoid aliasing `&mut` borrows by using the raw pointer.
      let keep_going = unsafe { process_subtree(&mut *source_node_ptr) };

      if !keep_going {
        continue;
      }

      let dependents = unsafe { &(*source_node_ptr).dependents };
      for d in dependents.iter() {
        queue.push(d.clone());
      }
    }
  }
}
