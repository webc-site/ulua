//! C++ `NotNull<Constraint> TypeFunctionContext::pushConstraint(ConstraintV&& c)
//! const` (BuiltinTypeFunctions.cpp:376-387). Forwards to the solver and, when a
//! current constraint exists, inherits its blocks onto the new constraint.
use core::ptr::NonNull;

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{constraint::Constraint, type_function_context::TypeFunctionContext},
  type_aliases::constraint_v::ConstraintV,
};

impl TypeFunctionContext {
  pub fn push_constraint(&self, c: ConstraintV) -> NonNull<Constraint> {
    LUAU_ASSERT!(!self.solver.is_null());

    let location = if !self.constraint.is_null() {
      unsafe { (*self.constraint).location }
    } else {
      Location::new(
        Position { line: 0, column: 0 },
        Position { line: 0, column: 0 },
      )
    };

    let new_constraint = unsafe { (*self.solver).push_constraint(self.scope, location, c) };

    // Every constraint that is blocked on the current constraint must also be
    // blocked on this new one.
    if !self.constraint.is_null() {
      unsafe {
        (*self.solver).inherit_blocks(
          self.constraint,
          new_constraint.as_ptr() as *const Constraint,
        );
      }
    }

    new_constraint
  }
}
