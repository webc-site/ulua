// ConstraintGenerator::visitBlockWithoutChildScope (ConstraintGenerator.cpp:1276-1297).
use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::dfint;

use crate::{
  enums::control_flow::ControlFlow,
  records::{constraint_generator::ConstraintGenerator, recursion_counter::RecursionCounter},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// C++ `visitBlockWithoutChildScope(const ScopePtr& scope, AstStatBlock* block)`：
  /// `scope` 为调用方持有的存活 `ScopePtr` 共享借用（cpp `const ScopePtr&` 的
  /// Rust 对应，不再经裸指针重建 Arc）；`block` 为解析器 arena 存活的
  /// `AstStatBlock` 共享借用。
  pub fn visit_block_without_child_scope(
    &mut self,
    scope: &ScopePtr,
    block: &AstStatBlock,
  ) -> ControlFlow {
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.recursion_count);

    if self.recursion_count >= dfint::LuauConstraintGeneratorRecursionLimit.get() {
      self.report_code_too_complex(block.base.base.location);
      return ControlFlow::None;
    }

    self.prototype_type_definitions(scope, block);

    let mut first_control_flow: Option<ControlFlow> = None;
    for stat in block.body.iter() {
      let cf = self.visit_stat(scope, stat);
      if cf != ControlFlow::None && first_control_flow.is_none() {
        first_control_flow = Some(cf);
      }
    }

    first_control_flow.unwrap_or(ControlFlow::None)
  }
}
