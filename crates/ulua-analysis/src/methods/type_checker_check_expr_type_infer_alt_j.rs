use alloc::{format, string::ToString};
use core::ptr::null;

use ulua_ast::{
  functions::to_string_ast::to_string,
  records::ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    first::first, follow_type::follow_type_id, has_length::has_length,
    to_string_to_string_alt_c::to_string_type_id,
    type_could_have_metatable::type_could_have_metatable,
  },
  methods::type_checker_check_binary_operation::is_any_like,
  records::{
    function_type::FunctionType, generic_error::GenericError, not_a_table::NotATable,
    not_predicate::NotPredicate, type_checker::TypeChecker, with_predicate::WithPredicate,
  },
  type_aliases::{
    predicate::Predicate, predicate_vec::PredicateVec, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_unary(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprUnary,
  ) -> WithPredicate<TypeId> {
    let boolean_type = self.boolean_type;
    let number_type = self.number_type;
    let nil_type = self.nil_type;

    let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
      scope,
      // SAFETY: expr.expr 指向 AST arena 节点（parser 保证非空）。
      unsafe { &*expr.expr },
      None,
      false,
    );
    let mut operand_type = follow_type_id(result.r#type);

    match expr.op {
      AstExprUnaryOp::Not => WithPredicate::with_predicate_t_predicate_vec(
        boolean_type,
        PredicateVec::from(alloc::vec![Predicate::Not(NotPredicate {
          predicates: result.predicates,
        })]),
      ),
      AstExprUnaryOp::Minus => {
        let operand_is_any = is_any_like(operand_type);

        if operand_is_any {
          return WithPredicate::with_predicate_t(operand_type);
        }

        if type_could_have_metatable(operand_type) {
          if let Some(fnt) = self.find_metatable_entry(
            operand_type,
            "__unm".to_string(),
            &expr.base.base.location,
            true,
          ) {
            let actual_function_type =
              self.instantiate(scope, fnt, expr.base.base.location, null());
            let arguments = self.add_type_pack_initializer_list_type_id(&[operand_type]);
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
            state.log.commit();

            self.report_errors(&state.errors);
            let has_errors = !state.errors.is_empty();

            let mut ret_type = first(ret_type_pack, false).unwrap_or(nil_type);
            if has_errors {
              ret_type = self.error_recovery_type_type_id(ret_type);
            }

            return WithPredicate::with_predicate_t(ret_type);
          }

          self.report_error_location_type_error_data(
            &expr.base.base.location,
            TypeErrorData::GenericError(GenericError::new(format!(
              "Unary operator '{}' not supported by type '{}'",
              to_string(expr.op),
              to_string_type_id(operand_type)
            ))),
          );
          return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
        }

        let errs = self.try_unify(operand_type, number_type, scope, &expr.base.base.location);
        self.report_errors(&errs);
        WithPredicate::with_predicate_t(number_type)
      }
      AstExprUnaryOp::Len => {
        self.tablify(operand_type);

        operand_type = self.strip_from_nil_and_report(operand_type, &expr.base.base.location);

        // # operator is guaranteed to return number
        if is_any_like(operand_type) {
          return WithPredicate::with_predicate_t(number_type);
        }

        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());

        if type_could_have_metatable(operand_type)
          && let Some(fnt) = self.find_metatable_entry(
            operand_type,
            "__len".to_string(),
            &expr.base.base.location,
            true,
          )
        {
          let actual_function_type = self.instantiate(scope, fnt, expr.base.base.location, null());
          let arguments = self.add_type_pack_initializer_list_type_id(&[operand_type]);
          let ret_type_pack = self.add_type_pack_initializer_list_type_id(&[number_type]);
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
          state.log.commit();

          self.report_errors(&state.errors);
        }

        if !has_length(operand_type, &mut seen, &mut self.recursion_count) {
          self.report_error_location_type_error_data(
            &expr.base.base.location,
            TypeErrorData::NotATable(NotATable { ty: operand_type }),
          );
        }

        WithPredicate::with_predicate_t(number_type)
      }
    }
  }
}
