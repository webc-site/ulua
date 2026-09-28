//! `constraint_graph` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。
//!
//! §2（arena / 自引用图 → 索引句柄）：依赖列表原先以 `*mut ConstraintList`
//! （`PinnedStorage` 的 Box 堆址）在 `ConstraintMap` 值面与形参间横传，同一数组
//! 既 `as_ptr` 读又 `&mut *` 写，双借用全靠 `unsafe` 加「Box 地址不移动」的隐含
//! 前提维持。现改为 [`SlotArena`] 发放的 `SlotId`：读写必经 `&`/`&mut` 借用检查器，
//! 业务侧只能「先取槽号、再按槽号读写」，本文件的列表访问因此不再有一处 `unsafe`
//! （顶点格式化、`TypePackIds` 空槽哨兵等 `TypeId` 相关 `unsafe` 仍属 B 类，归 W5/W6）。

use alloc::{string::String, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_common::{
  fflag,
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, variant::Variant3},
};

use crate::{
  functions::{
    dot_escape::dot_escape,
    follow_type, follow_type_pack, get_type, get_type_pack,
    to_string_constraint_graph::to_string,
    to_string_to_string::{
      to_string_constraint_to_string_options, to_string_type_id_to_string_options,
      to_string_type_pack_id_to_string_options,
    },
  },
  records::{
    arena_handle::alias_ref,
    blocked_constraint_registry::{ConstraintId, register_constraint, resolve_constraint},
    constraint::Constraint,
    constraint_graph::{ConstraintGraph, ConstraintMap},
    constraint_list::ConstraintList,
    generic_type_visitor::GenericTypeVisitorTrait,
    primitive_type_constraint::PrimitiveTypeConstraint,
    reference_count_initializer::ReferenceCountInitializer,
    slot_arena::{SlotArena, SlotId},
    to_string_options::ToStringOptions,
    type_ids::TypeIds,
    unblocked_types::UnblockedTypes,
  },
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, bound_type::BoundType,
    bound_type_pack::BoundTypePack, constraint_v::ConstraintVMember, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_ids::TypePackIds,
  },
};

/// 依赖列表条目的初始状态（cpp `ConstraintList` 默认构造）。
fn new_constraint_list() -> ConstraintList {
  ConstraintList {
    present: DenseHashMap::default(),
    order: Vec::new(),
    entries: 0,
  }
}

/// §3（出参改返回值）：cpp `ReferenceCountInitializer` 以裸指针出参回填
/// `mutatedTypes`/`mutatedTypePacks`，三个调用点重复同一段「局部变量 + 取址」样板。
/// 这里收拢为一处，遍历入口由 `traverse` 闭包给出，结果以返回值交给调用方。
fn collect_reachable_types<T>(
  target: T,
  traverse: impl FnOnce(&mut ReferenceCountInitializer, T),
) -> (TypeIds, TypePackIds) {
  let mut mutated_types = TypeIds::new();
  let mut mutated_type_packs = TypePackIds::new(null_mut());

  let mut rci = ReferenceCountInitializer::reference_count_initializer_reference_count_initializer(
    &mut mutated_types as *mut TypeIds,
    &mut mutated_type_packs as *mut TypePackIds,
  );
  traverse(&mut rci, target);

  (mutated_types, mutated_type_packs)
}

impl ConstraintGraph {
  /// 槽号 → 只读列表视图：槽号全部由本图的 [`SlotArena`] 发放且永不回收，故解析必然成功。
  fn list(&self, id: SlotId) -> &ConstraintList {
    self
      .constraint_lists
      .get(id)
      .expect("依赖列表槽号必由本图发放")
  }

  /// 槽号 → 独占可变列表视图（同上契约；可变借用期由调用点的语句边界界定）。
  fn list_mut(&mut self, id: SlotId) -> &mut ConstraintList {
    self
      .constraint_lists
      .get_mut(id)
      .expect("依赖列表槽号必由本图发放")
  }
}

