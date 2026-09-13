use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::FInt;

use crate::{
  enums::control_flow::ControlFlow,
  records::{recursion_counter::RecursionCounter, type_checker::TypeChecker},
  type_aliases::scope_ptr_type::ScopePtr,
};

impl TypeChecker {
  pub fn check_block(&mut self, scope: &ScopePtr, block: &AstStatBlock) -> ControlFlow {
    let _rc = unsafe { RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count) };
    let limit = FInt::LuauCheckRecursionLimit.get();
    if limit > 0 && self.check_recursion_count >= limit {
      self.report_error_code_too_complex(&block.base.base.location);
      return ControlFlow::None;
    }

    self.check_block_without_recursion_check(scope, block)
  }
}
