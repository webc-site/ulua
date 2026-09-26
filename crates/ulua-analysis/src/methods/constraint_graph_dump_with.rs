use core::ptr::NonNull;

use crate::{
  functions::to_string_to_string::{
    to_string_constraint_to_string_options, to_string_type_id_to_string_options,
    to_string_type_pack_id_to_string_options,
  },
  records::{
    blocked_constraint_registry::{register_constraint, resolve_constraint},
    constraint::Constraint,
    constraint_graph::ConstraintGraph,
    to_string_options::ToStringOptions,
  },
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

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
      // Safety: `deps` 是 `NonNull<ConstraintList>`，其非空性由 NonNull 不变量保证；它指向
      // 由 `find_dependency_list` 从本图存储中取回、在本次迭代结束前保持有效的依赖列表节点
      // （其间只读图、不再触发新的图分配），重建共享借用仅读取 size/order。
      let deps_ref = unsafe { deps.as_ref() };
      println!(
        "\t{}\t{}",
        deps_ref.size(),
        // Safety: `c_ptr` 取自入参 `unsolved_constraints: &[NonNull<Constraint>]` 的
        // `as_ptr()`，每个 NonNull 非空且指向求解会话持有的存活 `Constraint`；重建共享借用
        // 只供 `to_string_constraint_to_string_options` 只读格式化。
        to_string_constraint_to_string_options(unsafe { &*c_ptr }, opts)
      );

      for dep in deps_ref.order.iter() {
        // The C++ `for (auto dep : *deps)` iterates only present entries.
        if !deps_ref.contains(dep.clone()) {
          continue;
        }

        if let Some(ty) = dep.get_if_0() {
          println!(
            "\t\t|\tType {}",
            to_string_type_id_to_string_options(*ty, opts)
          );
        } else if let Some(tp) = dep.get_if_1() {
          println!(
            "\t\t|\tPack {}",
            to_string_type_pack_id_to_string_options(*tp, opts)
          );
        } else if let Some(cons) = dep.get_if_2() {
          // §2：V2 即句柄，读回节点走 resolve_constraint（只读格式化），
          // 越界/陈旧句柄至多缺一行转储（cpp 悬垂解引用同域，仅 dump 路径）。
          if let Some(node) = resolve_constraint(*cons) {
            println!(
              "\t\t|\tCons {}",
              to_string_constraint_to_string_options(node, opts)
            );
          }
        }
      }
    }
  }
}