impl ConstraintGraph {
  pub fn add_dependency_of_constraint_vertex_constraint_vertex(
    &mut self,
    dependency: BlockedConstraintId,
    target: BlockedConstraintId,
  ) -> bool {
    // 两个槽号先取齐（`find_*` 的副作用是未登记时新建空列表），此后只做读写。
    let deps = self.find_dependency_list(target.clone());
    let reverse_deps = self.find_reverse_dependency_list(dependency.clone());

    if self.list(deps).contains(target.clone()) {
      LUAU_ASSERT!(self.list(reverse_deps).contains(dependency.clone()));
      return false;
    }

    self.list_mut(deps).insert(dependency.clone());
    self.list_mut(reverse_deps).insert(target);

    true
  }

  pub fn add_dependency_of_constraint_constraint(
    &mut self,
    dependency: &mut Constraint,
    target: &mut Constraint,
  ) -> bool {
    let dep_vertex = Variant3::V2(register_constraint(dependency as *const Constraint));
    let target_vertex = Variant3::V2(register_constraint(target as *const Constraint));

    self.add_dependency_of_constraint_vertex_constraint_vertex(dep_vertex, target_vertex)
  }
}

impl ConstraintGraph {
  pub fn clear_reverse_dependencies_of(&mut self, vertex: BlockedConstraintId) {
    // LUAU_ASSERT(vertex.get_if<const Constraint*>() == nullptr);
    // We cannot directly call get_if on ConstraintVertex here because it's a type alias
    // to BlockedConstraintId. The assertion is preserved as a comment since the
    // ConstraintVertex type alias already enforces this constraint at the type level.
    // The original C++ assertion checks that the vertex is not a Constraint*, which
    // is guaranteed by the type alias definition.

    let rev_deps = self.find_reverse_dependency_list(vertex.clone());

    // For all of the reverse dependencies of vertex (vertices that depend on vertex) ...
    // `order` 先快照：循环体内 `find_dependency_list` 会向槽池追加新列表，
    // 迭代借用无法与其并存。被改的都是其它顶点各自的依赖列表（另一张 map），
    // 本列表内容在循环期间不变，故快照与逐时刻迭代等价。
    let rdeps: Vec<BlockedConstraintId> = self.list(rev_deps).order.to_vec();
    for rdep in rdeps {
      // Remove vertex from the list of dependencies.
      let deps = self.find_dependency_list(rdep);
      self.list_mut(deps).remove(vertex.clone());
    }

    // Then clear this set.
    self.list_mut(rev_deps).clear();
  }
}

impl ConstraintGraph {
  pub fn copy_dependencies_of_type_id(&mut self, source: TypeId, target: TypeId) {
    let source_dependencies = self.find_dependency_list(BlockedConstraintId::V0(source));

    // C++ `copyDependenciesOf`：遍历 target 收集其内部的 free/blocked/PE 类型，
    // 使 target 的内层类型承接 source 原有的依赖边（此处不删除 source 自身）。
    let (mutated_types, mutated_type_packs) =
      collect_reachable_types(target, |rci, ty| rci.traverse_type_id(ty));

    self.copy_dependencies_to_reachable_types(
      None,
      source_dependencies,
      mutated_types,
      mutated_type_packs,
    );
  }
}

