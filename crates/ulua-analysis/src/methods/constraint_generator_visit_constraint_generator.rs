use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass, ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
  },
  rtti::ast_node_as,
};
use ulua_common::{DFInt, FFlag, LUAU_ASSERT};

use crate::{
  enums::control_flow::ControlFlow,
  functions::does_call_error::does_call_error,
  records::{
    constraint_generator::ConstraintGenerator, recursion_counter::RecursionCounter,
    recursion_limiter::RecursionLimiter, scope::Scope,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat(
    &mut self,
    scope: &ScopePtr,
    stat: *mut AstStat,
  ) -> ControlFlow {
    unsafe {
      let _counter = RecursionCounter::recursion_counter_i32(&mut self.recursion_count);
      let _limiter: Option<RecursionLimiter> = None;

      if self.recursion_count >= DFInt::LuauConstraintGeneratorRecursionLimit.get() {
        self.report_code_too_complex((*stat).base.location);
        return ControlFlow::None;
      }

      // C++ takes `const ScopePtr&`; sibling visit methods diverge between
      // `*mut Scope` and `&ScopePtr` callee signatures, so adapt per callee.
      let scope_raw: *mut Scope = scope.as_ref() as *const Scope as *mut Scope;

      let node = stat as *mut AstNode;
      if (*node).is::<AstStatBlock>() {
        self.visit_scope_ptr_ast_stat_block(scope, stat as *mut AstStatBlock)
      } else if (*node).is::<AstStatIf>() {
        self.visit_scope_ptr_ast_stat_if(scope, stat as *mut AstStatIf)
      } else if (*node).is::<AstStatWhile>() {
        self.visit_scope_ptr_ast_stat_while(scope, stat as *mut AstStatWhile)
      } else if (*node).is::<AstStatRepeat>() {
        self.visit_scope_ptr_ast_stat_repeat(scope, stat as *mut AstStatRepeat)
      } else if (*node).is::<AstStatBreak>() {
        ControlFlow::Breaks
      } else if (*node).is::<AstStatContinue>() {
        ControlFlow::Continues
      } else if (*node).is::<AstStatReturn>() {
        self.visit_scope_ptr_ast_stat_return(scope.clone(), stat as *mut AstStatReturn)
      } else if (*node).is::<AstStatExpr>() {
        let expr_stat = stat as *mut AstStatExpr;
        self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
          scope,
          (*expr_stat).expr,
          &Vec::new(),
          true,
        );
        let call = ast_node_as::<AstExprCall>((*expr_stat).expr as *mut AstNode);
        if !call.is_null() && does_call_error(&*call) {
          return ControlFlow::Throws;
        }
        ControlFlow::None
      } else if (*node).is::<AstStatLocal>() {
        self.visit_scope_ptr_ast_stat_local(scope, stat as *mut AstStatLocal)
      } else if (*node).is::<AstStatFor>() {
        self.visit_scope_ptr_ast_stat_for(scope, stat as *mut AstStatFor)
      } else if (*node).is::<AstStatForIn>() {
        self.visit_scope_ptr_ast_stat_for_in(scope, stat as *mut AstStatForIn)
      } else if (*node).is::<AstStatAssign>() {
        self.visit_scope_ptr_ast_stat_assign(scope, stat as *mut AstStatAssign)
      } else if (*node).is::<AstStatCompoundAssign>() {
        self.visit_scope_ptr_ast_stat_compound_assign(scope, stat as *mut AstStatCompoundAssign)
      } else if (*node).is::<AstStatFunction>() {
        self.visit_scope_ptr_ast_stat_function(scope, stat as *mut AstStatFunction)
      } else if (*node).is::<AstStatLocalFunction>() {
        self.visit_scope_ptr_ast_stat_local_function(scope, stat as *mut AstStatLocalFunction)
      } else if (*node).is::<AstStatTypeAlias>() {
        self.visit_scope_ptr_ast_stat_type_alias(scope, stat as *mut AstStatTypeAlias)
      } else if (*node).is::<AstStatTypeFunction>() {
        self.visit_scope_ptr_ast_stat_type_function(scope, stat as *mut AstStatTypeFunction)
      } else if (*node).is::<AstStatDeclareGlobal>() {
        self.visit_scope_ptr_ast_stat_declare_global(scope_raw, stat as *mut AstStatDeclareGlobal)
      } else if (*node).is::<AstStatDeclareFunction>() {
        self.visit_scope_ptr_ast_stat_declare_function(scope, stat as *mut AstStatDeclareFunction)
      } else if (*node).is::<AstStatDeclareExternType>() {
        self.visit_scope_ptr_ast_stat_declare_extern_type(
          scope_raw,
          stat as *mut AstStatDeclareExternType,
        )
      } else if (*node).is::<AstStatClass>() {
        LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());
        self.visit_scope_ptr_ast_stat_class(scope, stat as *mut AstStatClass)
      } else if (*node).is::<AstStatError>() {
        self.visit_scope_ptr_ast_stat_error(scope, stat as *mut AstStatError)
      } else {
        LUAU_ASSERT!(false, "Internal error: Unknown AstStat type");
        ControlFlow::None
      }
    }
  }
}
