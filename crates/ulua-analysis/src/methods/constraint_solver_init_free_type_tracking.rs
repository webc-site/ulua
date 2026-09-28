use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::to_string_to_string::{
    to_string_constraint_to_string_options, to_string_type_id_to_string_options,
    to_string_type_pack_id_to_string_options,
  },
  records::{
    blocked_constraint_registry::register_constraint, constraint::Constraint,
    constraint_solver::ConstraintSolver,
  },
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};
impl ConstraintSolver {
  pub fn init_free_type_tracking(&mut self) {
    if fflag::LuauConstraintGraph.get() {
      for c in &self.constraints {
        // Safety: constraints 元素是构造期入队的非空裸句柄（NotNull 语义），指向
        // solver_constraints 中 Box<Constraint>——Box 堆址稳定且活过本初始化循环
        // （初始化期无人移除），`&**c` 仅取共享只读借用；push 的是同一地址的
        // Copy 身份键。
        let borrow = unsafe { &**c };
        self.unsolved_constraints.push(borrow as *const Constraint);

        let (types, type_packs) = borrow.get_maybe_mutated_types();

        for ty in types.order.iter() {
          // Safety: cgraph 为构造期 SolverParams 注入的非空 ConstraintGraph
          // （LuauConstraintGraph 开启期布线不变量），图与 solver 同生命周期；
          // 两个 BlockedConstraintId 仅按 Copy 句柄/裸指针身份入图建边，
          // 不经其解引用。
          unsafe {
            (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
              BlockedConstraintId::V2(register_constraint(borrow as *const Constraint)),
              BlockedConstraintId::V0(*ty),
            )
          };
          if fflag::DebugLuauLogSolver.get() {
            let ty_str = to_string_type_id_to_string_options(*ty, &mut self.opts.clone());
            let c_str = to_string_constraint_to_string_options(borrow, &mut self.opts.clone());
            println!("Type {} depends on constraint {}", ty_str, c_str);
          }
        }

        for tp in type_packs.iter() {
          // Safety: 同 ty 侧——cgraph 构造期布线非空、图与 solver 同寿，
          // BlockedConstraintId 仅作图顶点身份键，不被解引用。
          unsafe {
            (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
              BlockedConstraintId::V2(register_constraint(borrow as *const Constraint)),
              BlockedConstraintId::V1(*tp),
            )
          };
          if fflag::DebugLuauLogSolver.get() {
            let tp_str = to_string_type_pack_id_to_string_options(*tp, &mut self.opts.clone());
            let c_str = to_string_constraint_to_string_options(borrow, &mut self.opts.clone());
            println!("Type pack {} depends on constraint {}", tp_str, c_str);
          }
        }
      }
    } else {
      let constraints = self.constraints.clone();
      for c in &constraints {
        // Safety: c 是克隆自 constraints 的同一非空裸句柄（NotNull 语义），指向
        // solver_constraints 中 Box 持有、地址稳定的存活 Constraint；克隆的只是
        // 指针值，节点仍由 solver 独占持有并活过本循环，`&**c` 共享借用有效。
        let borrow = unsafe { &**c };
        self.unsolved_constraints.push(borrow as *const Constraint);

        let (types, _type_packs) = borrow.get_maybe_mutated_types();

        for ty in types.order.iter() {
          let key = *ty;
          let entry = self
            .deprecated_type_to_constraint_set
            .entry(key)
            .or_default();
          // We don't care if this is fresh, we can blindly insert.
          entry.insert(borrow as *const Constraint);
        }

        let (_types, fresh1) = self
          .deprecated_constraint_to_mutated_types
          .try_insert(borrow as *const Constraint, types);
        LUAU_ASSERT!(fresh1);

        for dep in &borrow.deprecated_dependencies {
          self.block_not_null_constraint_not_null_constraint(*dep, borrow as *const Constraint);
        }
      }
    }
  }
}
