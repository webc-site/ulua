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
      // Safety: 分支守卫已确认 `self.constraint` 非空；它指向驱动本类型函数求值的当前
      // `Constraint`（solver 持有、在本调用期内存活），仅只读复制其 `Location` 字段，无别名冲突。
      unsafe { (*self.constraint).location }
    } else {
      Location::new(
        Position { line: 0, column: 0 },
        Position { line: 0, column: 0 },
      )
    };

    // Safety: 首行 `LUAU_ASSERT!(!self.solver.is_null())` 保证 solver 裸指针非空；solver 是
    // 构造期接线的 ConstraintSolver，比本 context 长寿。`push_constraint` 为 `&mut self`，
    // 在单线程求值循环中此刻无人持有对同一 solver 的其它借用，重建独占引用不产生别名冲突。
    let new_constraint = unsafe { (*self.solver).push_constraint(self.scope, location, c) };

    // Every constraint that is blocked on the current constraint must also be
    // blocked on this new one.
    if !self.constraint.is_null() {
      // Safety: solver 同上仍非空且本线程独占（`push_constraint` 借用已在此处结束）。
      // `self.constraint` 由外层守卫确认非空，`new_constraint` 是刚由 solver 构造并保活的
      // `NonNull`，二者作为 `*const Constraint` 传入 `inherit_blocks(&mut self)` 只读引用。
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
