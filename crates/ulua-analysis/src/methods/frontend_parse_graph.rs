use alloc::vec::Vec;

use ulua_common::{
  macros::{
    luau_assert::LUAU_ASSERT,
    luau_timetrace_scope::{LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_SCOPE},
  },
  records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::mark::Mark,
  records::{
    arena_handle::{alias, alias_ref},
    frontend::Frontend,
    source_node::SourceNode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
/// 遍历栈帧：`Node` 为待前序访问的 SourceNode 指针；`Mark` 为后序处理标记帧，
/// 取代旧 `null_mut()` 哨兵（迭代序/环检测语义与原实现逐字一致）。
enum ParseFrame {
  /// 与 `path` 栈顶配对的后序标记（原 `stack.push(null_mut())` 哨兵位）。
  Mark,
  /// 非空 SourceNode 指针帧。
  Node(*mut SourceNode),
}

/// `require_set` 子依赖解析三态：`Skip` = 已知且不脏（关键优化，跳过遍历）；
/// `Unresolved` = 尚无 SourceNode（落到 `get_source_node`）；`Dirty` = 已知脏节点。
/// 取代旧 `Some(null_mut())` 待建局部哨兵。
enum KnownDep {
  Skip,
  Unresolved,
  Dirty(*mut SourceNode),
}

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
    let mut stack: Vec<ParseFrame> = Vec::new();
    let mut path: Vec<*mut SourceNode> = Vec::new();
    let mut cyclic = false;

    {
      let (source_node, _) = self.get_source_node(root, limits);
      if let Some(source_node) = source_node {
        stack.push(ParseFrame::Node(source_node.as_ptr()));
      }
    }

    while let Some(top) = stack.pop() {
      match top {
        ParseFrame::Mark => {
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

          build_queue.push(alias_ref(top).name.clone());

          // at this point we know all valid dependencies are processed into SourceNodes
          let require_set = &alias_ref(top).require_set;
          for dep in require_set.iter() {
            if let Some(source_node_arc) = self.source_nodes.get(dep) {
              let source_node_ref: *mut SourceNode =
                source_node_arc.as_ref() as *const SourceNode as *mut SourceNode;
              let dependents = &mut alias(source_node_ref).dependents;
              dependents.insert(alias_ref(top).name.clone());
            }
          }
        }
        ParseFrame::Node(top) => {
          // note: topseen ref gets invalidated in any seen[] access, beware - only one seen[] access per iteration!
          let topseen = seen.get_or_insert(top);

          if *topseen != Mark::None {
            cyclic |= *topseen == Mark::Temporary;
            continue;
          }

          *topseen = Mark::Temporary;

          // push marker for post-order processing
          stack.push(ParseFrame::Mark);
          path.push(top);

          // push children
          let require_set = &alias_ref(top).require_set;
          for dep in require_set.iter() {
            // Resolve the already-known SourceNode pointer (if any) in a scope so the
            // immutable borrow of `self.source_nodes` is released before the mutable
            // `self.get_source_node` call below.
            let known: KnownDep = {
              if let Some(source_node_arc) = self.source_nodes.get(dep) {
                // this is a critical optimization: we do *not* traverse non-dirty subtrees.
                // this relies on the fact that markDirty marks reverse-dependencies dirty as well
                // thus if a node is not dirty, all its transitive deps aren't dirty, which means that they won't ever need
                // to be built, *and* can't form a cycle with any nodes we did process.
                if !source_node_arc.has_dirty_module(for_autocomplete) {
                  KnownDep::Skip
                } else {
                  KnownDep::Dirty(source_node_arc.as_ref() as *const SourceNode as *mut SourceNode)
                }
              } else {
                // no SourceNode known yet; fall through to getSourceNode
                KnownDep::Unresolved
              }
            };

            match known {
              // dependency is known but not dirty: skip
              KnownDep::Skip => continue,
              // dependency is known and dirty
              KnownDep::Dirty(ptr) => {
                // This module might already be in the outside build queue
                // Note: canSkip is not available in this context, so we skip this check

                // note: this check is technically redundant *except* that getSourceNode has somewhat broken memoization
                // calling getSourceNode twice in succession will reparse the file, since getSourceNode leaves dirty flag set
                if seen.contains_key(&ptr) {
                  stack.push(ParseFrame::Node(ptr));
                  continue;
                }
              }
              // dependency not yet resolved
              KnownDep::Unresolved => {}
            }

            let (source_node, _) = self.get_source_node(dep, limits);
            if let Some(source_node) = source_node {
              let source_node = source_node.as_ptr();
              stack.push(ParseFrame::Node(source_node));

              // note: this assignment is paired with .contains() check above and effectively deduplicates getSourceNode()
              seen.try_insert(source_node, Mark::None);
            }
          }
        }
      }
    }

    cyclic
  }
}
