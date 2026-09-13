use core::ptr::NonNull;

use crate::{
  functions::{follow_type::follow_type_id, get_mutable_type::get_mutable_type_id},
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    infinite_type_finder::InfiniteTypeFinder, instantiation_signature::InstantiationSignature,
    iterative_type_visitor::IterativeTypeVisitorTrait, metatable_type::MetatableType,
    name_constraint::NameConstraint, table_type::TableType,
  },
  type_aliases::name_type::Name,
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_name_constraint_not_null_constraint(
    &mut self,
    c: &NameConstraint,
    constraint: *const Constraint,
  ) -> bool {
    if self.is_blocked_type_id(c.named_type) {
      return self.block_type_id_not_null_constraint(c.named_type, constraint);
    }

    let target = follow_type_id(c.named_type);

    unsafe {
      if (*target).persistent || (*target).owning_arena != self.arena {
        return true;
      }
    }

    if let Some(tf) = unsafe { (*(*constraint).scope).lookup_type(&Name::from(c.name.as_str())) } {
      let signature = InstantiationSignature {
        fn_sig: tf,
        arguments: c.type_parameters.clone(),
        pack_arguments: c.type_pack_parameters.clone(),
      };

      let mut itf =
        InfiniteTypeFinder::infinite_type_finder_infinite_type_finder(self, &signature, unsafe {
          NonNull::new_unchecked((*constraint).scope)
        });
      itf.run_type_id(target);

      if itf.found_infinite_type {
        unsafe {
          (*(*constraint).scope)
            .invalid_type_aliases
            .try_insert(c.name.clone(), (*constraint).location)
        };
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            target,
            (*self.builtin_types).error_type,
          )
        };
        return true;
      }
    }

    if let Some(ttv) = get_mutable_type_id::<TableType>(target) {
      if c.synthetic && ttv.name.is_none() {
        ttv.synthetic_name = Some(c.name.clone());
      } else {
        ttv.name = Some(c.name.clone());
        ttv.instantiated_type_params = c.type_parameters.clone();
        ttv.instantiated_type_pack_params = c.type_pack_parameters.clone();
      }
    } else if let Some(mtv) = get_mutable_type_id::<MetatableType>(target) {
      mtv.synthetic_name = Some(c.name.clone());
    }

    true
  }
}
