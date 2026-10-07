//! `constraint` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  records::{constraint::Constraint, scope::Scope},
  type_aliases::constraint_v::ConstraintV,
};

// `Constraint` 的访问器收口：C++ 侧 `NotNull<Scope*>` 的裸指针解引用统一收进
// 这里，调用点不再出现 `unsafe { (*constraint).scope }` 样板（`location` 为
// 值语义字段，经 `&Constraint` 直接读即可，无需访问器）。

impl Constraint {
  /// C++ `NotNull<Scope*> Constraint::scope`：指向 solver 当前 scope 栈中的
  /// `Scope`，生命周期覆盖整个 solver 存续期，不会在 constraint 存活期间释放。
  pub fn scope_ref(&self) -> &Scope {
    // SAFETY: scope 由 ConstraintGenerator 以 NonNull 语义建立（C++ NotNull<Scope*> 同契约）。
    unsafe { &*self.scope }
  }
}

impl Constraint {
  /// C++ `Constraint::Constraint(NotNull<Scope> scope, const Location& location, ConstraintV&& c)`
  /// (Constraint.cpp:12-17).
  pub fn constraint_not_null_scope_location_constraint_v(
    scope: NonNull<Scope>,
    location: &Location,
    c: ConstraintV,
  ) -> Self {
    Constraint {
      scope: scope.as_ptr(),
      location: *location,
      c,
      deprecated_dependencies: Vec::new(),
    }
  }
}