impl ConstraintGraph {
  pub fn copy_dependencies_to_reachable_types(
    &mut self,
    original_vertex: Option<BlockedConstraintId>,
    source_dependencies: SlotId,
    mutated_types: TypeIds,
    mutated_type_packs: TypePackIds,
  ) {
    // 源列表 `order` 先快照：循环内 `find_*` 可能追加新槽，迭代借用不能与写并存。
    // 上游 C++ 不变量（插入目标列表与源 order 不相交）未变，故等价。
    let vertices: Vec<BlockedConstraintId> = self.list(source_dependencies).order.to_vec();

    for vertex in vertices {
      let vertex_reverse_deps = self.find_reverse_dependency_list(vertex.clone());

      if let Some(ref original) = original_vertex {
        self.list_mut(vertex_reverse_deps).remove(original.clone());
      }

      for sub_target in mutated_types.order.iter() {
        let sub_target = BlockedConstraintId::V0(*sub_target);
        let ty_deps = self.find_dependency_list(sub_target.clone());

        self.list_mut(ty_deps).insert(vertex.clone());
        self.list_mut(vertex_reverse_deps).insert(sub_target);
      }

      for sub_pack_target in mutated_type_packs.iter() {
        let sub_pack_target = BlockedConstraintId::V1(*sub_pack_target);
        let tp_deps = self.find_dependency_list(sub_pack_target.clone());

        self.list_mut(tp_deps).insert(vertex.clone());
        self.list_mut(vertex_reverse_deps).insert(sub_pack_target);
      }
    }
  }
}

impl ConstraintGraph {
  pub fn dump(&mut self) {
    for (v, deps) in self.dependencies.iter() {
      // to_string 已全 safe：dependencies 的键全部经 add_dependency_of_* 插入，
      // TypeId/TypePackId 指向会话类型 arena 存活节点（bump 块不移动），
      // ConstraintId 由注册表保活；dump 是只读调试遍历，不失效任何句柄。
      let vstr = to_string(v.clone());
      // 槽号是 Copy，只读视图与上面的键借同为 `&self`，可并存。
      let deps = self.list(*deps);
      for d in deps.order.iter() {
        // The C++ `for (auto d : *deps)` iterates only present entries.
        if !deps.contains(d.clone()) {
          continue;
        }

        let mut line = String::new();
        dot_escape(&mut line, &vstr);
        line.push_str(" -> ");
        // d 是 deps 列表中登记的顶点句柄，与键 v 同源，只读格式化不改变存活性。
        let dstr = to_string(d.clone());
        dot_escape(&mut line, &dstr);
        std::println!("{}", line);
      }
    }
  }
}

impl ConstraintGraph {
  pub fn dump_blocked(&mut self, c: NonNull<Constraint>, opts: &mut ToStringOptions) {
    println!("Blocked on:");
    let c_ptr = c.as_ptr() as *const Constraint;
    let deps = self.find_dependency_list(BlockedConstraintId::V2(register_constraint(c_ptr)));
    let deps = self.list(deps);
    for dep in deps.order.iter() {
      // The C++ `for (auto dep : *deps)` iterates only present entries.
      if !deps.contains(dep.clone()) {
        continue;
      }

      dump_dependency_line(dep, "\t", opts);
    }
  }
}

/// `dumpBlocked`/`dumpWith` 两处同构的依赖行转储（cpp `dump` 内层循环直译）；
/// `indent` 区分两个调用面（"\t" vs "\t\t|\t"）。调用前须由 `contains` 过滤
/// 非存在项（C++ `for (auto dep : *deps)` 只迭代 present 条目）。
pub(crate) fn dump_dependency_line(
  dep: &BlockedConstraintId,
  indent: &str,
  opts: &mut ToStringOptions,
) {
  if let Some(ty) = dep.get_if_0() {
    println!(
      "{indent}Type {}",
      to_string_type_id_to_string_options(*ty, opts)
    );
  } else if let Some(tp) = dep.get_if_1() {
    println!(
      "{indent}Pack {}",
      to_string_type_pack_id_to_string_options(*tp, opts)
    );
  } else if let Some(cons) = dep.get_if_2() {
    // §2：V2 即句柄，读回节点走 resolve_constraint（只读格式化），
    // 越界/陈旧句柄至多缺一行转储（cpp 悬垂解引用同域，仅 dump 路径）。
    if let Some(node) = resolve_constraint(*cons) {
      println!(
        "{indent}Cons {}",
        to_string_constraint_to_string_options(node, opts)
      );
    }
  }
}

