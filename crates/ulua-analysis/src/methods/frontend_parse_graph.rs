use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_common::{
  macros::{
    luau_assert::LUAU_ASSERT,
    luau_timetrace_scope::{LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_SCOPE},
  },
  records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::mark::Mark,
  records::{frontend::Frontend, source_node::SourceNode, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
impl Frontend {
  pub fn parse_graph(
    &mut self,
    build_queue: &mut Vec<ModuleName>,
    root: &ModuleName,
    limits: &TypeCheckLimits,
    for_autocomplete: bool,
  ) -> bool {
    LUAU_TIMETRACE_SCOPE!("Frontend::parseGraph", "Frontend");
    LUAU_TIMETRACE_ARGUMENT!("root", root.as_str());

    let mut seen: DenseHashMap<*mut SourceNode, Mark> = DenseHashMap::default();
    let mut stack: Vec<*mut SourceNode> = Vec::new();
    let mut path: Vec<*mut SourceNode> = Vec::new();
    let mut cyclic = false;

    {
      let (source_node, _) = self.get_source_node(root, limits);
      if let Some(source_node) = source_node {
        stack.push(source_node.as_ptr());
      }
    }

    while let Some(top) = stack.pop() {
      if top.is_null() {
        // special marker for post-order processing
        LUAU_ASSERT!(!path.is_empty());

        // 紧邻 LUAU_ASSERT(!path.is_empty()) 蕴含 pop() 命中 Some。
        let top = path
          .pop()
          .expect("紧邻 LUAU_ASSERT(!path.is_empty()) 蕴含非空");

        // note: topseen ref gets invalidated in any seen[] access, beware - only one seen[] access per iteration!
        let topseen = seen.get_or_insert(top);
        LUAU_ASSERT!(*topseen == Mark::Temporary);
        *topseen = Mark::Permanent;

        // Safety: top 源自 get_source_node / `Arc::as_ref` 得到的存活 SourceNode，由
        // `self.source_nodes` 的 Arc 持有且 `&mut self` 全程有效，非空且对齐，只读 name。
        build_queue.push(unsafe { (*top).name.clone() });

        // at this point we know all valid dependencies are processed into SourceNodes
        // Safety: 同上 top 存活非空，取只读借用遍历其 require_set 字段。
        let require_set = unsafe { &(*top).require_set };
        for dep in require_set.iter() {
          if let Some(source_node_arc) = self.source_nodes.get(dep) {
            let source_node_ref: *mut SourceNode =
              source_node_arc.as_ref() as *const SourceNode as *mut SourceNode;
            // Safety: source_node_ref 由 `self.source_nodes` 中 Arc<SourceNode> 经 as_ref 得到，
            // Frontend 是其唯一持有者、单线程串行遍历，故该写句柄指向存活对齐的 SourceNode 且
            // 无并发借用；对 dependents 字段的写与正在迭代的 require_set 字段内存不相交。
            let dependents = unsafe { &mut (*source_node_ref).dependents };
            // Safety: top 存活非空（同上），只读 name 后 clone 作为插入值。
            dependents.insert(unsafe { (*top).name.clone() });
          }
        }
      } else {
        // note: topseen ref gets invalidated in any seen[] access, beware - only one seen[] access per iteration!
        let topseen = seen.get_or_insert(top);

        if *topseen != Mark::None {
          cyclic |= *topseen == Mark::Temporary;
          continue;
        }

        *topseen = Mark::Temporary;

        // push marker for post-order processing
        stack.push(null_mut());
        path.push(top);

        // push children
        // Safety: 此为 else 分支，top 非空（`!top.is_null()` 成立）且为存活 SourceNode，
        // 取只读借用遍历其 require_set。
        let require_set = unsafe { &(*top).require_set };
        for dep in require_set.iter() {
          // Resolve the already-known SourceNode pointer (if any) in a scope so the
          // immutable borrow of `self.source_nodes` is released before the mutable
          // `self.get_source_node` call below.
          let known: Option<*mut SourceNode> = {
            if let Some(source_node_arc) = self.source_nodes.get(dep) {
              // this is a critical optimization: we do *not* traverse non-dirty subtrees.
              // this relies on the fact that markDirty marks reverse-dependencies dirty as well
              // thus if a node is not dirty, all its transitive deps aren't dirty, which means that they won't ever need
              // to be built, *and* can't form a cycle with any nodes we did process.
              if !source_node_arc.has_dirty_module(for_autocomplete) {
                None
              } else {
                Some(source_node_arc.as_ref() as *const SourceNode as *mut SourceNode)
              }
            } else {
              // no SourceNode known yet; fall through to getSourceNode
              Some(null_mut())
            }
          };

          match known {
            // dependency is known but not dirty: skip
            None => continue,
            // dependency is known and dirty
            Some(ptr) if !ptr.is_null() => {
              // This module might already be in the outside build queue
              // Note: canSkip is not available in this context, so we skip this check

              // note: this check is technically redundant *except* that getSourceNode has somewhat broken memoization
              // calling getSourceNode twice in succession will reparse the file, since getSourceNode leaves dirty flag set
              if seen.contains_key(&ptr) {
                stack.push(ptr);
                continue;
              }
            }
            // dependency not yet resolved
            Some(_) => {}
          }

          let (source_node, _) = self.get_source_node(dep, limits);
          if let Some(source_node) = source_node {
            let source_node = source_node.as_ptr();
            stack.push(source_node);

            // note: this assignment is paired with .contains() check above and effectively deduplicates getSourceNode()
            seen.try_insert(source_node, Mark::None);
          }
        }
      }
    }

    cyclic
  }
}
