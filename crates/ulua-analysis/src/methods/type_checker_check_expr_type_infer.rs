use alloc::{
  format,
  string::{String, ToString},
  vec::Vec,
};
use core::{
  ptr::{NonNull, null},
  str::from_utf8,
};

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  functions::to_string_ast::to_str,
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_table::ItemKind,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
  },
  rtti::ast_node_try_as_ptr,
};
use ulua_common::{
  fflag, fint, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet,
};

use crate::{
  enums::{table_state::TableState, value_context::ValueContext},
  functions::{
    allows_no_return_values::allows_no_return_values, arc_as_mut::arc_as_mut,
    as_mutable_type_pack::as_mutable_type_pack, begin_type::begin_union_type, first::first,
    follow_type, follow_type_pack, get_mutable_type_pack, get_type, get_type_pack,
    has_length::has_length, maybe_singleton::maybe_singleton, maybe_string::maybe_string,
    reduce_union::reduce_union, to_string_to_string::to_string_type_id,
    try_get_l_value::try_get_l_value, try_get_type_guard_predicate::try_get_type_guard_predicate,
    type_could_have_metatable::type_could_have_metatable,
  },
  methods::type_checker_check_binary_operation::is_any_like,
  records::{
    and_predicate::AndPredicate,
    builtin_types::BuiltinTypes,
    count_mismatch::CountMismatchContext,
    eq_predicate::EqPredicate,
    free_type_pack::FreeTypePack,
    function_type::FunctionType,
    generic_error::GenericError,
    generic_type_pack::GenericTypePack,
    not_a_table::NotATable,
    not_predicate::NotPredicate,
    or_predicate::OrPredicate,
    recursion_counter::RecursionCounter,
    table_type::TableType,
    truthy_predicate::TruthyPredicate,
    type_checker::TypeChecker,
    type_checker_2::TypeChecker2,
    type_error::TypeError,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    types_are_unrelated::TypesAreUnrelated,
    union_type::UnionType,
    unknown_symbol::{Context, UnknownSymbol},
    variadic_type_pack::VariadicTypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, name_type::Name, predicate::Predicate,
    predicate_vec::PredicateVec, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl TypeChecker {
  pub fn check_expr(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
    expected_type: Option<TypeId>,
    force_singleton: bool,
  ) -> WithPredicate<TypeId> {
    // SAFETY: 指针来自对 self 字段 check_recursion_count 的独占 &mut 借用，非空且对齐；
    // RAII 守卫在 self 再次使用前 Drop 并递减计数，无并发别名（C++ `RecursionCounter
    // _rc(&checkRecursionCount)`，TypeInfer.cpp:428 同构）。
    let _rc = RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count);
    let limit = fint::LuauCheckRecursionLimit.get();
    if limit > 0 && self.check_recursion_count >= limit {
      self.report_error_code_too_complex(&expr.base.location);
      return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
    }

    let mut result = match expr.as_expr_ref() {
      AstExprRef::Group(group) => {
        self.check_expr(
          scope,
          // expr 已句柄化：get() 只读借用出自存活 &AstExpr（AST arena 节点）。
          group.expr.get(),
          expected_type,
          false,
        )
      }
      AstExprRef::ConstantNil(_) => WithPredicate::with_predicate_t(self.nil_type),
      AstExprRef::ConstantBool(bool_expr) => {
        let use_singleton = force_singleton || expected_type.is_some_and(maybe_singleton);
        WithPredicate::with_predicate_t(if use_singleton {
          self.singleton_type_bool(bool_expr.value)
        } else {
          self.boolean_type
        })
      }
      AstExprRef::ConstantString(string_expr) => {
        let use_singleton = force_singleton || expected_type.is_some_and(maybe_singleton);
        if use_singleton {
          let bytes = string_expr.value.as_bytes();
          WithPredicate::with_predicate_t(
            self.singleton_type_string(String::from_utf8_lossy(bytes).into_owned()),
          )
        } else {
          WithPredicate::with_predicate_t(self.string_type)
        }
      }
      AstExprRef::ConstantNumber(_) => WithPredicate::with_predicate_t(self.number_type),
      AstExprRef::ConstantInteger(_) => WithPredicate::with_predicate_t(self.integer_type),
      AstExprRef::Local(local_expr) => {
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
            // local 槽已句柄化恒非空（parser 为每个 local 引用在 AstLocal 池分配节点）；
            // 此处仅只读 name（null 名由 as_str_or_empty 按 "" 处理）。
            let name = local_expr.local.get().name.as_str_or_empty().to_string();
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
      }
      AstExprRef::Global(global_expr) => self.check_expr_global(scope, global_expr),
      AstExprRef::Varargs(varargs_expr) => {
        // SAFETY: follow_type_pack_id 为 unsafe 函数，pack 句柄有效。
        let vararg_pack =
          follow_type_pack::follow(self.check_expr_pack(scope, &varargs_expr.base).r#type);

        if get_type_pack::get::<TypePack>(vararg_pack).is_some() {
          WithPredicate::with_predicate_t(first(vararg_pack, false).unwrap_or(self.nil_type))
        } else if get_type_pack::get::<FreeTypePack>(vararg_pack).is_some() {
          let head = self.fresh_type_scope_ptr(scope.clone());
          let tail = self.fresh_type_pack_scope_ptr(scope);
          if let Some(pack) = get_mutable_type_pack::get_mutable::<TypePack>(vararg_pack) {
            *pack = TypePack::new(alloc::vec![head], Some(tail));
          } else {
            // SAFETY:  as_mutable_type_pack 去 const（C++ asMutable 同义），句柄有效。
            unsafe {
              *as_mutable_type_pack(vararg_pack) =
                TypePackVar::from(TypePack::new(alloc::vec![head], Some(tail)));
            }
          }
          WithPredicate::with_predicate_t(head)
        } else if get_type_pack::get::<ErrorTypePack>(vararg_pack).is_some() {
          WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
        } else if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(vararg_pack) {
          WithPredicate::with_predicate_t(vtp.ty)
        } else if get_type_pack::get::<GenericTypePack>(vararg_pack).is_some() {
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
      }
      AstExprRef::Call(call_expr) => {
        let pack_result = self.check_expr_pack(scope, &call_expr.base);
        // SAFETY: follow_type_pack_id 为 unsafe 函数，pack 句柄有效。
        let ret_pack = follow_type_pack::follow(pack_result.r#type);

        if let Some(pack) = get_type_pack::get::<TypePack>(ret_pack) {
          WithPredicate::with_predicate_t_predicate_vec(
            pack.head.first().copied().unwrap_or(self.nil_type),
            pack_result.predicates,
          )
        } else if get_type_pack::get::<FreeTypePack>(ret_pack).is_some() {
          let head = self.fresh_type_type_level(scope.level);
          let tail = self.fresh_type_pack_type_level(scope.level);
          let pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::new(
            alloc::vec![head],
            Some(tail),
          )));
          self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
            pack,
            ret_pack,
            scope,
            &call_expr.base.base.location,
            CountMismatchContext::Arg,
          );
          WithPredicate::with_predicate_t_predicate_vec(head, pack_result.predicates)
        } else if get_type_pack::get::<ErrorTypePack>(ret_pack).is_some() {
          WithPredicate::with_predicate_t_predicate_vec(
            self.error_recovery_type_scope_ptr(scope),
            pack_result.predicates,
          )
        } else if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(ret_pack) {
          WithPredicate::with_predicate_t_predicate_vec(vtp.ty, pack_result.predicates)
        } else if get_type_pack::get::<GenericTypePack>(ret_pack).is_some() {
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
      }
      AstExprRef::IndexName(index_name) => self.check_expr_index_name(scope, index_name),
      AstExprRef::IndexExpr(index_expr) => {
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
      }
      AstExprRef::Function(function) => self.check_expr_function(scope, function, expected_type),
      AstExprRef::Table(table_expr) => {
        // SAFETY: 与函数入口同法——&mut 临时借用物化为 self.check_recursion_count 的裸指针，
        // 非空/对齐；_table_rc 存活期间不再产生对该字段的其他借用，Drop 时恢复计数。
        let _table_rc = RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count);
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
          let followed = follow_type::follow(expected_type);
          if let Some(ttv) = get_type::get::<TableType>(followed) {
            if ttv.state == TableState::Sealed {
              expected_table = Some(ttv);

              if let Some(indexer) = &ttv.indexer {
                expected_index_type = Some(indexer.index_type);
                expected_index_result_type = Some(indexer.index_result_type);
              }
            }
          } else if let Some(utv) = get_type::get::<UnionType>(followed) {
            expected_union = Some(utv);
          }
        }

        for item in table_expr.items.iter() {
          let mut expected_result_type: Option<TypeId> = None;
          let mut is_indexed_item = false;

          if item.kind == ItemKind::List {
            expected_result_type = expected_index_result_type;
            is_indexed_item = true;
          } else if matches!(item.kind, ItemKind::Record | ItemKind::General) {
            if !item.key.is_null() {
              // 判型+下转合并为一次具名 unsafe 操作（指针版 RTTI 门面，null/不匹配
              // 折叠为 None），替代原先手写的 cast 解引用 + ast_node_try_as 两步。
              let key = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(item.key) };
              if let Some(key) = key {
                let key_str = from_utf8(key.value.as_bytes()).unwrap_or("");

                if let Some(expected_table) = expected_table {
                  if let Some(prop) = expected_table.props.get(key_str) {
                    expected_result_type = Some(prop.type_deprecated());
                  } else if expected_index_type.is_some_and(maybe_string) {
                    expected_result_type = expected_index_result_type;
                  }
                } else if let Some(expected_union) = expected_union {
                  let mut expected_result_types = Vec::new();

                  // C++ `for (TypeId expectedOption : expectedUnion)`——
                  // UnionTypeIterator 展平嵌套 union 并 follow，裸遍历 options
                  // 会漏掉嵌套成员。
                  for expected_option in begin_union_type(expected_union) {
                    let Some(ttv) =
                      get_type::get::<TableType>(follow_type::follow(expected_option))
                    else {
                      continue;
                    };

                    if let Some(prop) = ttv.props.get(key_str) {
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
              .check_expr(
                scope,
                // SAFETY: item.key 非 null，指向 AST arena 节点。
                unsafe { &*item.key },
                expected_index_type,
                false,
              )
              .r#type
          };
          let value_type = self
            .check_expr(
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
      }
      AstExprRef::Unary(unary) => self.check_expr_unary(scope, unary),
      AstExprRef::Binary(binary) => self.check_expr_binary(scope, binary, expected_type),
      AstExprRef::TypeAssertion(type_assertion) => {
        self.check_expr_type_assertion(scope, type_assertion)
      }
      AstExprRef::Error(error_expr) => {
        // SAFETY: current_module 在类型检查期间独占（C++ 直接读改 module->errors 同义）。
        let old_size = unsafe { (*(arc_as_mut(self.expect_current_module()))).errors.len() };
        // expressions 槽位解引用收口 iter_nodes（只读遍历、cpp 同序）。
        for child in error_expr.expressions.iter_nodes() {
          self.check_expr(scope, child, None, false);
        }
        // SAFETY: 同上；子表达式检查可能递归取 module，故此处独立短时访问。
        unsafe {
          (*(arc_as_mut(self.expect_current_module())))
            .errors
            .truncate(old_size);
        }
        WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
      }
      AstExprRef::IfElse(if_else) => self.check_expr_if_else(scope, if_else, expected_type),
      AstExprRef::InterpString(interp_string) => {
        self.check_expr_interp_string(scope, interp_string)
      }
      AstExprRef::Instantiate(instantiate) => self.check_expr_instantiate(scope, instantiate),
    };

    result.r#type = follow_type::follow(result.r#type);

    // SAFETY: current_module 在类型检查期间独占（C++ 直接改 module->astTypes 同义）。
    let module = unsafe { &mut *(arc_as_mut(self.expect_current_module())) };
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

impl TypeChecker2 {
  pub fn allows_no_return_values(&self, tp: TypePackId) -> bool {
    allows_no_return_values(tp)
  }
}

impl TypeChecker {
  pub fn check_expr_global(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprGlobal,
  ) -> WithPredicate<TypeId> {
    let lvalue = try_get_l_value(&expr.base);
    LUAU_ASSERT!(lvalue.is_some());

    if let Some(ty) = self.resolve_l_value_scope_ptr_l_value(
      scope.clone(),
      &lvalue
        .clone()
        .expect("cpp LUAU_ASSERT(lvalue)：本 visit 仅对 lvalue 形态表达式调用"),
    ) {
      let predicate = TruthyPredicate {
        lvalue: lvalue.expect("cpp LUAU_ASSERT(lvalue)：同上，Some 判定在前"),
        location: expr.base.base.location,
      };
      return WithPredicate::with_predicate_t_predicate_vec(
        ty,
        PredicateVec::from(vec![Predicate::Truthy(predicate)]),
      );
    }

    let name_str = expr.name.as_str_or_empty();
    let error_data =
      TypeErrorData::UnknownSymbol(UnknownSymbol::new(name_str.to_string(), Context::Binding));
    let error = TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
    self.report_error_type_error(&error);

    WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
  }
}

pub fn parse_pattern_string_bytes(
  builtin_types: NonNull<BuiltinTypes>,
  data: &[u8],
) -> Vec<TypeId> {
  // SAFETY: NonNull 入参类型保证非空/对齐，调用方（C++ `const BuiltinTypes&` 形参的指针化）
  // 保证其指向本次解析模式串期间存活的 BuiltinTypes；只读访问，无并发写。
  let builtin_types = unsafe { builtin_types.as_ref() };
  let size = data.len();

  let mut result = Vec::new();
  let mut depth = 0;
  let mut parsing_set = false;

  let mut i = 0;
  while i < size {
    let b = data[i];
    if b == b'%' {
      i += 1;
      if !parsing_set && i < size && data[i] == b'b' {
        i += 2;
      }
    } else if !parsing_set && b == b'[' {
      parsing_set = true;
      if i + 1 < size && data[i + 1] == b']' {
        i += 1;
      }
    } else if parsing_set && b == b']' {
      parsing_set = false;
    } else if b == b'(' {
      if !parsing_set {
        if i + 1 < size && data[i + 1] == b')' {
          i += 1;
          result.push(builtin_types.optional_number_type);
        } else {
          depth += 1;
          result.push(builtin_types.optional_string_type);
        }
      }
    } else if b == b')' && !parsing_set {
      depth -= 1;
      if depth < 0 {
        break;
      }
    }
    i += 1;
  }

  if depth != 0 || parsing_set {
    return Vec::new();
  }

  if result.is_empty() {
    result.push(builtin_types.optional_string_type);
  }

  result
}

impl TypeChecker {
  pub fn check_expr_index_name(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIndexName,
  ) -> WithPredicate<TypeId> {
    let name: Name = expr.index.as_str_or_empty().to_string();

    // Redundant call if we find a refined lvalue, but this function must be called in order to recursively populate ast_types.
    let mut lhs_type = self
      .check_expr(
        scope,
        // expr 已句柄化恒非空（C++ 直接解引用 `expr->expr`）：.get() 只读借用。
        expr.expr.get(),
        None,
        false,
      )
      .r#type;

    if let Some(lvalue) = try_get_l_value(&expr.base)
      && let Some(ty) = self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue)
    {
      let predicate = TruthyPredicate {
        lvalue,
        location: expr.base.base.location,
      };
      return WithPredicate::with_predicate_t_predicate_vec(
        ty,
        PredicateVec::from(vec![Predicate::Truthy(predicate)]),
      );
    }

    // 同上——expr.expr 已句柄化恒非空，仅读取其 base.location。
    lhs_type = self.strip_from_nil_and_report(lhs_type, &expr.expr.get().base.location);

    if let Some(ty) = self.get_index_type_from_type(
      scope.clone(),
      lhs_type,
      &name,
      &expr.base.base.location,
      true,
    ) {
      return WithPredicate::with_predicate_t(ty);
    }

    WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
  }

  pub fn check_expr_function(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprFunction,
    expected_type: Option<TypeId>,
  ) -> WithPredicate<TypeId> {
    let (fun_ty, fun_scope) =
      self.check_function_signature(scope, 0, expr, None, None, expected_type);

    self.check_function_body(&fun_scope, fun_ty, expr);

    WithPredicate::with_predicate_t(self.quantify(&fun_scope, fun_ty, expr.base.base.location))
  }

  pub fn check_expr_unary(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprUnary,
  ) -> WithPredicate<TypeId> {
    let boolean_type = self.boolean_type;
    let number_type = self.number_type;
    let nil_type = self.nil_type;

    let result = self.check_expr(
      scope,
      // expr 已句柄化：get() 只读借用出自存活 &AstExpr（AST arena 节点）。
      expr.expr.get(),
      None,
      false,
    );
    let mut operand_type = follow_type::follow(result.r#type);

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
          if let Some(fnt) =
            self.find_metatable_entry(operand_type, "__unm", &expr.base.base.location, true)
          {
            let actual_function_type =
              self.instantiate(scope, fnt, expr.base.base.location, null());
            let arguments = self.add_type_pack_initializer_list_type_id(&[operand_type]);
            let ret_type_pack = self.fresh_type_pack_scope_ptr(scope);
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
              to_str(expr.op),
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

        let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();

        if type_could_have_metatable(operand_type)
          && let Some(fnt) =
            self.find_metatable_entry(operand_type, "__len", &expr.base.base.location, true)
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

  pub fn check_expr_binary(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprBinary,
    expected_type: Option<TypeId>,
  ) -> WithPredicate<TypeId> {
    if expr.op == AstExprBinaryOp::And {
      let lhs = self.check_expr(
        scope,
        // left 已句柄化：get() 只读借用出自存活 &AstExprBinary，无别名冲突。
        expr.left.get(),
        expected_type,
        false,
      );
      let lhs_ty = lhs.r#type;
      let lhs_predicates = lhs.predicates;

      let inner_scope = self.child_scope(scope, &expr.base.base.location);
      self.resolve_predicate_vec_scope_ptr_bool(&lhs_predicates, &inner_scope, true);

      let rhs = self.check_expr(
        &inner_scope,
        // right 已句柄化：get() 只读借用（inner_scope 不影响其存活）。
        expr.right.get(),
        expected_type,
        false,
      );
      let rhs_ty = rhs.r#type;
      let rhs_predicates = rhs.predicates;

      let result_ty =
        self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &PredicateVec::new());
      WithPredicate::with_predicate_t_predicate_vec(
        result_ty,
        PredicateVec::from(alloc::vec![Predicate::And(AndPredicate {
          lhs: lhs_predicates,
          rhs: rhs_predicates,
        })]),
      )
    } else if expr.op == AstExprBinaryOp::Or {
      let lhs = self.check_expr(
        scope,
        // Or 分支 lhs：left 已句柄化，get() 只读借用。
        expr.left.get(),
        expected_type,
        false,
      );
      let lhs_ty = lhs.r#type;
      let lhs_predicates = lhs.predicates;

      let inner_scope = self.child_scope(scope, &expr.base.base.location);
      self.resolve_predicate_vec_scope_ptr_bool(&lhs_predicates, &inner_scope, false);

      let rhs = self.check_expr(
        &inner_scope,
        // Or 分支 rhs：right 已句柄化，get() 只读借用（inner_scope 仅作用域派生）。
        expr.right.get(),
        expected_type,
        false,
      );
      let rhs_ty = rhs.r#type;
      let rhs_predicates = rhs.predicates;

      // Because of C++, I'm not sure if lhsPredicates was not moved out by the time we call checkBinaryOperation.
      let result = self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &lhs_predicates);
      WithPredicate::with_predicate_t_predicate_vec(
        result,
        PredicateVec::from(alloc::vec![Predicate::Or(OrPredicate {
          lhs: lhs_predicates,
          rhs: rhs_predicates,
        })]),
      )
    } else if matches!(
      expr.op,
      AstExprBinaryOp::CompareEq | AstExprBinaryOp::CompareNe
    ) {
      // For these, passing expected_type is worse than simply forcing them, because their implementation
      // may inadvertently check if expectedTypes exist first and use it, instead of forceSingleton first.
      let lhs = self.check_expr(
        scope,
        // 比较分支：left 已句柄化，get() 只读借用。
        expr.left.get(),
        None,
        true,
      );
      let rhs = self.check_expr(
        scope,
        // rhs 同上：同一 AstExprBinary 的句柄子节点，get() 只读借用。
        expr.right.get(),
        None,
        true,
      );
      let lhs_ty = lhs.r#type;
      let rhs_ty = rhs.r#type;

      if let Some(predicate) = try_get_type_guard_predicate(expr) {
        return WithPredicate::with_predicate_t_predicate_vec(
          self.boolean_type,
          PredicateVec::from(alloc::vec![predicate]),
        );
      }

      let mut predicates: PredicateVec = PredicateVec::new();

      // 左操作数已句柄化：get() 短时只读借用供 try_get_l_value。
      if let Some(lvalue) = try_get_l_value(expr.left.get()) {
        predicates.push(Predicate::Eq(EqPredicate {
          lvalue,
          ty: rhs_ty,
          location: expr.base.base.location,
        }));
      }

      // 右操作数同理（C++ `tryGetLValue(expr->right)` 直译）：get() 只读借用。
      if let Some(lvalue) = try_get_l_value(expr.right.get()) {
        predicates.push(Predicate::Eq(EqPredicate {
          lvalue,
          ty: lhs_ty,
          location: expr.base.base.location,
        }));
      }

      if !predicates.is_empty() && expr.op == AstExprBinaryOp::CompareNe {
        predicates = PredicateVec::from(alloc::vec![Predicate::Not(NotPredicate { predicates })]);
      }

      let result_ty =
        self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &PredicateVec::new());
      WithPredicate::with_predicate_t_predicate_vec(result_ty, predicates)
    } else {
      // Expected type_arguments are not useful for other binary operators.
      let lhs = self.check_expr(
        scope,
        // 其余运算符分支：left 已句柄化，get() 只读借用。
        expr.left.get(),
        None,
        false,
      );
      let rhs = self.check_expr(
        scope,
        // right 同 lhs：只读遍历 AST，检查期间解析树整体存活。
        expr.right.get(),
        None,
        false,
      );
      let lhs_ty = lhs.r#type;
      let rhs_ty = rhs.r#type;
      let lhs_predicates = lhs.predicates;

      // Intentionally discarding predicates with other operators.
      let result_ty = self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &lhs_predicates);
      WithPredicate::with_predicate_t(result_ty)
    }
  }

  pub fn check_expr_type_assertion(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprTypeAssertion,
  ) -> WithPredicate<TypeId> {
    // annotation/expr 已句柄化恒非空（C++ `resolveType(scope, *expr->annotation)`
    // 直译）：.get() 安全借用，检查期内只读。
    let annotation_type = self.resolve_type(scope.clone(), expr.annotation.get());
    let result = self.check_expr(
      scope,
      // 被断言子表达式同为句柄化非空槽位。
      expr.expr.get(),
      Some(annotation_type),
      false,
    );

    if self
      .can_unify_type_id_type_id_scope_ptr_location(
        annotation_type,
        result.r#type,
        scope,
        &expr.base.base.location,
      )
      .is_empty()
    {
      return WithPredicate::with_predicate_t_predicate_vec(annotation_type, result.predicates);
    }

    if self
      .can_unify_type_id_type_id_scope_ptr_location(
        result.r#type,
        annotation_type,
        scope,
        &expr.base.base.location,
      )
      .is_empty()
    {
      return WithPredicate::with_predicate_t_predicate_vec(annotation_type, result.predicates);
    }

    self.report_error_location_type_error_data(
      &expr.base.base.location,
      TypesAreUnrelated {
        left: result.r#type,
        right: annotation_type,
      }
      .into(),
    );
    WithPredicate::with_predicate_t_predicate_vec(
      self.error_recovery_type_type_id(annotation_type),
      result.predicates,
    )
  }

  pub fn check_expr_if_else(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIfElse,
    expected_type: Option<TypeId>,
  ) -> WithPredicate<TypeId> {
    let result = self.check_expr(
      scope,
      // condition 已句柄化恒非空；.get() 只读借用出自 expr 存活引用。
      expr.condition.get(),
      None,
      false,
    );

    // true/false_expr 同口径句柄化，location 只读拷贝经 .get()。
    let true_scope = self.child_scope(scope, &expr.true_expr.get().base.location);
    self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &true_scope, true);
    let true_type = self.check_expr(&true_scope, expr.true_expr.get(), expected_type, false);

    let false_scope = self.child_scope(scope, &expr.false_expr.get().base.location);
    self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &false_scope, false);
    let false_type = self.check_expr(&false_scope, expr.false_expr.get(), expected_type, false);

    if false_type.r#type == true_type.r#type {
      return WithPredicate::with_predicate_t(true_type.r#type);
    }

    let types = reduce_union(&[true_type.r#type, false_type.r#type]);
    if types.is_empty() {
      return WithPredicate::with_predicate_t(self.never_type);
    }

    WithPredicate::with_predicate_t(if types.len() == 1 {
      types[0]
    } else {
      self.add_type(&UnionType { options: types })
    })
  }

  pub fn check_expr_interp_string(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprInterpString,
  ) -> WithPredicate<TypeId> {
    // expressions 是 parser 填入 arena 的 *mut AstExpr 数组（元素非空、随解析
    // 结果存活）；槽位解引用收口 iter_nodes，只读遍历、cpp 同序。
    for child in expr.expressions.iter_nodes() {
      self.check_expr(scope, child, None, false);
    }

    WithPredicate::with_predicate_t(self.string_type)
  }

  pub fn check_expr_instantiate(
    &mut self,
    scope: &ScopePtr,
    explicit_type_instantiation: &AstExprInstantiate,
  ) -> WithPredicate<TypeId> {
    if !fflag::LuauExplicitTypeInstantiationSupport.get() {
      return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
    }

    // SAFETY: parser 为 AstExprInstantiate 在 arena 分配被实例化表达式（非空），只读使用。
    let base_expr = unsafe { &*explicit_type_instantiation.expr };
    let base_type = self.check_expr(scope, base_expr, None, false);

    // SAFETY: 满足被调 unsafe fn 的入参契约——base_type.r#type 是刚由 checkExpr 产出的
    // arena 类型句柄；function_expr/location 派生自上面存活的 arena 节点 base_expr；
    // type_arguments 为 parser 拥有的 arena 数组，本次调用内只读。
    WithPredicate::with_predicate_t(unsafe {
      self.instantiate_type_parameters(
        scope.clone(),
        base_type.r#type,
        explicit_type_instantiation.type_arguments,
        base_expr as *const _,
        &base_expr.base.location,
      )
    })
  }
}
