use alloc::vec::Vec;
use core::ptr::{NonNull, null_mut};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::to_string_to_string::to_string_constraint_to_string_options,
  records::{
    blocked_constraint_registry::{register_constraint, resolve_constraint},
    constraint::Constraint,
    constraint_graph::ConstraintGraph,
    to_string_options::ToStringOptions,
    type_ids::TypeIds,
    unblocked_types::UnblockedTypes,
  },
  type_aliases::{blocked_constraint_id::BlockedConstraintId, type_pack_ids::TypePackIds},
};
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
    // Safety: reverse_deps 为 find_reverse_dependency_list 返回的 NonNull<ConstraintList>（NonNull
    // 已保证非空对齐），指向 self.constraint_lists（PinnedStorage<Vec<Box<T>>>）中地址稳定的存活节点；
    // 本次 unblock 期间 self 借用存续、push 新节点不移动已有 Box 内容，故只读借用全程有效且无并存 &mut。
    let reverse_deps_ref = unsafe { reverse_deps.as_ref() };

    for rdep in reverse_deps_ref.order.iter() {
      // The C++ `for (auto rdep : *reverseDeps)` iterates only present entries.
      if !reverse_deps_ref.contains(rdep.clone()) {
        continue;
      }

      if let Some(ty) = rdep.get_if_0() {
        let ty = *ty;
        result.types.insert_type_id(ty);
        let deps = self.find_dependency_list(BlockedConstraintId::V0(ty));
        // Safety: deps 为 find_dependency_list 返回的 NonNull<ConstraintList>，as_ptr 给出该顶点在
        // PinnedStorage 中地址稳定的节点（后续 push 不移动已有 Box）；remove 需独占借用，此节点与
        // reverse_deps_ref 指向不同的 ConstraintList，单线程遍历无重叠别名。
        let deps_mut = unsafe { &mut *deps.as_ptr() };
        deps_mut.remove(c_vertex.clone());
      } else if let Some(tp) = rdep.get_if_1() {
        let tp = *tp;
        let _ = result.packs.insert(tp);
        let deps = self.find_dependency_list(BlockedConstraintId::V1(tp));
        // Safety: 同 V0 分支——deps 为 PinnedStorage 中该类型包顶点对应的地址稳定节点，remove 的
        // 独占借用与 reverse_deps_ref 指向不同对象，单线程无重叠别名。
        let deps_mut = unsafe { &mut *deps.as_ptr() };
        deps_mut.remove(c_vertex.clone());
      } else if let Some(dep_cons) = rdep.get_if_2() {
        let dep_cons = *dep_cons;
        let deps = self.find_dependency_list(BlockedConstraintId::V2(dep_cons));
        // Safety: 同上——deps 为该约束顶点在 PinnedStorage 中地址稳定的节点，remove 独占借用与
        // reverse_deps_ref 指向不同对象，单线程无重叠别名。
        let deps_mut = unsafe { &mut *deps.as_ptr() };
        deps_mut.remove(c_vertex.clone());
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
              deps_mut.size() as i32,
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