impl ConstraintGraph {
  pub fn dump_with(
    &mut self,
    unsolved_constraints: &[NonNull<Constraint>],
    opts: &mut ToStringOptions,
  ) {
    // TODO: It might be nice to *also* dump the types here.
    println!("constraints:");
    for c in unsolved_constraints.iter() {
      let c_ptr = c.as_ptr() as *const Constraint;
      let deps = self.find_dependency_list(BlockedConstraintId::V2(register_constraint(c_ptr)));
      let deps = self.list(deps);
      println!(
        "\t{}\t{}",
        deps.size(),
        // 入参 `&[NonNull<Constraint>]` 由求解会话保活，只读格式化（收口于 `alias_ref`）。
        to_string_constraint_to_string_options(alias_ref(c_ptr), opts)
      );

      for dep in deps.order.iter() {
        // The C++ `for (auto dep : *deps)` iterates only present entries.
        if !deps.contains(dep.clone()) {
          continue;
        }

        dump_dependency_line(dep, "\t\t|\t", opts);
      }
    }
  }
}

/// `findDependencyList`/`findReverseDependencyList` 同构体（cpp 两份模板式
/// 重复）：vertex 未登记时在槽池新建空 ConstraintList 并登记 `SlotId`。
fn find_or_create_list(
  map: &mut ConstraintMap,
  lists: &mut SlotArena<ConstraintList>,
  vertex: BlockedConstraintId,
) -> SlotId {
  if let Some(&deps) = map.find(&vertex) {
    return deps;
  }

  let newlist = lists.push(new_constraint_list());

  let (_it, fresh) = map.try_insert(vertex, newlist);
  LUAU_ASSERT!(fresh);
  newlist
}

impl ConstraintGraph {
  pub fn find_dependency_list(&mut self, vertex: BlockedConstraintId) -> SlotId {
    // 两 map 与列表存储是互不相交的字段，可同时可变借用。
    find_or_create_list(&mut self.dependencies, &mut self.constraint_lists, vertex)
  }
}

impl ConstraintGraph {
  pub fn find_reverse_dependency_list(&mut self, vertex: BlockedConstraintId) -> SlotId {
    // 两 map 与列表存储是互不相交的字段，可同时可变借用。
    find_or_create_list(
      &mut self.reverse_dependencies,
      &mut self.constraint_lists,
      vertex,
    )
  }
}

impl ConstraintGraph {
  pub fn has_strictly_more_than_one_dependency(&mut self, vertex: BlockedConstraintId) -> bool {
    let deps = self.find_dependency_list(vertex);
    self.list(deps).size() > 1
  }
}

impl ConstraintGraph {
  pub fn has_unsolved_dependencies(&mut self, vertex: BlockedConstraintId) -> bool {
    let deps = self.find_dependency_list(vertex.clone());

    if let Some(c) = vertex.get_if_2() {
      // §2：V2 分支即 `ConstraintId` 句柄，读回节点走 `resolve_constraint`
      // （指向本次判定期间存活、对齐良好的 `Constraint`）；本分支仅以只读
      // 借用做 `PrimitiveTypeConstraint::get_if` 的变体判定，不改写该节点。
      let Some(constraint) = resolve_constraint(*c) else {
        return self.list(deps).size() > 0;
      };
      if PrimitiveTypeConstraint::get_if(&constraint.c).is_some() {
        // 顶点为原语类型约束时，允许残留一个自身依赖（cpp 同条件）。
        return self.list(deps).size() > 1;
      }
    }

    self.list(deps).size() > 0
  }
}

