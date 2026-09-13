//! `TypeChecker2::visit(AstStatForIn*)`（TypeChecker2.cpp:893-1056）。
use ulua_ast::records::{
  ast_expr::AstExpr, ast_node::AstNode, ast_stat_for_in::AstStatForIn, location::Location,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::value_context::ValueContext,
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry,
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_parameter_extents::get_parameter_extents,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id, instantiate::instantiate,
    is_optional::is_optional,
  },
  records::{
    any_type::AnyType,
    cannot_call_non_function::CannotCallNonFunction,
    count_mismatch::{CountMismatch, CountMismatchContext},
    error_type::ErrorType,
    function_type::FunctionType,
    generic_error::GenericError,
    instantiation::Instantiation,
    never_type::NeverType,
    normalization_too_complex::NormalizationTooComplex,
    optional_value_access::OptionalValueAccess,
    symbol::Symbol,
    table_type::TableType,
    txn_log::TxnLog,
    type_arena::TypeArena,
    type_checker_2::TypeChecker2,
    type_level::TypeLevel,
    type_pack::TypePack,
    unification_too_complex::UnificationTooComplex,
    union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl TypeChecker2 {
  pub fn visit_ast_stat_for_in(&mut self, for_in_statement: &AstStatForIn) {
    let vars = for_in_statement.vars;
    let values = for_in_statement.values;
    let body = for_in_statement.body;

    // for (AstLocal* local : forInStatement->vars)
    //     if (local->annotation)
    //         visit(local->annotation);
    let vars_slice = vars.as_slice();
    let values_slice = values.as_slice();
    for &local in vars_slice {
      // SAFETY: local 指向 AST arena 节点。
      if !local.is_null() && !unsafe { (*local).annotation.is_null() } {
        // SAFETY: 同上。
        unsafe { self.visit_ast_type((*local).annotation) };
      }
    }

    // for (AstExpr* expr : forInStatement->values)
    //     visit(expr, ValueContext::RValue);
    for &expr in values_slice {
      if !expr.is_null() {
        self.visit_ast_expr_value_context(expr, ValueContext::RValue);
      }
    }

    // visit(forInStatement->body);
    if !body.is_null() {
      self.visit_ast_stat_block(body);
    }

    // if (!forInStatement->vars.size || !forInStatement->values.size) return;
    if vars_slice.is_empty() || values_slice.is_empty() {
      return;
    }

    // NotNull<Scope> scope = stack.back();
    let scope = *self.stack.last().unwrap();

    // std::vector<TypeId> variableTypes;
    // for (AstLocal* var : forInStatement->vars)
    // {
    //     std::optional<TypeId> ty = scope->lookup(var);
    //     LUAU_ASSERT(ty);
    //     variableTypes.emplace_back(*ty);
    // }
    let mut variable_types: Vec<TypeId> = Vec::new();
    for &var in vars_slice {
      // SAFETY: scope 来自 stack.back()，与 visit 会话同寿（C++ 同契约）。
      let ty = unsafe { (*scope).lookup_symbol(Symbol::from_local(var)) };
      LUAU_ASSERT!(ty.is_some());
      variable_types.push(ty.unwrap());
    }

    // AstExpr* firstValue = forInStatement->values.data[0];
    let first_value = values_slice[0];

    // std::vector<TypeId> valueTypes;
    // std::optional<TypePackId> iteratorTail;
    let mut value_types: Vec<TypeId> = Vec::new();
    let mut iterator_tail: Option<TypePackId> = None;

    // TypePackId* retPack = module->astTypePacks.find(firstValue);
    // if (retPack) { auto [head, tail] = flatten(*retPack); valueTypes = head; iteratorTail = tail; }
    // else valueTypes.emplace_back(lookupType(firstValue));
    // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
    let ret_pack = unsafe { &*self.module }
      .ast_type_packs
      .find(&(first_value as *const AstExpr))
      .copied();
    if let Some(ret_pack) = ret_pack {
      let (head, tail) = flatten_type_pack_id(ret_pack);
      value_types = head;
      iterator_tail = tail;
    } else {
      // SAFETY: first_value 指向 AST arena 节点。
      let ty = self.lookup_type(unsafe { &*first_value });
      value_types.push(ty);
    }

    // TypeId* resolvedTy = module->astForInNextTypes.find(firstValue);
    // if (resolvedTy && (!retPack || valueTypes.size() > 1))
    //     valueTypes[0] = *resolvedTy;
    // SAFETY: 同上。
    let resolved_ty = unsafe { &*self.module }
      .ast_for_in_next_types
      .find(&(first_value as *const AstNode))
      .copied();
    if let Some(resolved_ty) = resolved_ty
      && (ret_pack.is_none() || value_types.len() > 1)
    {
      value_types[0] = resolved_ty;
    }

    if values_slice.len() > 2 {
      for &expr in &values_slice[1..values_slice.len() - 1] {
        // SAFETY: expr 指向 AST arena 节点。
        let ty = self.lookup_type(unsafe { &*expr });
        value_types.push(ty);
      }
    }

    // if (forInStatement->values.size > 1)
    // {
    //     auto [head, tail] = flatten(lookupPack(forInStatement->values.data[size - 1]));
    //     valueTypes.insert(end, head); iteratorTail = tail;
    // }
    if values_slice.len() > 1 {
      let last_expr = values_slice[values_slice.len() - 1];
      let pack = self.lookup_pack(last_expr);
      let (head, tail) = flatten_type_pack_id(pack);
      value_types.extend(head);
      iterator_tail = tail;
    }

    // TypePackId iteratorPack = arena.addTypePack(std::move(valueTypes), iteratorTail);
    // SAFETY: 同上。
    let iterator_pack = unsafe {
      let arena = &mut (*self.module).internal_types;
      arena.add_type_pack_vector_type_id_optional_type_pack_id(value_types, iterator_tail)
    };

    // TypePack iteratorTypes = extendTypePack(arena, builtinTypes, iteratorPack, 3);
    // SAFETY: 同上。
    let iterator_types = unsafe {
      let arena = &mut (*self.module).internal_types;
      extend_type_pack(arena, self.builtin_types, iterator_pack, 3, Vec::new())
    };

    // if (iteratorTypes.head.empty())
    // {
    //     reportError(GenericError{"..."}, getLocation(forInStatement->values));
    //     return;
    // }
    if iterator_types.head.is_empty() {
      let loc = Self::values_location(for_in_statement);
      self.report_error_type_error_data_location(
        GenericError::new(
          "for..in loops require at least one value to iterate over.  Got zero".to_string(),
        )
        .into(),
        &loc,
      );
      return;
    }

    // TypeId iteratorTy = follow(iteratorTypes.head[0]);
    let iterator_ty = follow_type_id(iterator_types.head[0]);

    // std::shared_ptr<const NormalizedType> iteratorNorm = normalizer.normalize(iteratorTy);
    let iterator_norm = self.normalizer.normalize(iterator_ty);

    // (The Rust normalize() always returns a valid Arc, so the `if (!iteratorNorm)`
    // branch reporting NormalizationTooComplex is dead; transcribed faithfully but
    // the condition can never be true.)
    let _ = NormalizationTooComplex::default;

    if let Some(next_fn) = get_type_id::<FunctionType>(iterator_ty) {
      // const FunctionType* nextFn = get<FunctionType>(iteratorTy);
      // checkFunction(nextFn, iteratorTypes.head, false);
      self.check_function_for_in(
        next_fn,
        iterator_types.head.clone(),
        false,
        for_in_statement,
        first_value,
        &variable_types,
      );
    } else if let Some(ttv) = get_type_id::<TableType>(iterator_ty) {
      // if ((vars.size == 1 || vars.size == 2) && ttv->indexer)
      if (vars.size == 1 || vars.size == 2)
        && let Some(indexer) = ttv.indexer.as_ref()
      {
        // testIsSubtype(variableTypes[0], ttv->indexer->index_type, vars.data[0]->location);
        // SAFETY: vars 元素指向 AST arena 节点。
        let v0_loc = unsafe { (*vars_slice[0]).location };
        self.test_is_subtype_type_id_type_id_location(
          variable_types[0],
          indexer.index_type,
          v0_loc,
        );
        if variable_types.len() == 2 {
          // SAFETY: 同上。
          let v1_loc = unsafe { (*vars_slice[1]).location };
          self.test_is_subtype_type_id_type_id_location(
            variable_types[1],
            indexer.index_result_type,
            v1_loc,
          );
        }
      } else {
        // reportError(GenericError{"..."}, forInStatement->values.data[0]->location);
        // SAFETY: first_value 指向 AST arena 节点。
        let loc = unsafe { (*first_value).base.location };
        self.report_error_type_error_data_location(
          GenericError::new("Cannot iterate over a table without indexer".to_string()).into(),
          &loc,
        );
      }
    } else if get_type_id::<AnyType>(iterator_ty).is_some()
      || get_type_id::<ErrorType>(iterator_ty).is_some()
      || get_type_id::<NeverType>(iterator_ty).is_some()
    {
      // nothing
    } else if is_optional(iterator_ty) && !iterator_norm.should_suppress_errors() {
      // reportError(OptionalValueAccess{iteratorTy}, forInStatement->values.data[0]->location);
      // SAFETY: first_value 指向 AST arena 节点。
      let loc = unsafe { (*first_value).base.location };
      self.report_error_type_error_data_location(
        OptionalValueAccess {
          optional: iterator_ty,
        }
        .into(),
        &loc,
      );
    } else if let Some(iter_mm_ty) = unsafe {
      find_metatable_entry(
        self.builtin_types,
        // SAFETY: self.module 同上。
        &mut (*self.module).errors,
        iterator_ty,
        "__iter",
        // SAFETY: first_value 指向 AST arena 节点。
        (*first_value).base.location,
      )
    } {
      // Instantiation instantiation{TxnLog::empty(), &arena, builtinTypes, TypeLevel{}, scope};
      // SAFETY: 同上。
      let mut instantiation = unsafe {
        Instantiation::instantiation_new(
          TxnLog::empty(),
          &mut (*self.module).internal_types as *mut TypeArena,
          self.builtin_types,
          TypeLevel::default(),
          scope,
        )
      };

      // if (std::optional<TypeId> instantiatedIterMmTy = instantiate(builtinTypes, NotNull{&arena}, limits, scope, *iterMmTy))
      // SAFETY: 同上。
      // SAFETY: self.builtin_types / self.limits / self.module 由 TypeChecker2 生命周期保证有效
      if let Some(instantiated_iter_mm_ty) = instantiate(
        unsafe { &*self.builtin_types },
        unsafe { &mut (*self.module).internal_types },
        unsafe { &*self.limits },
        scope,
        iter_mm_ty,
      ) {
        // if (const FunctionType* iterMmFtv = get<FunctionType>(*instantiatedIterMmTy))
        if let Some(iter_mm_ftv) = get_type_id::<FunctionType>(instantiated_iter_mm_ty) {
          let iter_mm_arg_types = iter_mm_ftv.arg_types;
          let iter_mm_ret_types = iter_mm_ftv.ret_types;

          // TypePackId argPack = arena.addTypePack({iteratorTy});
          // SAFETY: 同上。
          let arg_pack = unsafe {
            let arena = &mut (*self.module).internal_types;
            arena.add_type_pack_vector_type_id_optional_type_pack_id(vec![iterator_ty], None)
          };
          // testIsSubtype(argPack, iterMmFtv->argTypes, forInStatement->values.data[0]->location);
          self.test_is_subtype_type_pack_id_type_pack_id_location(
            arg_pack,
            iter_mm_arg_types,
            // SAFETY: first_value 指向 AST arena 节点。
            unsafe { (*first_value).base.location },
          );

          // TypePack mmIteratorTypes = extendTypePack(arena, builtinTypes, iterMmFtv->retTypes, 3);
          // SAFETY: 同上。
          let mm_iterator_types = unsafe {
            let arena = &mut (*self.module).internal_types;
            extend_type_pack(arena, self.builtin_types, iter_mm_ret_types, 3, Vec::new())
          };

          // if (mmIteratorTypes.head.size() == 0)
          if mm_iterator_types.head.is_empty() {
            // SAFETY: 同上。
            let loc = unsafe { (*first_value).base.location };
            self.report_error_type_error_data_location(
              GenericError::new("__iter must return at least one value".to_string()).into(),
              &loc,
            );
            return;
          }

          // TypeId nextFn = follow(mmIteratorTypes.head[0]);
          let next_fn = follow_type_id(mm_iterator_types.head[0]);

          // if (std::optional<TypeId> instantiatedNextFn = instantiation.substitute(nextFn))
          if let Some(instantiated_next_fn) = instantiation.base.substitute_type_id(next_fn) {
            // std::vector<TypeId> instantiatedIteratorTypes = mmIteratorTypes.head;
            // instantiatedIteratorTypes[0] = *instantiatedNextFn;
            let mut instantiated_iterator_types = mm_iterator_types.head;
            instantiated_iterator_types[0] = instantiated_next_fn;

            // if (const FunctionType* nextFtv = get<FunctionType>(*instantiatedNextFn))
            if let Some(next_ftv) = get_type_id::<FunctionType>(instantiated_next_fn) {
              self.check_function_for_in(
                next_ftv,
                instantiated_iterator_types,
                true,
                for_in_statement,
                first_value,
                &variable_types,
              );
            } else if !self.is_error_suppressing_location_type_id(
              // SAFETY: first_value 指向 AST arena 节点。
              unsafe { (*first_value).base.location },
              instantiated_next_fn,
            ) {
              // SAFETY: 同上。
              let loc = unsafe { (*first_value).base.location };
              self.report_error_type_error_data_location(
                CannotCallNonFunction {
                  ty: instantiated_next_fn,
                }
                .into(),
                &loc,
              );
            }
          } else {
            // reportError(UnificationTooComplex{}, forInStatement->values.data[0]->location);
            // SAFETY: 同上。
            let loc = unsafe { (*first_value).base.location };
            self
              .report_error_type_error_data_location(UnificationTooComplex::default().into(), &loc);
          }
        } else if !self.is_error_suppressing_location_type_id(
          // SAFETY: 同上。
          unsafe { (*first_value).base.location },
          instantiated_iter_mm_ty,
        ) {
          // reportError(CannotCallNonFunction{*iterMmTy}, forInStatement->values.data[0]->location);
          // SAFETY: 同上。
          let loc = unsafe { (*first_value).base.location };
          self.report_error_type_error_data_location(
            CannotCallNonFunction { ty: iter_mm_ty }.into(),
            &loc,
          );
        }
      } else {
        // reportError(UnificationTooComplex{}, forInStatement->values.data[0]->location);
        // SAFETY: 同上。
        let loc = unsafe { (*first_value).base.location };
        self.report_error_type_error_data_location(UnificationTooComplex::default().into(), &loc);
      }
    } else if iterator_norm.has_tables() {
      // Ok. All tables can be iterated.
    } else if !iterator_norm.should_suppress_errors() {
      // reportError(CannotCallNonFunction{iteratorTy}, forInStatement->values.data[0]->location);
      // SAFETY: 同上。
      let loc = unsafe { (*first_value).base.location };
      self.report_error_type_error_data_location(
        CannotCallNonFunction { ty: iterator_ty }.into(),
        &loc,
      );
    }
  }

  /// C++ `getLocation(forInStatement->values)` — span from the first value's begin
  /// to the last value's end (`Ast.h:1715`). Returns the all-zero span when empty.
  fn values_location(for_in_statement: &AstStatForIn) -> Location {
    let slice = for_in_statement.values.as_slice();
    match (slice.first(), slice.last()) {
      // SAFETY: slice 元素指向 AST arena 节点。
      (Some(&first), Some(&last)) => unsafe {
        Location::new((*first).base.location.begin, (*last).base.location.end)
      },
      _ => Location::default(),
    }
  }

  /// C++ lambda `checkFunction` captured by the `visit(AstStatForIn*)` method
  /// (`TypeChecker2.cpp:967-1056`). Lifted to a real `&mut self` method since the
  /// lambda captures `this`, `&arena`, `&forInStatement` and `&variableTypes`.
  fn check_function_for_in(
    &mut self,
    iter_ftv: &FunctionType,
    iter_tys: Vec<TypeId>,
    is_mm: bool,
    for_in_statement: &AstStatForIn,
    first_value: *mut AstExpr,
    variable_types: &[TypeId],
  ) {
    let iter_ftv_arg_types = iter_ftv.arg_types;
    let iter_ftv_ret_types = iter_ftv.ret_types;

    // if (iterTys.size() < 1 || iterTys.size() > 3)
    if iter_tys.is_empty() || iter_tys.len() > 3 {
      let loc = Self::values_location(for_in_statement);
      self.report_error_type_error_data_location(
        GenericError::new(
          if is_mm {
            "__iter metamethod must return (next[, table[, state]])"
          } else {
            "for..in loops must be passed (next[, table[, state]])"
          }
          .to_string(),
        )
        .into(),
        &loc,
      );
      return;
    }

    // TypePack expectedVariableTypes = extendTypePack(arena, builtinTypes, iterFtv->retTypes, variableTypes.size());
    // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
    let expected_variable_types = unsafe {
      let arena = &mut (*self.module).internal_types;
      extend_type_pack(
        arena,
        self.builtin_types,
        iter_ftv_ret_types,
        variable_types.len(),
        Vec::new(),
      )
    };
    // if (expectedVariableTypes.head.size() < variableTypes.size())
    if expected_variable_types.head.len() < variable_types.len() {
      if is_mm {
        let loc = Self::values_location(for_in_statement);
        self.report_error_type_error_data_location(
          GenericError::new(
            "__iter metamethod's next() function does not return enough values".to_string(),
          )
          .into(),
          &loc,
        );
      } else {
        // reportError(GenericError{...}, forInStatement->values.data[0]->location);
        // SAFETY: first_value 指向 AST arena 节点。
        let loc = unsafe { (*first_value).base.location };
        self.report_error_type_error_data_location(
          GenericError::new("next() does not return enough values".to_string()).into(),
          &loc,
        );
      }
      return;
    }

    // if (get<ErrorType>(follow(flattenPack(iterFtv->argTypes)))) return;
    let flattened = self.flatten_pack(iter_ftv_arg_types);
    if get_type_id::<ErrorType>(follow_type_id(flattened)).is_some() {
      return;
    }

    // auto [minCount, maxCount] = getParameterExtents(TxnLog::empty(), iterFtv->argTypes, true);
    let (min_count, _max_count) =
      unsafe { get_parameter_extents(TxnLog::empty(), iter_ftv_arg_types, true) };

    // TypePack flattenedArgTypes = extendTypePack(arena, builtinTypes, iterFtv->argTypes, 2);
    // SAFETY: 同上。
    let _flattened_arg_types = unsafe {
      let arena = &mut (*self.module).internal_types;
      extend_type_pack(arena, self.builtin_types, iter_ftv_arg_types, 2, Vec::new())
    };
    // size_t firstIterationArgCount = iterTys.empty() ? 0 : iterTys.size() - 1;
    let first_iteration_arg_count = iter_tys.len().saturating_sub(1);
    // size_t actualArgCount = expectedVariableTypes.head.size();
    let actual_arg_count = expected_variable_types.head.len();

    // if (firstIterationArgCount < minCount)
    if first_iteration_arg_count < min_count {
      if is_mm {
        let loc = Self::values_location(for_in_statement);
        self.report_error_type_error_data_location(
          GenericError::new("__iter metamethod must return (next[, table[, state]])".to_string())
            .into(),
          &loc,
        );
      } else {
        // SAFETY: first_value 指向 AST arena 节点。
        let loc = unsafe { (*first_value).base.location };
        self.report_error_type_error_data_location(
          CountMismatch {
            expected: 2,
            maximum: None,
            actual: first_iteration_arg_count,
            context: CountMismatchContext::Arg,
            is_variadic: false,
            function: String::new(),
          }
          .into(),
          &loc,
        );
      }
      return;
    } else if actual_arg_count < min_count {
      if is_mm {
        let loc = Self::values_location(for_in_statement);
        self.report_error_type_error_data_location(
          GenericError::new("__iter metamethod must return (next[, table[, state]])".to_string())
            .into(),
          &loc,
        );
      } else {
        // SAFETY: 同上。
        let loc = unsafe { (*first_value).base.location };
        self.report_error_type_error_data_location(
          CountMismatch {
            expected: 2,
            maximum: None,
            actual: first_iteration_arg_count,
            context: CountMismatchContext::Arg,
            is_variadic: false,
            function: String::new(),
          }
          .into(),
          &loc,
        );
      }
      return;
    }

    // const TypeId iterFunc = follow(iterTys[0]);
    let iter_func = follow_type_id(iter_tys[0]);

    // std::vector<TypeId> prospectiveArgTypes = std::vector(iterTys.begin() + 1, iterTys.end());
    let mut prospective_arg_types: Vec<TypeId> = iter_tys[1..].to_vec();
    // if (const TypePack* iterFuncArgs = get<TypePack>(follow(iterFtv->argTypes));
    //     iterFuncArgs && iterFuncArgs->head.size() > prospectiveArgTypes.size())
    //     prospectiveArgTypes.resize(iterFuncArgs->head.size(), builtinTypes->nil_type);
    if let Some(iter_func_args) =
      get_type_pack_id::<TypePack>(unsafe { follow_type_pack_id(iter_ftv_arg_types) })
      && iter_func_args.head.len() > prospective_arg_types.len()
    {
      // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
      prospective_arg_types.resize(iter_func_args.head.len(), unsafe {
        (*self.builtin_types).nil_type
      });
    }
    // const TypePackId prospectiveArgs = arena.addTypePack(prospectiveArgTypes, std::nullopt);
    // SAFETY: self.module 同上。
    let prospective_args = unsafe {
      let arena = &mut (*self.module).internal_types;
      arena.add_type_pack_vector_type_id_optional_type_pack_id(prospective_arg_types, None)
    };

    // std::vector<TypeId> prospectiveRetTypes = {};
    let mut prospective_ret_types: Vec<TypeId> = Vec::new();
    // if (variableTypes.size() > 0)
    //     prospectiveRetTypes.emplace_back(arena.addType(UnionType{{variableTypes[0], builtinTypes->nil_type}}));
    if !variable_types.is_empty() {
      // SAFETY: 同上。
      let added = unsafe {
        let arena = &mut (*self.module).internal_types;
        arena.add_type(UnionType {
          options: vec![variable_types[0], (*self.builtin_types).nil_type],
        })
      };
      prospective_ret_types.push(added);
    }
    // if (variableTypes.size() > 1) prospectiveRetTypes.emplace_back(variableTypes[1]);
    if variable_types.len() > 1 {
      prospective_ret_types.push(variable_types[1]);
    }
    // if (const TypePack* iterFuncRets = get<TypePack>(follow(iterFtv->retTypes));
    //     iterFuncRets && iterFuncRets->head.size() > prospectiveRetTypes.size())
    //     prospectiveRetTypes.resize(iterFuncRets->head.size(), builtinTypes->any_type);
    if let Some(iter_func_rets) =
      get_type_pack_id::<TypePack>(unsafe { follow_type_pack_id(iter_ftv_ret_types) })
      && iter_func_rets.head.len() > prospective_ret_types.len()
    {
      // SAFETY: 同上。
      prospective_ret_types.resize(iter_func_rets.head.len(), unsafe {
        (*self.builtin_types).any_type
      });
    }
    // const TypePackId prospectiveRets = arena.addTypePack(prospectiveRetTypes, builtinTypes->any_type_pack);
    // SAFETY: 同上。
    let any_type_pack = unsafe { (*self.builtin_types).any_type_pack };
    let prospective_rets = unsafe {
      let arena = &mut (*self.module).internal_types;
      arena.add_type_pack_vector_type_id_optional_type_pack_id(
        prospective_ret_types,
        Some(any_type_pack),
      )
    };

    // const TypeId prospectiveFunction = arena.addType(FunctionType{prospectiveArgs, prospectiveRets, std::nullopt, isMm});
    // SAFETY: 同上。
    let prospective_function = unsafe {
      let arena = &mut (*self.module).internal_types;
      arena.add_type(FunctionType::function_type_new(
        prospective_args,
        prospective_rets,
        None,
        is_mm,
      ))
    };

    // testIsSubtypeForInStat(iterFunc, prospectiveFunction, *forInStatement);
    self.test_is_subtype_for_in_stat(iter_func, prospective_function, for_in_statement);
  }
}
