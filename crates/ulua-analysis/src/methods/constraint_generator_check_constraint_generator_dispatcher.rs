//! Hand-ported dispatcher: ConstraintGenerator::check(const ScopePtr&, AstExpr*, ...)
//! Source: `Analysis/src/ConstraintGenerator.cpp:3042`.
//!
//! The top-level expression `check` overload. C++ overloads `check` on the
//! static type of the expression; here we recover that with RTTI dispatch to
//! the per-node `check_scope_ptr_ast_expr_*` methods. Two convenience entry
//! points are provided to mirror the defaulted C++ arguments
//! (`expected_type = {}`, `forceSingleton = false`, `generalize = true`).
use alloc::{sync::Arc, vec::Vec};
use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_table::AstExprTable, ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::AstExprUnary, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::{DFInt, LUAU_ASSERT};

use crate::{
  records::{constraint_generator::ConstraintGenerator, inference::Inference, module::Module},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// Convenience: `check(scope, expr)` with all C++ defaults.
  pub fn check_scope_ptr_ast_expr(&mut self, scope: &ScopePtr, expr: *mut AstExpr) -> Inference {
    self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(scope, expr, None, false, true)
  }

  /// Convenience: `check(scope, expr, expected_type)`.
  pub fn check_scope_ptr_ast_expr_optional_type_id(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExpr,
    expected_type: Option<TypeId>,
  ) -> Inference {
    self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(
      scope,
      expr,
      expected_type,
      false,
      true,
    )
  }

  /// Convenience: `check(scope, expr, expected_type, forceSingleton)`.
  pub fn check_scope_ptr_ast_expr_optional_type_id_bool(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExpr,
    expected_type: Option<TypeId>,
    force_singleton: bool,
  ) -> Inference {
    self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(
      scope,
      expr,
      expected_type,
      force_singleton,
      true,
    )
  }

  pub fn check_scope_ptr_ast_expr_optional_type_id_bool_bool(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExpr,
    expected_type: Option<TypeId>,
    force_singleton: bool,
    generalize: bool,
  ) -> Inference {
    // RecursionCounter counter{&recursionCount};
    self.recursion_count += 1;
    let result = self.check_dispatch_impl(scope, expr, expected_type, force_singleton, generalize);
    self.recursion_count -= 1;
    result
  }

  fn check_dispatch_impl(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExpr,
    expected_type: Option<TypeId>,
    force_singleton: bool,
    generalize: bool,
  ) -> Inference {
    unsafe {
      if self.recursion_count >= DFInt::LuauConstraintGeneratorRecursionLimit.get() {
        self.report_code_too_complex((*expr).base.location);
        return Inference::inference_type_id_refinement_id(
          (*self.builtin_types).error_type,
          null_mut(),
        );
      }

      // We may recurse a given expression more than once when checking
      // compound assignment, so we store and cache expressions here.
      if self.inferred_expr_cache.contains(&expr) {
        return self.inferred_expr_cache.get_or_insert(expr).clone();
      }

      let node = expr as *mut AstNode;

      let result: Inference = {
        let group = ast_node_as::<AstExprGroup>(node);
        if !group.is_null() {
          self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(
            scope,
            (*group).expr,
            expected_type,
            force_singleton,
            generalize,
          )
        } else if !ast_node_as::<AstExprConstantString>(node).is_null() {
          let string_expr = ast_node_as::<AstExprConstantString>(node);
          self.check_scope_ptr_ast_expr_constant_string_optional_type_id_bool(
            scope,
            &*string_expr,
            expected_type,
            force_singleton,
          )
        } else if !ast_node_as::<AstExprConstantNumber>(node).is_null() {
          Inference::inference_type_id_refinement_id((*self.builtin_types).number_type, null_mut())
        } else if !ast_node_as::<AstExprConstantInteger>(node).is_null() {
          Inference::inference_type_id_refinement_id((*self.builtin_types).integer_type, null_mut())
        } else if !ast_node_as::<AstExprConstantBool>(node).is_null() {
          let bool_expr = ast_node_as::<AstExprConstantBool>(node);
          self.check_scope_ptr_ast_expr_constant_bool_optional_type_id_bool(
            scope,
            &*bool_expr,
            expected_type,
            force_singleton,
          )
        } else if !ast_node_as::<AstExprConstantNil>(node).is_null() {
          Inference::inference_type_id_refinement_id((*self.builtin_types).nil_type, null_mut())
        } else if !ast_node_as::<AstExprLocal>(node).is_null() {
          let local = ast_node_as::<AstExprLocal>(node);
          self.check_scope_ptr_ast_expr_local(scope, local)
        } else if !ast_node_as::<AstExprGlobal>(node).is_null() {
          let global = ast_node_as::<AstExprGlobal>(node);
          self.check_scope_ptr_ast_expr_global(scope, global)
        } else if !ast_node_as::<AstExprVarargs>(node).is_null() {
          let pack = self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
            scope,
            expr,
            &Vec::new(),
            true,
          );
          self.flatten_pack(scope, (*expr).base.location, pack)
        } else if !ast_node_as::<AstExprCall>(node).is_null() {
          let call = ast_node_as::<AstExprCall>(node);
          let pack = self.check_pack_scope_ptr_ast_expr_call(scope, call);
          self.flatten_pack(scope, (*expr).base.location, pack)
        } else if !ast_node_as::<AstExprFunction>(node).is_null() {
          let func = ast_node_as::<AstExprFunction>(node);
          self.check_scope_ptr_ast_expr_function_optional_type_id_bool(
            scope,
            func,
            expected_type,
            generalize,
          )
        } else if !ast_node_as::<AstExprIndexName>(node).is_null() {
          let index_name = ast_node_as::<AstExprIndexName>(node);
          self.check_scope_ptr_ast_expr_index_name(scope, index_name)
        } else if !ast_node_as::<AstExprIndexExpr>(node).is_null() {
          let index_expr = ast_node_as::<AstExprIndexExpr>(node);
          self.check_scope_ptr_ast_expr_index_expr(scope, index_expr)
        } else if !ast_node_as::<AstExprTable>(node).is_null() {
          let table = ast_node_as::<AstExprTable>(node);
          self.check_scope_ptr_ast_expr_table_optional_type_id(scope, table, expected_type)
        } else if !ast_node_as::<AstExprUnary>(node).is_null() {
          let unary = ast_node_as::<AstExprUnary>(node);
          self.check_scope_ptr_ast_expr_unary(scope, unary)
        } else if !ast_node_as::<AstExprBinary>(node).is_null() {
          // C++: check(scope, binary, expected_type) returns the full
          // Inference (type + refinement). The
          // `check_scope_ptr_ast_expr_binary_optional_type_id` wrapper
          // discards the refinement (returns only `.ty`), so dispatch
          // to `check_ast_expr_binary` directly to stay faithful.
          let binary = ast_node_as::<AstExprBinary>(node);
          self.check_ast_expr_binary(
            scope,
            (*binary).base.base.location,
            (*binary).op,
            (*binary).left,
            (*binary).right,
            expected_type,
          )
        } else if !ast_node_as::<AstExprIfElse>(node).is_null() {
          let if_else = ast_node_as::<AstExprIfElse>(node);
          self.check_scope_ptr_ast_expr_if_else_optional_type_id(scope, if_else, expected_type)
        } else if !ast_node_as::<AstExprTypeAssertion>(node).is_null() {
          let type_assert = ast_node_as::<AstExprTypeAssertion>(node);
          self.check_scope_ptr_ast_expr_type_assertion(scope, type_assert)
        } else if !ast_node_as::<AstExprInterpString>(node).is_null() {
          let interp_string = ast_node_as::<AstExprInterpString>(node);
          self.check_scope_ptr_ast_expr_interp_string(scope, interp_string)
        } else if !ast_node_as::<AstExprInstantiate>(node).is_null() {
          let instantiate = ast_node_as::<AstExprInstantiate>(node);
          self.check_scope_ptr_ast_expr_instantiate(scope, instantiate)
        } else {
          let err = ast_node_as::<AstExprError>(node);
          if !err.is_null() {
            // Open question: Should we traverse into this?
            let expressions = (*err).expressions;
            for &sub_expr in expressions.as_slice() {
              self.check_scope_ptr_ast_expr(scope, sub_expr);
            }
            Inference::inference_type_id_refinement_id((*self.builtin_types).error_type, null_mut())
          } else {
            LUAU_ASSERT!(false);
            Inference::inference_type_id_refinement_id(
              self.fresh_type(scope, self.polarity),
              null_mut(),
            )
          }
        }
      };

      *self.inferred_expr_cache.get_or_insert(expr) = result.clone();

      LUAU_ASSERT!(!result.ty.is_null());

      if let Some(module) = &self.module {
        let module_ptr = Arc::as_ptr(module) as *mut Module;
        *(*module_ptr)
          .ast_types
          .get_or_insert(expr as *const AstExpr) = result.ty;
        if let Some(et) = expected_type {
          *(*module_ptr)
            .ast_expected_types
            .get_or_insert(expr as *const AstExpr) = et;
        }
      }

      result
    }
  }
}

use ulua_ast::records::ast_expr_local::AstExprLocal;
