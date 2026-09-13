use alloc::{sync::Arc, vec::Vec};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::{DFInt, LUAU_ASSERT};

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, inference_pack::InferencePack, module::Module,
    recursion_counter::RecursionCounter,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExpr,
    expected_types: &[Option<TypeId>],
    generalize: bool,
  ) -> InferencePack {
    let _counter = unsafe { RecursionCounter::recursion_counter_i32(&mut self.recursion_count) };

    if self.recursion_count >= DFInt::LuauConstraintGeneratorRecursionLimit.get() {
      self.report_code_too_complex(unsafe { (*expr).base.location });
      return InferencePack {
        tp: unsafe { (*self.builtin_types).error_type_pack },
        refinements: Vec::new(),
      };
    }

    let result: InferencePack;

    let node = expr as *mut AstNode;
    let call = unsafe { ast_node_as::<AstExprCall>(node) };
    if !call.is_null() {
      result = unsafe { self.check_pack_scope_ptr_ast_expr_call(scope, call) };
    } else if unsafe { (*node).is::<AstExprVarargs>() } {
      if let Some(vararg_pack) = scope.as_ref().vararg_pack {
        result = InferencePack {
          tp: vararg_pack,
          refinements: Vec::new(),
        };
      } else {
        result = InferencePack {
          tp: unsafe { (*self.builtin_types).error_type_pack },
          refinements: Vec::new(),
        };
      }
    } else {
      let mut expected_type: Option<TypeId> = None;
      if !expected_types.is_empty() {
        expected_type = expected_types[0];
      }
      let t: TypeId = self
        .check_scope_ptr_ast_expr_optional_type_id_bool_bool(
          scope,
          expr,
          expected_type,
          false,
          generalize,
        )
        .ty;
      result = InferencePack {
        tp: unsafe { (*self.arena).add_type_pack_initializer_list_type_id(&[t]) },
        refinements: Vec::new(),
      };
    }

    LUAU_ASSERT!(!result.tp.is_null());
    if let Some(module) = &self.module {
      let module_ptr = Arc::as_ptr(module) as *mut Module;
      unsafe {
        *(*module_ptr)
          .ast_type_packs
          .get_or_insert(expr as *const AstExpr) = result.tp;
      }
    }
    result
  }
}
