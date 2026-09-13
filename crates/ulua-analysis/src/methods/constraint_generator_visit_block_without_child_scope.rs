// ConstraintGenerator::visitBlockWithoutChildScope (ConstraintGenerator.cpp:1276-1297).
use alloc::sync::Arc;
use core::mem::ManuallyDrop;

use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::DFInt;

use crate::{
  enums::control_flow::ControlFlow,
  records::{
    constraint_generator::ConstraintGenerator, recursion_counter::RecursionCounter, scope::Scope,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_block_without_child_scope(
    &mut self,
    scope: *mut Scope,
    block: *mut AstStatBlock,
  ) -> ControlFlow {
    let _counter = unsafe { RecursionCounter::recursion_counter_i32(&mut self.recursion_count) };

    if self.recursion_count >= DFInt::LuauConstraintGeneratorRecursionLimit.get() {
      self.report_code_too_complex(unsafe { (*block).base.base.location });
      return ControlFlow::None;
    }

    unsafe { self.prototype_type_definitions(scope, block) };

    // Borrow the caller-owned `Scope` as a `ScopePtr` without taking ownership
    // (C++ passes `const ScopePtr&`); ManuallyDrop keeps the refcount intact.
    let scope_ptr: ManuallyDrop<ScopePtr> =
      ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });

    let mut first_control_flow: Option<ControlFlow> = None;
    let body = unsafe { (*block).body };
    for &stat in body.as_slice() {
      let cf = unsafe { self.visit_scope_ptr_ast_stat(&scope_ptr, stat) };
      if cf != ControlFlow::None && first_control_flow.is_none() {
        first_control_flow = Some(cf);
      }
    }

    first_control_flow.unwrap_or(ControlFlow::None)
  }
}
