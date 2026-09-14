use alloc::{string::String, sync::Arc, vec::Vec};
use core::{ffi::CStr, str::from_utf8};

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::AstExprBinary,
    ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs,
    ast_node::AstNode,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_common::FInt;

use crate::{
  enums::{table_state::TableState, value_context::ValueContext},
  functions::{
    as_mutable_type_pack::as_mutable_type_pack_id, first::first, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_mutable_type_pack::get_mutable_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id, maybe_singleton::maybe_singleton,
    maybe_string::maybe_string, try_get_l_value::try_get_l_value,
  },
  records::{
    count_mismatch::CountMismatchContext,
    free_type_pack::FreeTypePack,
    generic_error::GenericError,
    generic_type_pack::GenericTypePack,
    module::Module,
    recursion_counter::RecursionCounter,
    table_type::TableType,
    truthy_predicate::TruthyPredicate,
    type_checker::TypeChecker,
    type_error::TypeError,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    union_type::UnionType,
    unknown_symbol::{Context, UnknownSymbol},
    variadic_type_pack::VariadicTypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, predicate::Predicate, predicate_vec::PredicateVec,
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_expr_scope_ptr_ast_expr_optional_type_id_bool(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
    expected_type: Option<TypeId>,
    force_singleton: bool,
  ) -> WithPredicate<TypeId> {
    let _rc = unsafe { RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count) };
    let limit = FInt::LuauCheckRecursionLimit.get();
    if limit > 0 && self.check_recursion_count >= limit {
      self.report_error_code_too_complex(&expr.base.location);
      return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
    }

    let mut result = if let Some(group) = ast_node_try_as::<AstExprGroup>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        // SAFETY: group->expr 指向 AST arena 节点。
        unsafe { &*group.expr },
        expected_type,
        false,
      )
    } else if ast_node_is::<AstExprConstantNil>(expr) {
      WithPredicate::with_predicate_t(self.nil_type)
    } else if let Some(bool_expr) = ast_node_try_as::<AstExprConstantBool>(&expr.base) {
      let use_singleton = force_singleton || expected_type.is_some_and(maybe_singleton);
      WithPredicate::with_predicate_t(if use_singleton {
        self.singleton_type_bool(bool_expr.value)
      } else {
        self.boolean_type
      })
    } else if let Some(string_expr) = ast_node_try_as::<AstExprConstantString>(&expr.base) {
      let use_singleton = force_singleton || expected_type.is_some_and(maybe_singleton);
      if use_singleton {
        let bytes = string_expr.value.as_bytes();
        WithPredicate::with_predicate_t(
          self.singleton_type_string(String::from_utf8_lossy(bytes).into_owned()),
        )
      } else {
        WithPredicate::with_predicate_t(self.string_type)
      }
    } else if ast_node_is::<AstExprConstantNumber>(expr) {
      WithPredicate::with_predicate_t(self.number_type)
    } else if ast_node_is::<AstExprConstantInteger>(expr) {
      WithPredicate::with_predicate_t(self.integer_type)
    } else if let Some(local_expr) = ast_node_try_as::<AstExprLocal>(&expr.base) {
      let lvalue = try_get_l_value(&local_expr.base);
      if let Some(lvalue) = lvalue {
        if let Some(ty) = self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue) {
          WithPredicate::with_predicate_t_predicate_vec(
            ty,
            PredicateVec::from(alloc::vec![Predicate::Truthy(TruthyPredicate {
              lvalue,
              location: local_expr.base.base.location,
            })]),
          )
        } else {
          // SAFETY: local 指向 AST arena 节点；name.value 可能为 null，先判空。
          let name = unsafe {
            if (*local_expr.local).name.value.is_null() {
              String::new()
            } else {
              CStr::from_ptr((*local_expr.local).name.value)
                .to_string_lossy()
                .into_owned()
            }
          };
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            local_expr.base.base.location,
            TypeErrorData::UnknownSymbol(UnknownSymbol::new(name, Context::Binding)),
          ));
          WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
        }
      } else {
        self.ice_string_location(
          "AstExprLocal exists but no LValue was produced",
          &expr.base.location,
        );
        WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
      }
    } else if let Some(global_expr) = ast_node_try_as::<AstExprGlobal>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_global(scope, global_expr)
    } else if let Some(varargs_expr) = ast_node_try_as::<AstExprVarargs>(&expr.base) {
      // SAFETY: follow_type_pack_id 为 unsafe 函数，pack 句柄有效。
      let vararg_pack =
        unsafe { follow_type_pack_id(self.check_expr_pack(scope, &varargs_expr.base).r#type) };

      if get_type_pack_id::<TypePack>(vararg_pack).is_some() {
        WithPredicate::with_predicate_t(first(vararg_pack, false).unwrap_or(self.nil_type))
      } else if get_type_pack_id::<FreeTypePack>(vararg_pack).is_some() {
        let head = self.fresh_type_scope_ptr(scope.clone());
        let tail = self.fresh_type_pack_scope_ptr(scope.clone());
        if let Some(pack) = get_mutable_type_pack_id::<TypePack>(vararg_pack) {
          *pack = TypePack {
            head: alloc::vec![head],
            tail: Some(tail),
          };
        } else {
          // SAFETY: as_mutable_type_pack_id 去 const（C++ asMutable 同义），句柄有效。
          unsafe {
            *as_mutable_type_pack_id(vararg_pack) = TypePackVar::from(TypePack {
              head: alloc::vec![head],
              tail: Some(tail),
            });
          }
        }
        WithPredicate::with_predicate_t(head)
      } else if get_type_pack_id::<ErrorTypePack>(vararg_pack).is_some() {
        WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
      } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(vararg_pack) {
        WithPredicate::with_predicate_t(vtp.ty)
      } else if get_type_pack_id::<GenericTypePack>(vararg_pack).is_some() {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          varargs_expr.base.base.location,
          TypeErrorData::GenericError(GenericError::new(String::from(
            "Trying to get a type from a variadic type parameter",
          ))),
        ));
        WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
      } else {
        self.ice_string_location(
          "Unknown TypePack type in checkExpr(AstExprVarargs)",
          &expr.base.location,
        );
        WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
      }
    } else if let Some(call_expr) = ast_node_try_as::<AstExprCall>(&expr.base) {
      let pack_result = self.check_expr_pack(scope, &call_expr.base);
      // SAFETY: follow_type_pack_id 为 unsafe 函数，pack 句柄有效。
      let ret_pack = unsafe { follow_type_pack_id(pack_result.r#type) };

      if let Some(pack) = get_type_pack_id::<TypePack>(ret_pack) {
        WithPredicate::with_predicate_t_predicate_vec(
          pack.head.first().copied().unwrap_or(self.nil_type),
          pack_result.predicates,
        )
      } else if get_type_pack_id::<FreeTypePack>(ret_pack).is_some() {
        let head = self.fresh_type_type_level(scope.level);
        let tail = self.fresh_type_pack_type_level(scope.level);
        let pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
          head: alloc::vec![head],
          tail: Some(tail),
        }));
        self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
          pack,
          ret_pack,
          scope,
          &call_expr.base.base.location,
          CountMismatchContext::Arg,
        );
        WithPredicate::with_predicate_t_predicate_vec(head, pack_result.predicates)
      } else if get_type_pack_id::<ErrorTypePack>(ret_pack).is_some() {
        WithPredicate::with_predicate_t_predicate_vec(
          self.error_recovery_type_scope_ptr(scope),
          pack_result.predicates,
        )
      } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(ret_pack) {
        WithPredicate::with_predicate_t_predicate_vec(vtp.ty, pack_result.predicates)
      } else if get_type_pack_id::<GenericTypePack>(ret_pack).is_some() {
        WithPredicate::with_predicate_t_predicate_vec(self.any_type, pack_result.predicates)
      } else {
        self.ice_string_location(
          "Unknown TypePack type in checkExpr(AstExprCall)",
          &expr.base.location,
        );
        WithPredicate::with_predicate_t_predicate_vec(
          self.error_recovery_type_scope_ptr(scope),
          pack_result.predicates,
        )
      }
    } else if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_index_name(scope, index_name)
    } else if let Some(index_expr) = ast_node_try_as::<AstExprIndexExpr>(&expr.base) {
      let ty = self.check_l_value(scope, &index_expr.base, ValueContext::RValue);
      if let Some(lvalue) = try_get_l_value(&index_expr.base) {
        if let Some(refined_ty) = self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue) {
          WithPredicate::with_predicate_t_predicate_vec(
            refined_ty,
            PredicateVec::from(alloc::vec![Predicate::Truthy(TruthyPredicate {
              lvalue,
              location: index_expr.base.base.location,
            })]),
          )
        } else {
          WithPredicate::with_predicate_t(ty)
        }
      } else {
        WithPredicate::with_predicate_t(ty)
      }
    } else if let Some(function_expr) = ast_node_try_as::<AstExprFunction>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_function_optional_type_id(
        scope,
        function_expr,
        expected_type,
      )
    } else if let Some(table_expr) = ast_node_try_as::<AstExprTable>(&expr.base) {
      let _table_rc =
        unsafe { RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count) };
      if limit > 0 && self.check_recursion_count >= limit {
        self.report_error_code_too_complex(&table_expr.base.base.location);
        return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
      }

      let mut field_types = Vec::with_capacity(table_expr.items.size);

      let mut expected_table: Option<&TableType> = None;
      let mut expected_union: Option<&UnionType> = None;
      let mut expected_index_type: Option<TypeId> = None;
      let mut expected_index_result_type: Option<TypeId> = None;

      if let Some(expected_type) = expected_type {
        let followed = follow_type_id(expected_type);
        if let Some(ttv) = get_type_id::<TableType>(followed) {
          if ttv.state == TableState::Sealed {
            expected_table = Some(ttv);

            if let Some(indexer) = &ttv.indexer {
              expected_index_type = Some(indexer.index_type);
              expected_index_result_type = Some(indexer.index_result_type);
            }
          }
        } else if let Some(utv) = get_type_id::<UnionType>(followed) {
          expected_union = Some(utv);
        }
      }

      for item in table_expr.items.iter() {
        let mut expected_result_type: Option<TypeId> = None;
        let mut is_indexed_item = false;

        if item.kind == ItemKind::List {
          expected_result_type = expected_index_result_type;
          is_indexed_item = true;
        } else if item.kind == ItemKind::Record || item.kind == ItemKind::General {
          if !item.key.is_null() {
            // SAFETY: item.key 非 null，指向 AST arena 节点；repr(C) base 偏移 0，cast 有效。
            let key =
              ast_node_try_as::<AstExprConstantString>(unsafe { &*item.key.cast::<AstNode>() });
            if let Some(key) = key {
              let key_str = String::from(from_utf8(key.value.as_bytes()).unwrap_or(""));

              if let Some(expected_table) = expected_table {
                if let Some(prop) = expected_table.props.get(&key_str) {
                  expected_result_type = Some(prop.type_deprecated());
                } else if expected_index_type.is_some_and(maybe_string) {
                  expected_result_type = expected_index_result_type;
                }
              } else if let Some(expected_union) = expected_union {
                let mut expected_result_types = Vec::new();

                for &expected_option in &expected_union.options {
                  let Some(ttv) = get_type_id::<TableType>(follow_type_id(expected_option)) else {
                    continue;
                  };

                  if let Some(prop) = ttv.props.get(&key_str) {
                    expected_result_types.push(prop.type_deprecated());
                  } else if let Some(indexer) = &ttv.indexer
                    && maybe_string(indexer.index_type)
                  {
                    expected_result_types.push(indexer.index_result_type);
                  }
                }

                if expected_result_types.len() == 1 {
                  expected_result_type = Some(expected_result_types[0]);
                } else if expected_result_types.len() > 1 {
                  expected_result_type = Some(self.add_type(&UnionType {
                    options: expected_result_types,
                  }));
                }
              }
            } else {
              expected_result_type = expected_index_result_type;
              is_indexed_item = true;
            }
          } else {
            expected_result_type = expected_index_result_type;
            is_indexed_item = true;
          }
        }

        let key_type = if item.key.is_null() {
          self.number_type
        } else {
          self
            .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
              scope,
              // SAFETY: item.key 非 null，指向 AST arena 节点。
              unsafe { &*item.key },
              expected_index_type,
              false,
            )
            .r#type
        };
        let value_type = self
          .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
            scope,
            // SAFETY: item.value 指向 AST arena 节点。
            unsafe { &*item.value },
            expected_result_type,
            false,
          )
          .r#type;
        field_types.push((key_type, value_type));

        if is_indexed_item && expected_index_result_type.is_none() {
          expected_index_result_type = Some(value_type);
        }
      }
      WithPredicate::with_predicate_t(self.check_expr_table(
        scope,
        table_expr,
        &field_types,
        expected_type,
      ))
    } else if let Some(unary_expr) = ast_node_try_as::<AstExprUnary>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_unary(scope, unary_expr)
    } else if let Some(binary_expr) = ast_node_try_as::<AstExprBinary>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_binary_optional_type_id(scope, binary_expr, expected_type)
    } else if let Some(type_assertion) = ast_node_try_as::<AstExprTypeAssertion>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_type_assertion(scope, type_assertion)
    } else if let Some(error_expr) = ast_node_try_as::<AstExprError>(&expr.base) {
      // SAFETY: current_module 在类型检查期间独占（C++ 直接读改 module->errors 同义）。
      let old_size = unsafe {
        (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module))
          .errors
          .len()
      };
      for child in error_expr.expressions.iter() {
        self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          scope,
          // SAFETY: child 指向 AST arena 节点。
          unsafe { &**child },
          None,
          false,
        );
      }
      // SAFETY: 同上；子表达式检查可能递归取 module，故此处独立短时访问。
      unsafe {
        (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module))
          .errors
          .truncate(old_size);
      }
      WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
    } else if let Some(if_else) = ast_node_try_as::<AstExprIfElse>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_if_else_optional_type_id(scope, if_else, expected_type)
    } else if let Some(interp) = ast_node_try_as::<AstExprInterpString>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_interp_string(scope, interp)
    } else if let Some(instantiate) = ast_node_try_as::<AstExprInstantiate>(&expr.base) {
      self.check_expr_scope_ptr_ast_expr_instantiate(scope, instantiate)
    } else {
      self.ice_string_location("Unhandled AstExpr", &expr.base.location);
      WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
    };

    result.r#type = follow_type_id(result.r#type);

    // SAFETY: current_module 在类型检查期间独占（C++ 直接改 module->astTypes 同义）。
    let module =
      unsafe { &mut *(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module) };
    let key = expr as *const AstExpr;
    if module.ast_types.find(&key).is_none() {
      *module.ast_types.get_or_insert(key) = result.r#type;
    }
    if let Some(expected_type) = expected_type {
      *module.ast_expected_types.get_or_insert(key) = expected_type;
    }

    result
  }
}
