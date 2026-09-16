//! Source: `Analysis/src/TypeInfer.cpp` (TypeChecker::checkRelationalOperation, L2713-3024)
use alloc::{format, sync::Arc};
use core::ptr::null;

use ulua_ast::{
  functions::to_string_ast_alt_b::to_string,
  records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::op_kind::OpKind,
  functions::{
    are_eq_comparable::are_eq_comparable, follow_type::follow_type_id,
    get_identifier_of_base_var_type_infer::get_identifier_of_base_var,
    get_metatable_type::get_metatable_type_id_not_null_builtin_types, get_type_alt_j::get_type_id,
    is_boolean::is_boolean, is_nil::is_nil, is_prim::is_prim, is_string::is_string,
    op_to_meta_table_entry::op_to_meta_table_entry, to_string_to_string_alt_c::to_string_type_id,
  },
  methods::type_checker_check_binary_operation::is_any_like,
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton,
    cannot_infer_binary_operation::CannotInferBinaryOperation, error_type::ErrorType,
    free_type::FreeType, function_type::FunctionType, generic_error::GenericError, module::Module,
    never_type::NeverType, primitive_type::PrimitiveType, singleton_type::SingletonType,
    type_checker::TypeChecker, union_type::UnionType,
  },
  type_aliases::{
    predicate_vec::PredicateVec, scope_ptr_type::ScopePtr,
    singleton_variant::SingletonVariantMember, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_relational_operation(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprBinary,
    lhs_type: TypeId,
    rhs_type: TypeId,
    predicates: &PredicateVec,
  ) -> TypeId {
    let is_equality =
      expr.op == AstExprBinaryOp::CompareEq || expr.op == AstExprBinaryOp::CompareNe;

    let lhs_type = self.relational_strip_nil(lhs_type, expr.op == AstExprBinaryOp::Or);
    let rhs_type = self.relational_strip_nil(rhs_type, false);

    // If we know nothing at all about the lhs type, we can usually say nothing about the result.
    // The notable exception to this is the equality and inequality operators, which always produce a boolean.
    let lhs_is_any = is_any_like(lhs_type);

    // Peephole check for `cond and a or b -> type(a)|type(b)`
    // TODO: Kill this when singleton types arrive. :(
    // SAFETY: expr.left 指向 AST arena 节点；repr(C) base 偏移 0，cast 有效。
    let subexp = ast_node_try_as::<AstExprBinary>(unsafe { &*expr.left.cast::<AstNode>() });
    if expr.op == AstExprBinaryOp::Or
      && let Some(subexp) = subexp
      && subexp.op == AstExprBinaryOp::And
    {
      let sub_scope = self.child_scope(scope, &subexp.base.base.location);
      self.resolve_predicate_vec_scope_ptr_bool(predicates, &sub_scope, true);
      let right_ty = self
        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          &sub_scope,
          // SAFETY: subexp->right 指向 AST arena 节点。
          unsafe { &*subexp.right },
          None,
          false,
        )
        .r#type;
      let stripped = self.relational_strip_nil(right_ty, true);
      return self.union_of_types(
        rhs_type,
        stripped,
        &sub_scope,
        &expr.base.base.location,
        true,
      );
    }

    // Lua casts the results of these to boolean
    match expr.op {
      AstExprBinaryOp::CompareNe
      | AstExprBinaryOp::CompareEq
      | AstExprBinaryOp::CompareLt
      | AstExprBinaryOp::CompareGt
      | AstExprBinaryOp::CompareGe
      | AstExprBinaryOp::CompareLe => {
        if expr.op == AstExprBinaryOp::CompareNe || expr.op == AstExprBinaryOp::CompareEq {
          if self.is_nonstrict_mode() && (is_nil(lhs_type) || is_nil(rhs_type)) {
            return self.boolean_type;
          }

          if lhs_is_any || is_any_like(rhs_type) {
            return self.boolean_type;
          }
          // [[fallthrough]] into the comparison body below.
        }

        // If one of the operand is never, it doesn't make sense to unify these.
        if get_type_id::<NeverType>(lhs_type).is_some()
          || get_type_id::<NeverType>(rhs_type).is_some()
        {
          return self.boolean_type;
        }

        if is_equality {
          // Unless either type is free or any, an equality comparison is only
          // valid when the intersection of the two operands is non-empty.
          //
          // eg it is okay to compare string? == number? because the two types
          // have nil in common, but string == number is not allowed.
          // SAFETY: current_module 在类型检查期间独占；as_ptr 转 *mut 供本调用可变访问。
          let arena = unsafe {
            &mut (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module))
              .internal_types
          };
          let eq_test_result = are_eq_comparable(arena, &mut self.normalizer, lhs_type, rhs_type);
          let eq_test_result = match eq_test_result {
            Some(ok) => ok,
            None => {
              self.report_error_code_too_complex(&expr.base.base.location);
              return self.error_recovery_type_type_id(self.boolean_type);
            }
          };

          if !eq_test_result {
            self.report_error_location_type_error_data(
              &expr.base.base.location,
              TypeErrorData::GenericError(GenericError::new(format!(
                "Type {} cannot be compared with {}",
                to_string_type_id(lhs_type),
                to_string_type_id(rhs_type)
              ))),
            );
            return self.error_recovery_type_type_id(self.boolean_type);
          }
        }

        /* Subtlety here:
         * We need to do this unification first, but there are situations where we don't actually want to
         * report any problems that might have been surfaced as a result of this step because we might already
         * have a better, more descriptive error teed up.
         */
        let mut state = self.mk_unifier(scope, &expr.base.base.location);
        if !is_equality {
          state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
            rhs_type, lhs_type, false, false, None,
          );
          state.log.commit();
        }

        let needs_metamethod = !is_equality;

        let left_type = follow_type_id(lhs_type);
        if get_type_id::<PrimitiveType>(left_type).is_some()
          || get_type_id::<AnyType>(left_type).is_some()
          || get_type_id::<ErrorType>(left_type).is_some()
          || get_type_id::<UnionType>(left_type).is_some()
        {
          self.report_errors(&state.errors);

          // The original version of this check also produced this error when we had a union type.
          // However, the old solver does not readily have the ability to discern if the union is comparable.
          // This is the case when the lhs is e.g. a union of singletons and the rhs is the combined type.
          // The new solver has much more powerful logic for resolving relational operators, but for now,
          // we need to be conservative in the old solver to deliver a reasonable developer experience.
          if !is_equality && state.errors.is_empty() && is_boolean(left_type) {
            self.report_error_location_type_error_data(
              &expr.base.base.location,
              TypeErrorData::GenericError(GenericError::new(format!(
                "Type '{}' cannot be compared with relational operator {}",
                to_string_type_id(left_type),
                to_string(expr.op)
              ))),
            );
          }

          return self.boolean_type;
        }

        let metamethod_name = op_to_meta_table_entry(expr.op);

        // works around gcc false positive "maybe uninitialized" warnings
        let string_no_mt: Option<TypeId> = None;
        let left_metatable: Option<TypeId> = if is_string(lhs_type) {
          string_no_mt
        } else {
          // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
          get_metatable_type_id_not_null_builtin_types(left_type, unsafe { &*self.builtin_types })
        };
        let right_metatable: Option<TypeId> = if is_string(rhs_type) {
          string_no_mt
        } else {
          // SAFETY: 同上。
          get_metatable_type_id_not_null_builtin_types(follow_type_id(rhs_type), unsafe {
            &*self.builtin_types
          })
        };

        if left_metatable != right_metatable {
          let mut matches = false;
          if is_equality {
            if let Some(utv) = get_type_id::<UnionType>(left_type)
              && right_metatable.is_some()
            {
              for &left_option in utv.options.iter() {
                // SAFETY: 同上。
                if get_metatable_type_id_not_null_builtin_types(
                  follow_type_id(left_option),
                  unsafe { &*self.builtin_types },
                ) == right_metatable
                {
                  matches = true;
                  break;
                }
              }
            }

            if !matches
              && let Some(utv) = get_type_id::<UnionType>(rhs_type)
              && left_metatable.is_some()
            {
              for &right_option in utv.options.iter() {
                // SAFETY: 同上。
                if get_metatable_type_id_not_null_builtin_types(
                  follow_type_id(right_option),
                  unsafe { &*self.builtin_types },
                ) == left_metatable
                {
                  matches = true;
                  break;
                }
              }
            }
          }

          if !matches {
            self.report_error_location_type_error_data(
                            &expr.base.base.location,
                            TypeErrorData::GenericError(GenericError::new(format!(
                                "Types {} and {} cannot be compared with {} because they do not have the same metatable",
                                to_string_type_id(lhs_type),
                                to_string_type_id(rhs_type),
                                to_string(expr.op)
                            ))),
                        );
            return self.error_recovery_type_type_id(self.boolean_type);
          }
        }

        if left_metatable.is_some() {
          let metamethod = self.find_metatable_entry(
            lhs_type,
            metamethod_name.clone(),
            &expr.base.base.location,
            true,
          );
          if let Some(metamethod) = metamethod {
            let ftv = get_type_id::<FunctionType>(follow_type_id(metamethod));
            if is_equality && let Some(ftv) = ftv {
              let bool_pack = self.add_type_pack_initializer_list_type_id(&[self.boolean_type]);
              let ret_types = ftv.ret_types;
              state.try_unify_type_pack_id_type_pack_id_bool_entry(bool_pack, ret_types, false);

              if !state.errors.is_empty() {
                self.report_error_location_type_error_data(
                  &expr.base.base.location,
                  TypeErrorData::GenericError(GenericError::new(format!(
                    "Metamethod '{}' must return type 'boolean'",
                    metamethod_name
                  ))),
                );
                return self.error_recovery_type_type_id(self.boolean_type);
              }

              state.log.commit();
            }

            self.report_errors(&state.errors);

            let arg_pack = self.add_type_pack_initializer_list_type_id(&[lhs_type, rhs_type]);
            let ret_pack = self.add_type_pack_initializer_list_type_id(&[self.boolean_type]);
            let mut ftv2 = FunctionType::function_type_new(arg_pack, ret_pack, None, false);
            ftv2.level = scope.level;
            let actual_function_type = self.add_type_tv_internal(ftv2);
            let inst_actual =
              self.instantiate(scope, actual_function_type, expr.base.base.location, null());
            let inst_meta = self.instantiate(scope, metamethod, expr.base.base.location, null());
            state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
              inst_actual,
              inst_meta,
              true,
              false,
              None,
            );

            state.log.commit();

            self.report_errors(&state.errors);
            return self.boolean_type;
          } else if needs_metamethod {
            self.report_error_location_type_error_data(
              &expr.base.base.location,
              TypeErrorData::GenericError(GenericError::new(format!(
                "Table {} does not offer metamethod {}",
                to_string_type_id(lhs_type),
                metamethod_name
              ))),
            );
            return self.error_recovery_type_type_id(self.boolean_type);
          }
        }

        if get_type_id::<FreeType>(follow_type_id(lhs_type)).is_some() && !is_equality {
          let name = get_identifier_of_base_var(expr.left);
          self.report_error_location_type_error_data(
            &expr.base.base.location,
            TypeErrorData::CannotInferBinaryOperation(CannotInferBinaryOperation::new(
              expr.op,
              name,
              OpKind::Comparison,
            )),
          );
          return self.error_recovery_type_type_id(self.boolean_type);
        }

        if needs_metamethod {
          self.report_error_location_type_error_data(
            &expr.base.base.location,
            TypeErrorData::GenericError(GenericError::new(format!(
              "Type {} cannot be compared with {} because it has no metatable",
              to_string_type_id(lhs_type),
              to_string(expr.op)
            ))),
          );
          return self.error_recovery_type_type_id(self.boolean_type);
        }

        self.boolean_type
      }

      AstExprBinaryOp::And => {
        if lhs_is_any {
          lhs_type
        } else {
          // If lhs is free, we can't tell which 'falsy' components it has, if any
          if get_type_id::<FreeType>(lhs_type).is_some() {
            let false_singleton = self.singleton_type_bool(false);
            let union_ty = self.add_type_tv_internal(UnionType {
              options: alloc::vec![self.nil_type, false_singleton],
            });
            return self.union_of_types(union_ty, rhs_type, scope, &expr.base.base.location, false);
          }

          let (oty, not_never) = self.pick_types_from_sense(lhs_type, false, self.never_type); // Filter out falsy types

          if not_never {
            let oty = oty.unwrap();

            // Perform a limited form of type reduction for booleans
            if is_prim(oty, PrimitiveType::BOOLEAN) && self.is_boolean_singleton(rhs_type) {
              return self.boolean_type;
            }
            if is_prim(rhs_type, PrimitiveType::BOOLEAN) && self.is_boolean_singleton(oty) {
              return self.boolean_type;
            }

            self.union_of_types(oty, rhs_type, scope, &expr.base.base.location, false)
          } else {
            rhs_type
          }
        }
      }

      AstExprBinaryOp::Or => {
        if lhs_is_any {
          lhs_type
        } else {
          let (oty, not_never) = self.pick_types_from_sense(lhs_type, true, self.never_type); // Filter out truthy types

          if not_never {
            let oty = oty.unwrap();

            // Perform a limited form of type reduction for booleans
            if is_prim(oty, PrimitiveType::BOOLEAN) && self.is_boolean_singleton(rhs_type) {
              return self.boolean_type;
            }
            if is_prim(rhs_type, PrimitiveType::BOOLEAN) && self.is_boolean_singleton(oty) {
              return self.boolean_type;
            }

            self.union_of_types(oty, rhs_type, scope, &expr.base.base.location, true)
          } else {
            rhs_type
          }
        }
      }

      _ => {
        self.ice_string_location(
          &format!(
            "checkRelationalOperation called with incorrect binary expression '{}'",
            to_string(expr.op)
          ),
          &expr.base.base.location,
        );
        unreachable!()
      }
    }
  }

  /// C++ `stripNil` lambda inside `checkRelationalOperation` (L2721-2739).
  fn relational_strip_nil(&mut self, ty: TypeId, is_or_op: bool) -> TypeId {
    let ty = follow_type_id(ty);
    if !self.is_nonstrict_mode() && !is_or_op {
      return ty;
    }

    if get_type_id::<UnionType>(ty).is_some() {
      let cleaned = self.try_strip_union_from_nil(ty);

      // If there is no union option without 'nil'
      match cleaned {
        None => return self.nil_type,
        Some(c) => return follow_type_id(c),
      }
    }

    ty
  }

  /// C++ `get<BooleanSingleton>(get<SingletonType>(follow(ty)))` truthiness check.
  fn is_boolean_singleton(&self, ty: TypeId) -> bool {
    get_type_id::<SingletonType>(follow_type_id(ty)).is_some_and(|stv| {
      <BooleanSingleton as SingletonVariantMember>::get_if(&stv.variant).is_some()
    })
  }
}