impl ConstraintGraph {
  pub fn inherit_blocks(
    &mut self,
    existing_vertex: BlockedConstraintId,
    new_vertex: BlockedConstraintId,
  ) {
    let existing_reverse_deps = self.find_reverse_dependency_list(existing_vertex);
    let new_reverse_deps = self.find_reverse_dependency_list(new_vertex.clone());

    // `order` 先快照：循环内 `find_dependency_list` 会追加新槽。写目标与迭代源
    // 分属不同列表（vertex→列表为双射，new_vertex 是刚登记的新约束），故等价。
    let existing_rdeps: Vec<BlockedConstraintId> = self.list(existing_reverse_deps).order.to_vec();

    for existing_rdep in existing_rdeps {
      self
        .list_mut(new_reverse_deps)
        .insert(existing_rdep.clone());

      let new_deps = self.find_dependency_list(existing_rdep);
      self.list_mut(new_deps).insert(new_vertex.clone());
    }
  }
}

impl ConstraintGraph {
  pub fn repair_type_references_type_id(&mut self, mut ty: TypeId) {
    let root = follow_type::follow(ty);

    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
    let _ = seen.insert(root);

    while !seen.contains(&ty) {
      let _ = seen.insert(ty);
      self.shift_references_type_id(ty, root);

      if let Some(bt) = get_type::get::<BoundType>(ty) {
        ty = bt.bound_to;
      } else {
        break;
      }
    }
  }

  pub(crate) fn repair_type_references_type_pack_id(&mut self, mut ty: TypePackId) {
    let root = follow_type_pack::follow(ty);

    let mut seen: DenseHashSet<TypePackId> = DenseHashSet::default();
    let _ = seen.insert(root);

    while !seen.contains(&ty) {
      let _ = seen.insert(ty);
      self.shift_references_type_pack_id(ty, root);

      if let Some(bt) = get_type_pack::get::<BoundTypePack>(ty) {
        ty = bt.bound_to;
      } else {
        break;
      }
    }
  }
}

impl ConstraintGraph {
  pub fn shift_references_type_id(&mut self, source: TypeId, target: TypeId) {
    if source == target {
      return;
    }

    let source_dependencies = self.find_dependency_list(BlockedConstraintId::V0(source));

    // C++ `shiftReferences`：先遍历 target，收集其内部的 free/blocked/PE 类型，
    // 这些类型将承接 source 原有的依赖边。
    let (mutated_types, mutated_type_packs) =
      collect_reachable_types(target, |rci, ty| rci.traverse_type_id(ty));

    self.copy_dependencies_to_reachable_types(
      Some(BlockedConstraintId::V0(source)),
      source_dependencies,
      mutated_types,
      mutated_type_packs,
    );

    self.clear_reverse_dependencies_of(BlockedConstraintId::V0(source));
  }

  pub fn shift_references_type_pack_id(&mut self, source: TypePackId, target: TypePackId) {
    if source == target {
      return;
    }

    let source_dependencies = self.find_dependency_list(BlockedConstraintId::V1(source));

    let (mutated_types, mutated_type_packs) =
      collect_reachable_types(target, |rci, tp| rci.traverse_type_pack_id(tp));

    self.copy_dependencies_to_reachable_types(
      Some(BlockedConstraintId::V1(source)),
      source_dependencies,
      mutated_types,
      mutated_type_packs,
    );

    self.clear_reverse_dependencies_of(BlockedConstraintId::V1(source));
  }
}

