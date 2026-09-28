//! `NotNull<Constraint> ConstraintSolver::pushConstraint(NotNull<Scope> scope, const Location& location, ConstraintV cv)`
//! (`Analysis/src/ConstraintSolver.cpp:4094-4126`, hand-ported faithfully).

use alloc::boxed::Box;
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  enums::subtyping_variance::SubtypingVariance,
  records::{
    blocked_constraint_registry::register_constraint, code_too_complex::CodeTooComplex,
    constraint::Constraint, constraint_solver::ConstraintSolver,
    equality_constraint::EqualityConstraint, scope::Scope, subtype_constraint::SubtypeConstraint,
    subtype_constraint_record::SubtypeConstraintRecord,
  },
  type_aliases::constraint_v::{ConstraintV, ConstraintVMember},
};

impl ConstraintSolver {
  pub fn push_constraint(
    &mut self,
    scope: NonNull<Scope>,
    location: Location,
    cv: ConstraintV,
  ) -> NonNull<Constraint> {
    let mut scr: Option<SubtypeConstraintRecord> = None;
    if let Some(sc) = SubtypeConstraint::get_if(&cv) {
      scr = Some(SubtypeConstraintRecord {
        sub_ty: sc.sub_type,
        super_ty: sc.super_type,
        variance: SubtypingVariance::Covariant,
      });
    } else if let Some(ec) = EqualityConstraint::get_if(&cv) {
      scr = Some(SubtypeConstraintRecord {
        sub_ty: ec.assignment_type,
        super_ty: ec.result_type,
        variance: SubtypingVariance::Invariant,
      });
    }

    if let Some(record) = scr
      && let Some(f) = self.seen_constraints.find(&record)
    {
      // Safety: seen_constraints 值由本 solver 以刚建 Box 的堆地址登记，恒非空。
      return NonNull::new(*f).expect("seen 表值由 Box 堆地址登记，恒非空");
    }

    let mut c: Box<Constraint> = Box::new(
      Constraint::constraint_not_null_scope_location_constraint_v(scope, &location, cv),
    );
    // Box 堆上内容与 Box 结构体分离，push 移动 Box 不影响该指针有效性。
    let borrow: *mut Constraint = &mut *c;
    // §2 收口：Constraint 唯一分配点在此，立即向 blocked_constraint_registry
    // 登记堆地址换取 ConstraintId 句柄（幂等 find-or-insert），此后
    // BlockedConstraintId::V2 仅以句柄形态流转，裸指针不再外泄。
    register_constraint(borrow as *const Constraint);

    if let Some(record) = scr {
      *self.seen_constraints.get_or_insert(record) = borrow;
    }

    self.solver_constraints.push(c);
    self.unsolved_constraints.push(borrow as *const Constraint);

    if self.solver_constraint_limit > 0 {
      self.solver_constraint_limit -= 1;

      if self.solver_constraint_limit == 0 {
        self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
      }
    }

    // Safety: borrow 取自刚构造 Box 的 &mut 解引用，天然非空。
    NonNull::new(borrow).expect("borrow 源自 &mut *c 解引用，恒非空")
  }
}
