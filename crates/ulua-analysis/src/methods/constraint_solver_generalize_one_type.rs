use ulua_common::fflag;

use crate::{
  functions::{
    follow_type,
    generalize::generalize,
    get_type,
    to_string_to_string::{to_string_type_id, to_string_type_id_to_string_options_mut},
  },
  records::{constraint_solver::ConstraintSolver, free_type::FreeType},
  type_aliases::type_id::TypeId,
};

impl ConstraintSolver {
  pub fn generalize_one_type(&mut self, ty: TypeId) {
    let ty = follow_type::follow(ty);
    let free_ty = get_type::get::<FreeType>(ty);

    let saveme = if fflag::DebugLuauLogSolver.get() {
      to_string_type_id_to_string_options_mut(ty, self.opts.clone())
    } else {
      "[FFlag::DebugLuauLogSolver Off]".to_string()
    };

    let Some(free_ty) = free_ty else {
      return;
    };

    let function_type = self.constraint_set.scope_to_function.find(&free_ty.scope);

    if let Some(function_type) = function_type {
      let result_ty = generalize(
        self.arena,
        self.builtin_types,
        free_ty.scope,
        &self.generalized_types_ as *const _ as *mut _,
        *function_type,
        Some(ty),
      );

      if fflag::DebugLuauLogSolver.get() {
        let current_ty_str = to_string_type_id(ty);
        let result_ty_str = result_ty
          .map(to_string_type_id)
          .unwrap_or_else(|| to_string_type_id(*function_type));

        println!(
          "Eagerly generalized {} (now {})\n\tin function {}",
          saveme, current_ty_str, result_ty_str
        );
      }
    }
  }
}
