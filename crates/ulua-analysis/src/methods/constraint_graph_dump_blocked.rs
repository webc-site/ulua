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
  pub fn dump_blocked(&mut self, c: NonNull<Constraint>, opts: &mut ToStringOptions) {
    println!("Blocked on:");
    let c_ptr = c.as_ptr() as *const Constraint;
    let deps = self.find_dependency_list(BlockedConstraintId::V2(register_constraint(c_ptr)));
    let deps_ref = unsafe { deps.as_ref() };
    for dep in deps_ref.order.iter() {
      // The C++ `for (auto dep : *deps)` iterates only present entries.
      if !deps_ref.contains(dep.clone()) {
        continue;
      }

      if let Some(ty) = dep.get_if_0() {
        println!("\tType {}", to_string_type_id_to_string_options(*ty, opts));
      } else if let Some(tp) = dep.get_if_1() {
        println!(
          "\tPack {}",
          to_string_type_pack_id_to_string_options(*tp, opts)
        );
      } else if let Some(cons) = dep.get_if_2() {
        // §2：V2 即句柄，读回节点走 resolve_constraint；越界/陈旧句柄至多
        // 缺一行转储（cpp 悬垂解引用同域，仅 debug dump 路径）。
        if let Some(node) = resolve_constraint(*cons) {
          println!(
            "\tCons {}",
            to_string_constraint_to_string_options(node, opts)
          );
        }
      }
    }
  }
}
