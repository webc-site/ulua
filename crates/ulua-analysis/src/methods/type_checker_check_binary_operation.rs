//! Source: `Analysis/src/TypeInfer.cpp` (TypeChecker::checkBinaryOperation, L3026-3165)
use alloc::{format, sync::Arc};
use core::ptr::null;

use ulua_ast::{
  functions::to_string_ast_alt_b::to_string,
  records::ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
};

use crate::{
  enums::op_kind::OpKind,
  functions::{
    first::first, follow_type::follow_type_id,
    get_identifier_of_base_var_type_infer::get_identifier_of_base_var, get_type_alt_j::get_type_id,
    op_to_meta_table_entry::op_to_meta_table_entry, to_string_to_string_alt_c::to_string_type_id,
    type_could_have_metatable::type_could_have_metatable,
  },
  records::{
    any_type::AnyType, cannot_infer_binary_operation::CannotInferBinaryOperation,
    error_type::ErrorType, free_type::FreeType, function_type::FunctionType,
    generic_error::GenericError, module::Module, never_type::NeverType, r#type::Type,
    type_checker::TypeChecker, union_type::UnionType,
  },
  type_aliases::{
    predicate_vec::PredicateVec, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_binary_operation(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprBinary,
    lhs_type: TypeId,
    rhs_type: TypeId,
    _predicates: &PredicateVec,
  ) -> TypeId {
    match expr.op {
      AstExprBinaryOp::CompareNe
      | AstExprBinaryOp::CompareEq
      | AstExprBinaryOp::CompareLt
      | AstExprBinaryOp::CompareGt
      | AstExprBinaryOp::CompareGe
      | AstExprBinaryOp::CompareLe
      | AstExprBinaryOp::And
      | AstExprBinaryOp::Or => {
        return self.check_relational_operation(scope, expr, lhs_type, rhs_type, _predicates);
      }
      _ => {}
    }

    let lhs_type = follow_type_id(lhs_type);
    let rhs_type = follow_type_id(rhs_type);

    if !self.is_nonstrict_mode() && get_type_id::<FreeType>(lhs_type).is_some() {
      let name = get_identifier_of_base_var(expr.left);
      self.report_error_location_type_error_data(
        &expr.base.base.location,
        TypeErrorData::CannotInferBinaryOperation(CannotInferBinaryOperation::new(
          expr.op,
          name,
          OpKind::Operation,
        )),
      );
      // We will fall-through to the `return any_type` check below.
    }

    // If we know nothing at all about the lhs type, we can usually say nothing about the result.
    // The notable exception to this is the equality and inequality operators, which always produce a boolean.
    let lhs_is_any = is_any_like(lhs_type);
    let rhs_is_any = is_any_like(rhs_type);

    if lhs_is_any {
      return lhs_type;
    }
    if rhs_is_any {
      return rhs_type;
    }

    if get_type_id::<FreeType>(lhs_type).is_some() {
      // Inferring this accurately will get a bit weird.
      // If the lhs type is not known, it could be assumed that it is a table or class that has a metatable
      // that defines the required method, but we don't know which.
      // For now, we'll give up and hope for the best.
      return self.any_type;
    }

    if get_type_id::<FreeType>(rhs_type).is_some() {
      self.unify_type_id_type_id_scope_ptr_location(
        rhs_type,
        lhs_type,
        scope,
        &expr.base.base.location,
      );
    }

    if type_could_have_metatable(lhs_type) || type_could_have_metatable(rhs_type) {
      let op = op_to_meta_table_entry(expr.op);
      if let Some(fnt) =
        self.find_metatable_entry(lhs_type, op.clone(), &expr.base.base.location, true)
      {
        return self.check_binary_operation_metatable_call(scope, expr, fnt, lhs_type, rhs_type);
      }
      if let Some(fnt) = self.find_metatable_entry(rhs_type, op, &expr.base.base.location, true) {
        // Note the intentionally reversed arguments here.
        return self.check_binary_operation_metatable_call(scope, expr, fnt, rhs_type, lhs_type);
      }

      self.report_error_location_type_error_data(
        &expr.base.base.location,
        TypeErrorData::GenericError(GenericError::new(format!(
          "Binary operator '{}' not supported by types '{}' and '{}'",
          to_string(expr.op),
          to_string_type_id(lhs_type),
          to_string_type_id(rhs_type)
        ))),
      );

      return self.error_recovery_type_scope_ptr(scope);
    }

    // SAFETY: expr.left/expr.right 指向 AST arena 节点（parser 保证非空）。
    let (left_loc, right_loc) =
      unsafe { ((*expr.left).base.location, (*expr.right).base.location) };

    match expr.op {
      AstExprBinaryOp::Concat => {
        let str_num = self.add_type_tv_internal(UnionType {
          options: alloc::vec![self.string_type, self.number_type],
        });
        let errs = self.try_unify(lhs_type, str_num, scope, &left_loc);
        self.report_errors(&errs);
        let str_num2 = self.add_type_tv_internal(UnionType {
          options: alloc::vec![self.string_type, self.number_type],
        });
        let errs = self.try_unify(rhs_type, str_num2, scope, &right_loc);
        self.report_errors(&errs);
        self.string_type
      }
      AstExprBinaryOp::Add
      | AstExprBinaryOp::Sub
      | AstExprBinaryOp::Mul
      | AstExprBinaryOp::Div
      | AstExprBinaryOp::FloorDiv
      | AstExprBinaryOp::Mod
      | AstExprBinaryOp::Pow => {
        let errs = self.try_unify(lhs_type, self.number_type, scope, &left_loc);
        self.report_errors(&errs);
        let errs = self.try_unify(rhs_type, self.number_type, scope, &right_loc);
        self.report_errors(&errs);
        self.number_type
      }
      _ => {
        // These should have been handled with checkRelationalOperation
        self.any_type
      }
    }
  }

  /// C++ lambda `checkMetatableCall` inside `checkBinaryOperation` (L3083-3120).
  fn check_binary_operation_metatable_call(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprBinary,
    fnt: TypeId,
    lhst: TypeId,
    rhst: TypeId,
  ) -> TypeId {
    let actual_function_type = self.instantiate(scope, fnt, expr.base.base.location, null());
    let arguments = self.add_type_pack_initializer_list_type_id(&[lhst, rhst]);
    let ret_type_pack = self.fresh_type_pack_scope_ptr(scope.clone());
    let mut ftv = FunctionType::function_type_new(arguments, ret_type_pack, None, false);
    ftv.level = scope.level;
    let expected_function_type = self.add_type_tv_internal(ftv);

    let mut state = self.mk_unifier(scope, &expr.base.base.location);
    state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
      actual_function_type,
      expected_function_type,
      true,
      false,
      None,
    );

    self.report_errors(&state.errors);
    let has_errors = !state.errors.is_empty();

    if has_errors {
      // If there are unification errors, the return type may still be unknown
      // so we loosen the argument types to see if that helps.
      let fallback_arguments = self.fresh_type_pack_scope_ptr(scope.clone());
      let mut fallback_ftv =
        FunctionType::function_type_new(fallback_arguments, ret_type_pack, None, false);
      fallback_ftv.level = scope.level;
      let fallback_function_type = self.add_type_tv_internal(fallback_ftv);
      state.errors.clear();
      state.log.clear();

      state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
        actual_function_type,
        fallback_function_type,
        true,
        false,
        None,
      );

      if state.errors.is_empty() {
        state.log.commit();
      }
    } else {
      state.log.commit();
    }

    let mut ret_type = first(ret_type_pack, false).unwrap_or(self.nil_type);
    if has_errors {
      ret_type = self.error_recovery_type_type_id(ret_type);
    }

    ret_type
  }

  /// Allocate a `Type` value directly in the current module's internal type arena, mirroring the
  /// C++ `addType(...)` used by `checkBinaryOperation`.
  pub(crate) fn add_type_tv_internal<T>(&mut self, tv: T) -> TypeId
  where
    T: Into<Type> + 'static,
  {
    // SAFETY: current_module 在类型检查期间独占使用（C++ 直接改 module->internalTypes
    // 同义）；Arc::as_ptr 转 *mut 仅供此处插入，单线程无别名。
    unsafe {
      (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module))
        .internal_types
        .add_type(tv)
    }
  }
}

/// C++ `get_if<AnyType>(t) || get_if<ErrorType>(t) || get_if<NeverType>(t)`：
/// “对该类型一无所知”的判定（TypeInfer.cpp checkBinaryOperation 等）。
pub(crate) fn is_any_like(ty: TypeId) -> bool {
  get_type_id::<AnyType>(ty).is_some()
    || get_type_id::<ErrorType>(ty).is_some()
    || get_type_id::<NeverType>(ty).is_some()
}