impl ConstraintGraph {
  pub fn unblock_constraint(&mut self, c: NonNull<Constraint>) -> UnblockedTypes {
    let mut result = UnblockedTypes {
      types: TypeIds::new(),
      packs: TypePackIds::new(null_mut()),
    };

    // The reverse dependencies of this constraint should contain all of the types
    // and type packs that this constraint may mutate, either as a free type or
    // as a blocked type.
    let c_ptr = c.as_ptr() as *const Constraint;
    let c_vertex = BlockedConstraintId::V2(register_constraint(c_ptr));
    let reverse_deps = self.find_reverse_dependency_list(c_vertex.clone());

    // `order` 连同存在性一并快照（cpp 范围 for 只迭代 present 条目）：循环内
    // `find_dependency_list` 会追加新槽。各分支只改动 rdep 自身的依赖列表，
    // 本列表内容在循环期间不变，故快照与逐时刻迭代等价。
    let rdeps: Vec<BlockedConstraintId> = {
      let list = self.list(reverse_deps);
      list
        .order
        .iter()
        .filter(|rdep| list.contains((*rdep).clone()))
        .cloned()
        .collect()
    };

    for rdep in rdeps {
      if let Some(ty) = rdep.get_if_0() {
        let ty = *ty;
        result.types.insert_type_id(ty);
        let deps = self.find_dependency_list(BlockedConstraintId::V0(ty));
        self.list_mut(deps).remove(c_vertex.clone());
      } else if let Some(tp) = rdep.get_if_1() {
        let tp = *tp;
        let _ = result.packs.insert(tp);
        let deps = self.find_dependency_list(BlockedConstraintId::V1(tp));
        self.list_mut(deps).remove(c_vertex.clone());
      } else if let Some(dep_cons) = rdep.get_if_2() {
        let dep_cons = *dep_cons;
        let deps = self.find_dependency_list(BlockedConstraintId::V2(dep_cons));
        let deps = self.list_mut(deps);
        deps.remove(c_vertex.clone());
        if fflag::DebugLuauLogSolver.get() {
          let mut opts = ToStringOptions {
            exhaustive: true,
            ..Default::default()
          };
          // §2：dep_cons 即 V2 句柄，读回节点走 resolve_constraint；本分支仅在
          // DebugLuauLogSolver 开关下只读打印，越界/陈旧句柄至多缺该行日志。
          if let Some(node) = resolve_constraint(dep_cons) {
            println!(
              "Unblocking count={}\t{}",
              deps.size() as i32,
              to_string_constraint_to_string_options(node, &mut opts)
            );
          }
        }
      } else {
        LUAU_ASSERT!(false);
      }
    }

    /*
     * This whole song and dance is to repair the constraint graph after we
     * dispatch a constraint.
     *
     * We are assuming that, after a constraint has been dispatched, some
     * number of mutations have been made to the type graph. Importantly: if a
     * type has been mutated, then it was previously a reverse dependency of
     * [c]. If that is the case, then we can walk the reverse deps of [c] and
     * try to find bound types, shift their references over to their bounds,
     * and "repair" the dependency graph without having to track every single
     * [bind] call.
     */

    for ty in result.types.order.iter() {
      self.repair_type_references_type_id(*ty);
    }

    let packs: Vec<_> = result.packs.iter().copied().collect();
    for type_pack in packs {
      self.repair_type_references_type_pack_id(type_pack);
    }

    result
  }
}

impl ConstraintGraph {
  pub fn unblock_type_or_pack_type_id(&mut self, vertex: TypeId) {
    self.repair_type_references_type_id(vertex);
    let followed = follow_type::follow(vertex);
    self.clear_reverse_dependencies_of(BlockedConstraintId::V0(followed));
  }

  /// # Safety
  /// `vertex` 须为指向存活类型/类型包 arena 节点的有效 `TypePackId`；解阻塞过程会经 `self` 内部
  /// 的 `log`/依赖表裸指针（构造时接线自存活 `TxnLog`/builder，比本图长寿）读取并就地更新。
  /// 单线程独占约束图，无并发写。对应 C++ `void ConstraintGraph::unblockTypeOrPack(TypePackId vertex)`
  /// (`cpp/Analysis/src/ConstraintGraph.cpp:216`)。
  pub unsafe fn unblock_type_or_pack_type_pack_id(&mut self, vertex: TypePackId) {
    self.repair_type_references_type_pack_id(vertex);
    let vertex = follow_type_pack::follow(vertex);
    let _vertex = vertex;

    self.clear_reverse_dependencies_of(BlockedConstraintId::V2(ConstraintId::NULL));
  }
}
