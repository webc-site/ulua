//! Faithful port of `ControlFlow TypeChecker::check(const ScopePtr& scope, const AstStatForIn& forin)`
//! (Analysis/src/TypeInfer.cpp:1202-1392).

use alloc::{string::String, vec::Vec};
use core::ptr::{null, null_mut};

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_stat_for_in::AstStatForIn, ast_type_or_pack::AstTypeOrPack, location::Location,
    position::Position,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    first::first, flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_table_type::get_table_type,
    get_type_alt_j::get_type_id, get_type_pack::get as get_type_pack,
  },
  methods::type_checker_check_function_signature::scope_mut,
  records::{
    any_type::AnyType, binding::Binding, cannot_call_non_function::CannotCallNonFunction,
    count_mismatch::CountMismatchContext, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, never_type::NeverType, symbol::Symbol, txn_log::TxnLog,
    type_checker::TypeChecker, type_pack::TypePack, type_pack_var::TypePackVar,
    union_type::UnionType,
  },
  type_aliases::{
    error_type::ErrorType, error_type_pack::ErrorTypePack, scope_ptr_type::ScopePtr,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_for_in(
    &mut self,
    scope: &ScopePtr,
    forin: &AstStatForIn,
  ) -> ControlFlow {
    // ScopePtr loopScope = childScope(scope, forin.location);
    let loop_scope = self.child_scope(scope, &forin.base.base.location);

    // std::vector<TypeId> varTypes; varTypes.reserve(forin.vars.size);
    let mut var_types: Vec<TypeId> = Vec::with_capacity(forin.vars.size);

    let vars = forin.vars.as_slice();

    for &var in vars {
      // AstType* ann = vars[i]->annotation;
      // TypeId ty = ann ? resolveType(scope, *ann) : anyIfNonstrict(freshType(loopScope));
      let ann = unsafe { (*var).annotation };
      let ty = if !ann.is_null() {
        // SAFETY: ann 非 null，指向 AST arena 节点。
        self.resolve_type(scope.clone(), unsafe { &*ann })
      } else {
        let fresh = self.fresh_type_scope_ptr(loop_scope.clone());
        self.any_if_nonstrict(fresh)
      };

      // loopScope->bindings[vars[i]] = {ty, vars[i]->location};
      // SAFETY: var 指向 AST arena 节点；见 scope_mut 契约。
      unsafe {
        (*scope_mut(&loop_scope)).bindings.insert(
          Symbol::from_local(var),
          Binding {
            type_id: ty,
            location: (*var).location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }

      // varTypes.push_back(ty);
      var_types.push(ty);
    }

    let values_slice = forin.values.as_slice();
    let first_value = values_slice.first().copied().unwrap_or(null_mut());

    // if (!firstValue)
    //     ice("expected at least an iterator function value, but we parsed nothing");
    if first_value.is_null() {
      self.ice_string("expected at least an iterator function value, but we parsed nothing");
    }
    // SAFETY: first_value 非 null，指向 AST arena 节点。
    let first_value_ref = unsafe { &*first_value };

    // TypeId iterTy = nullptr;
    // TypePackId callRetPack = nullptr;
    let mut iter_ty: TypeId;
    let mut call_ret_pack: TypePackId = null();

    // if (forin.values.size == 1 && firstValue->is<AstExprCall>())
    // SAFETY: repr(C) base 偏移 0，cast 有效。
    let first_value_call = ast_node_try_as::<AstExprCall>(unsafe { &(*first_value).base });
    if forin.values.size == 1
      && let Some(expr_call) = first_value_call
    {
      // callRetPack = checkExprPack(scope, *exprCall).type;
      // callRetPack = follow(callRetPack);
      call_ret_pack = self.check_expr_pack(scope, &expr_call.base).r#type;
      // SAFETY: follow_type_pack_id 为 unsafe 函数，句柄有效。
      call_ret_pack = unsafe { follow_type_pack_id(call_ret_pack) };

      // if (get<FreeTypePack>(callRetPack))
      if get_type_pack::<FreeTypePack>(call_ret_pack).is_some() {
        // iterTy = freshType(scope);
        iter_ty = self.fresh_type_scope_ptr(scope.clone());

        // unify(callRetPack, addTypePack({{iterTy}, freshTypePack(scope)}), scope, forin.location);
        let fresh_tail = self.fresh_type_pack_scope_ptr(scope.clone());
        let expected = self.add_type_pack_vector_type_id_optional_type_pack_id(
          &alloc::vec![iter_ty],
          Some(fresh_tail),
        );
        self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
          call_ret_pack,
          expected,
          scope,
          &forin.base.base.location,
          CountMismatchContext::Arg,
        );
      }
      // else if (get<ErrorTypePack>(callRetPack) || !first(callRetPack))
      else if get_type_pack::<ErrorTypePack>(call_ret_pack).is_some()
        || first(call_ret_pack, true).is_none()
      {
        // for (TypeId var : varTypes)
        //     unify(errorRecoveryType(scope), var, scope, forin.location);
        let err_ty = self.error_recovery_type_scope_ptr(scope);
        for &var in &var_types {
          self.unify_type_id_type_id_scope_ptr_location(
            err_ty,
            var,
            scope,
            &forin.base.base.location,
          );
        }

        // return check(loopScope, *forin.body);
        // SAFETY: forin.body 指向 AST arena 节点。
        return self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });
      }
      // else
      else {
        // iterTy = *first(callRetPack);
        iter_ty = first(call_ret_pack, true).unwrap();
        // iterTy = instantiate(scope, iterTy, exprCall->location);
        iter_ty = self.instantiate(
          scope,
          iter_ty,
          expr_call.base.base.location,
          TxnLog::empty(),
        );
      }
    } else {
      // iterTy = instantiate(scope, checkExpr(scope, *firstValue).type, firstValue->location);
      let checked = self
        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(scope, first_value_ref, None, false)
        .r#type;
      iter_ty = self.instantiate(
        scope,
        checked,
        first_value_ref.base.location,
        TxnLog::empty(),
      );
    }

    // iterTy = stripFromNilAndReport(iterTy, firstValue->location);
    iter_ty = self.strip_from_nil_and_report(iter_ty, &first_value_ref.base.location);

    // if (std::optional<TypeId> iterMM = findMetatableEntry(iterTy, "__iter", firstValue->location, /* addErrors= */ true))
    if self
      .find_metatable_entry(
        iter_ty,
        String::from("__iter"),
        &first_value_ref.base.location,
        true,
      )
      .is_some()
    {
      // if __iter metamethod is present, it will be called and the results are going to be called as if they are functions
      // for (TypeId var : varTypes)
      //     unify(any_type, var, scope, forin.location);
      let any_type = self.any_type;
      for &var in &var_types {
        self.unify_type_id_type_id_scope_ptr_location(
          any_type,
          var,
          scope,
          &forin.base.base.location,
        );
      }

      // return check(loopScope, *forin.body);
      // SAFETY: forin.body 指向 AST arena 节点。
      return self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });
    }

    // if (const TableType* iterTable = get<TableType>(iterTy))
    if let Some(iter_table) = get_table_type(iter_ty) {
      // if (iterTable->indexer)
      if let Some(indexer) = iter_table.indexer {
        // if (varTypes.size() > 0)
        //     unify(iterTable->indexer->index_type, varTypes[0], scope, forin.location);
        if !var_types.is_empty() {
          self.unify_type_id_type_id_scope_ptr_location(
            indexer.index_type,
            var_types[0],
            scope,
            &forin.base.base.location,
          );
        }

        // if (varTypes.size() > 1)
        //     unify(iterTable->indexer->indexResultType, varTypes[1], scope, forin.location);
        if var_types.len() > 1 {
          self.unify_type_id_type_id_scope_ptr_location(
            indexer.index_result_type,
            var_types[1],
            scope,
            &forin.base.base.location,
          );
        }

        // for (size_t i = 2; i < varTypes.size(); ++i)
        //     unify(nil_type, varTypes[i], scope, forin.location);
        let nil_type = self.nil_type;
        for &var in &var_types[2..] {
          self.unify_type_id_type_id_scope_ptr_location(
            nil_type,
            var,
            scope,
            &forin.base.base.location,
          );
        }
      } else {
        // for (TypeId var : varTypes)
        //     unify(unknown_type, var, scope, forin.location);
        let unknown_type = self.unknown_type;
        for &var in &var_types {
          self.unify_type_id_type_id_scope_ptr_location(
            unknown_type,
            var,
            scope,
            &forin.base.base.location,
          );
        }
      }

      // return check(loopScope, *forin.body);
      // SAFETY: forin.body 指向 AST arena 节点。
      return self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });
    }

    // const FunctionType* iterFunc = get<FunctionType>(iterTy);
    // if (!iterFunc)
    let Some(iter_func) = get_type_id::<FunctionType>(iter_ty) else {
      // TypeId varTy = get<AnyType>(iterTy) ? any_type : errorRecoveryType(loopScope);
      let var_ty = if get_type_id::<AnyType>(iter_ty).is_some() {
        self.any_type
      } else {
        self.error_recovery_type_scope_ptr(&loop_scope)
      };

      // for (TypeId var : varTypes)
      //     unify(varTy, var, scope, forin.location);
      for &var in &var_types {
        self.unify_type_id_type_id_scope_ptr_location(
          var_ty,
          var,
          scope,
          &forin.base.base.location,
        );
      }

      // if (!get<ErrorType>(iterTy) && !get<AnyType>(iterTy) && !get<FreeType>(iterTy) && !get<NeverType>(iterTy))
      //     reportError(firstValue->location, CannotCallNonFunction{iterTy});
      if get_type_id::<ErrorType>(iter_ty).is_none()
        && get_type_id::<AnyType>(iter_ty).is_none()
        && get_type_id::<FreeType>(iter_ty).is_none()
        && get_type_id::<NeverType>(iter_ty).is_none()
      {
        self.report_error_location_type_error_data(
          &first_value_ref.base.location,
          CannotCallNonFunction { ty: iter_ty }.into(),
        );
      }

      // return check(loopScope, *forin.body);
      // SAFETY: forin.body 指向 AST arena 节点。
      return self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });
    };
    // We only need the function's argTypes/retTypes; capture them up front so we
    // can keep mutably borrowing `self` for the remaining unifications.
    let iter_func_arg_types = iter_func.arg_types;
    let iter_func_ret_types = iter_func.ret_types;

    // if (forin.values.size == 1)
    if forin.values.size == 1 {
      // TypePackId argPack = nullptr;
      let arg_pack: TypePackId;

      // if (firstValue->is<AstExprCall>())
      if first_value_call.is_some() {
        // Extract the remaining return values of the call
        // auto [types, tail] = flatten(callRetPack);
        let (types, tail) = flatten_type_pack_id(call_ret_pack);

        if !types.is_empty() {
          // std::vector<TypeId> argTypes = std::vector<TypeId>(types.begin() + 1, types.end());
          // argPack = addTypePack(TypePackVar{TypePack{std::move(argTypes), tail}});
          let arg_types: Vec<TypeId> = types[1..].to_vec();
          arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
            head: arg_types,
            tail,
          }));
        } else {
          // argPack = addTypePack(TypePack{});
          arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
            head: Vec::new(),
            tail: None,
          }));
        }
      } else {
        // Check if iterator function accepts 0 arguments
        // argPack = addTypePack(TypePack{});
        arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
          head: Vec::new(),
          tail: None,
        }));
      }

      // Unifier state = mkUnifier(loopScope, firstValue->location);
      let mut state = self.mk_unifier(&loop_scope, &first_value_ref.base.location);

      // checkArgumentList(loopScope, *firstValue, state, argPack, iterFunc->argTypes, /*argLocations*/ {});
      self.check_argument_list(
        &loop_scope,
        first_value_ref,
        &mut state,
        arg_pack,
        iter_func_arg_types,
        &Vec::new(),
      );

      // state.log.commit();
      state.log.commit();

      // reportErrors(state.errors);
      let state_errors = state.errors.clone();
      self.report_errors(&state_errors);
    }

    // TypePackId retPack = iterFunc->retTypes;
    let mut ret_pack: TypePackId = iter_func_ret_types;

    // if (forin.values.size >= 2)
    if values_slice.len() >= 2 {
      // AstArray<AstExpr*> arguments{forin.values.data + 1, forin.values.size - 1};
      let arguments = AstArray::<*mut AstExpr> {
        // SAFETY: values_slice 至少 2 个元素，[1..] 对应 size-1 个有效。
        data: values_slice[1..].as_ptr() as *mut *mut AstExpr,
        size: values_slice.len() - 1,
      };

      // Position start = firstValue->location.begin;
      // Position end = values[forin.values.size - 1]->location.end;
      let start: Position = first_value_ref.base.location.begin;
      // SAFETY: 数组末元素指向 AST arena 节点。
      let end: Position = unsafe { (*values_slice[values_slice.len() - 1]).base.location.end };

      // AstExprCall exprCall{Location(start, end), firstValue, arguments, /* self= */ false, AstArray<AstTypeOrPack>{}, Location()};
      let expr_call = AstExprCall::new(
        Location::new(start, end),
        first_value,
        arguments,
        false,
        AstArray::<AstTypeOrPack> {
          data: null_mut(),
          size: 0,
        },
        Location::default(),
      );

      // retPack = checkExprPack(scope, exprCall).type;
      // 栈上合成节点（C++ 同为栈对象）；repr(C) base 偏移 0，&base 与原 cast 写法指针值一致。
      ret_pack = self.check_expr_pack(scope, &expr_call.base).r#type;
    }

    // We need to remove 'nil' from the set of options of the first return value
    // if (std::optional<TypeId> fty = first(retPack); fty && !varTypes.empty())
    if let Some(fty) = first(ret_pack, true)
      && !var_types.is_empty()
    {
      // TypeId keyTy = follow(*fty);
      let mut key_ty = follow_type_id(fty);

      // if (get<UnionType>(keyTy))
      //     if (std::optional<TypeId> ty = tryStripUnionFromNil(keyTy)) keyTy = *ty;
      if get_type_id::<UnionType>(key_ty).is_some()
        && let Some(stripped) = self.try_strip_union_from_nil(key_ty)
      {
        key_ty = stripped;
      }

      // unify(keyTy, varTypes.front(), scope, forin.location);
      self.unify_type_id_type_id_scope_ptr_location(
        key_ty,
        var_types[0],
        scope,
        &forin.base.base.location,
      );

      // We have already handled the first variable type, make it match in the pack check
      // varTypes.front() = *fty;
      var_types[0] = fty;
    }

    // TypePackId varPack = addTypePack(TypePackVar{TypePack{std::move(varTypes), freshTypePack(scope)}});
    let fresh_var_tail = self.fresh_type_pack_scope_ptr(scope.clone());
    let var_pack =
      self.add_type_pack_vector_type_id_optional_type_pack_id(&var_types, Some(fresh_var_tail));

    // unify(retPack, varPack, scope, forin.location);
    self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
      ret_pack,
      var_pack,
      scope,
      &forin.base.base.location,
      CountMismatchContext::Arg,
    );

    // check(loopScope, *forin.body);
    // SAFETY: forin.body 指向 AST arena 节点。
    self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });

    // return ControlFlow::None;
    ControlFlow::None
  }
}
