use core::ptr::NonNull;

use crate::{
  functions::{
    to_string_to_string_alt_m::to_string_type_id_to_string_options,
    to_string_to_string_alt_n::to_string_type_pack_id_to_string_options,
    to_string_to_string_alt_q::to_string_constraint_to_string_options,
  },
  records::{
    constraint::Constraint, constraint_graph::ConstraintGraph, to_string_options::ToStringOptions,
  },
  type_aliases::constraint_vertex::ConstraintVertex,
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
      let deps = self.find_dependency_list(ConstraintVertex::V2(c_ptr));
      let deps_ref = unsafe { deps.as_ref() };
      println!(
        "\t{}\t{}",
        deps_ref.size(),
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
          println!(
            "\t\t|\tCons {}",
            to_string_constraint_to_string_options(unsafe { &**cons }, opts)
          );
        }
      }
    }
  }
}
