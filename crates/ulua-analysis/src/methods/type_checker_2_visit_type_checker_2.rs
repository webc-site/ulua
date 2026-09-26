use alloc::{format, string::String, vec};
use core::{
  cmp::max,
  mem::{drop, take},
  ptr::{NonNull, from_ref, null_mut},
};

use ulua_ast::{
  enums::{
    ast_expr_ref::AstExprRef, ast_stat_ref::AstStatRef, ast_type_pack_ref::AstTypePackRef,
    ast_type_ref::AstTypeRef,
  },
  functions::to_string_ast::to_str_binary as to_str,
  records::{
    ast_class_method::AstClassMethod,
    ast_class_property::AstClassProperty,
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
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
    ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_expr_varargs::AstExprVarargs,
    ast_local::AstLocal,
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal,
    ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction,
    ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction,
    ast_stat_while::AstStatWhile,
    ast_type::AstType,
    ast_type_function::AstTypeFunction,
    ast_type_intersection::AstTypeIntersection,
    ast_type_list::AstTypeList,
    ast_type_or_pack::AstTypeOrPack,
    ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference,
    ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof,
    ast_type_union::AstTypeUnion,
    location::Location,
  },
  rtti::{AstNodePtr, ast_node_is_ptr, ast_node_try_as},
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::{
    annotation_check_mode::AnnotationCheckMode, normalization_result::NormalizationResult,
    op_kind::OpKind, type_context::TypeContext, value::Value, value_context::ValueContext,
  },
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    begin_type_pack::begin,
    end_type_pack::end,
    extend_type_pack::extend_type_pack,
    find_metatable_entry::find_metatable_entry,
    finite::finite,
    first::first,
    flatten_type_pack::flatten_type_pack_id,
    follow_type, follow_type_pack,
    get_identifier_of_base_var_type_infer::get_identifier_of_base_var,
    get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_parameter_extents::get_parameter_extents,
    get_table_type::get_table_type,
    get_type, get_type_pack,
    has_length::has_length,
    instantiate::instantiate,
    is_comparison_op::is_comparison_op,
    is_ok_to_compare::is_ok_to_compare,
    is_optional::is_optional,
    is_string::is_string,
    magic_names::{LUAU_BLOCKED_TYPE, LUAU_FORCE_CONSTRAINT_SOLVING_INCOMPLETE, LUAU_PRINT},
    match_assert::match_assert,
    match_type_of::match_type_of,
    op_to_meta_table_entry::op_to_meta_table_entry,
    should_suppress_errors_type_utils::should_suppress_errors,
    size_type_pack::size,
    strip_nil::strip_nil,
    to_string_to_string::to_string_type_id,
  },
  records::{
    any_type::AnyType,
    arena_handle::Handle,
    blocked_type::BlockedType,
    cannot_call_non_function::CannotCallNonFunction,
    cannot_compare_unrelated_types::CannotCompareUnrelatedTypes,
    cannot_extend_table::{self, CannotExtendTable},
    cannot_infer_binary_operation::CannotInferBinaryOperation,
    code_too_complex::CodeTooComplex,
    count_mismatch::{CountMismatch, CountMismatchContext},
    dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe,
    error_suppression::ErrorSuppression,
    extern_type::ExternType,
    extra_information::ExtraInformation,
    free_type::FreeType,
    function_exits_without_returning::FunctionExitsWithoutReturning,
    function_type::FunctionType,
    generic_error::GenericError,
    generic_type::GenericType,
    in_conditional_context::InConditionalContext,
    incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    instantiation::Instantiation,
    internal_error::InternalError,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    never_type::NeverType,
    normalization_too_complex::NormalizationTooComplex,
    not_a_table::NotATable,
    optional_value_access::OptionalValueAccess,
    property_access_violation::{self, PropertyAccessViolation},
    recursive_restraint_violation::RecursiveRestraintViolation,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    swapped_generic_type_parameter::SwappedGenericTypeParameter,
    symbol::Symbol,
    table_type::TableType,
    txn_log::TxnLog,
    type_checker_2::TypeChecker2,
    type_fun::TypeFun,
    type_function_instance_type::TypeFunctionInstanceType,
    type_level::TypeLevel,
    type_mismatch::TypeMismatch,
    type_pack::TypePack,
    types_are_unrelated::TypesAreUnrelated,
    unification_too_complex::UnificationTooComplex,
    union_type::UnionType,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{
    error_type::ErrorType,
    name_type::Name,
    singleton_variant::SingletonVariant,
    type_error_data::{IntoTypeErrorData, TypeErrorData},
    type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

/// C++ `TxnLog* log = nullptr` 默认实参哨兵的具名形态：表示不携带事务日志。
const NO_TXN_LOG: *mut TxnLog = null_mut();

impl TypeChecker2 {
  /// 语句 RTTI 分发（cpp `visit(AstStat*)`）：通过安全的 [`AstStatRef`] 模式匹配具体语句类型。
  pub fn visit_stat(&mut self, stat: &AstStat) {
    let node = &stat.base;
    // SAFETY: push_stack 只按指针身份查 ast_scopes 并压/弹栈帧；从存活节点
    // 引用派生的裸指针满足其身份键契约（cpp pushStack(fn) 同构）。
    let _pusher = self.push_stack((node as *const AstNode).cast_mut());
    match stat.as_stat_ref() {
      AstStatRef::Block(stat) => self.visit_stat_block(stat),
      AstStatRef::If(stat) => self.visit_stat_if(stat),
      AstStatRef::While(stat) => self.visit_stat_while(stat),
      AstStatRef::Repeat(stat) => self.visit_stat_repeat(stat),
      AstStatRef::Break(stat) => self.visit_stat_break(stat),
      AstStatRef::Continue(stat) => self.visit_stat_continue(stat),
      AstStatRef::Return(stat) => self.visit_stat_return(stat),
      AstStatRef::Expr(stat) => self.visit_stat_expr(stat),
      AstStatRef::Local(stat) => self.visit_stat_local(stat),
      AstStatRef::For(stat) => self.visit_stat_for(stat),
      AstStatRef::ForIn(stat) => self.visit_stat_for_in(stat),
      AstStatRef::Assign(stat) => self.visit_stat_assign(stat),
      AstStatRef::CompoundAssign(stat) => self.visit_stat_compound_assign(stat),
      AstStatRef::Function(stat) => self.visit_stat_function(stat),
      AstStatRef::LocalFunction(stat) => self.visit_stat_local_function(stat),
      AstStatRef::TypeAlias(stat) => self.visit_stat_type_alias(stat),
      AstStatRef::TypeFunction(stat) => self.visit_stat_type_function(stat),
      AstStatRef::DeclareFunction(stat) => self.visit_stat_declare_function(stat),
      AstStatRef::DeclareGlobal(stat) => self.visit_stat_declare_global(stat),
      AstStatRef::DeclareExternType(stat) => self.visit_stat_declare_extern_type(stat),
      AstStatRef::DeclareClass(stat) => self.visit_stat_class(stat),
      AstStatRef::Error(stat) => self.visit_stat_error(stat),
    }
  }

  pub fn visit_expr_group(&mut self, expr: &AstExprGroup, context: ValueContext) {
    // expr.expr 已句柄化：get() 只读借用出自存活 &AstExprGroup（cpp 透传契约）。
    let inner = expr.expr.get();
    self.visit_expr(inner, context);
  }

  /// nil/number/integer 常量字面量共用的调试断言（cpp `visit(AstExprConstant{Nil,Number,Integer}*)`
  /// 的 `LUAU_ENABLE_ASSERT` 块）：推导类型与期望内建类型必须满足子类型关系。
  /// `expected_first` 复刻 cpp 实参顺序差异：nil 传 `(actual, expected)`，
  /// number/integer 传 `(expected, inferred)`。
  #[cfg(debug_assertions)]
  fn assert_constant_subtype(
    &mut self,
    base: &AstExpr,
    expected_type: TypeId,
    expected_first: bool,
  ) {
    let inferred_type = self.lookup_type(base);
    let scope = self.find_innermost_scope(base.base.location);
    let (lhs, rhs) = if expected_first {
      (expected_type, inferred_type)
    } else {
      (inferred_type, expected_type)
    };

    let subtyping = self.subtyping_mut();
    let r = subtyping.is_subtype_type_id_type_id_not_null_scope(lhs, rhs, scope);

    LUAU_ASSERT!(
      r.is_subtype || self.is_error_suppressing_location_type_id(base.base.location, inferred_type)
    );
  }

  /// bool/string 常量字面量共用的「bestType 对 inferredType 做子类型校验并按位置
  /// 回报错误」尾段（cpp `TypeChecker2.cpp:1753,1796`）。两者唯一差异是错误抑制
  /// 判据：`gate_by_result` 为真时取子类型结果的 `is_error_suppressing`（bool），
  /// 否则按位置查 `is_error_suppressing_location_type_id`（string）。
  fn check_constant_best_type(&mut self, base: &AstExpr, best_type: TypeId, gate_by_result: bool) {
    // builtin_types/subtyping 已句柄化；节点字段全部走安全引用只读，本块无裸指针。
    let inferred_type = self.lookup_type(base);
    let location = base.base.location;
    let scope = self.find_innermost_scope(location);

    let mut r = self
      .subtyping_mut()
      .is_subtype_type_id_type_id_not_null_scope(best_type, inferred_type, scope);

    let suppressed = if gate_by_result {
      r.is_error_suppressing
    } else {
      self.is_error_suppressing_location_type_id(location, inferred_type)
    };
    if suppressed {
      return;
    }

    if !r.is_subtype {
      self.report_error_type_error_data_location(
        TypeMismatch::from_wanted_given(inferred_type, best_type).into(),
        &location,
      );
    }
    for e in &mut r.errors {
      e.location = location;
    }
    self.report_errors(take(&mut r.errors));
  }

  pub fn visit_expr_constant_nil(&mut self, expr: &AstExprConstantNil) {
    #[cfg(debug_assertions)]
    {
      // builtin_types 为 Handle 只读句柄，断言块内无裸指针操作。
      let expected_type = self.builtin_types.get().nil_type;
      self.assert_constant_subtype(&expr.base, expected_type, false);
    }
    #[cfg(not(debug_assertions))]
    let _ = expr;
  }

  pub(crate) fn visit_expr_constant_bool(&mut self, expr: &AstExprConstantBool) {
    // booleans use specialized inference logic for singleton type_arguments,
    // which can lead to real type errors here.
    // builtin_types 为 Handle 只读句柄；true/false_type 是 Copy 句柄，取出后借即止。
    let builtin_types = self.builtin_types.get();
    let best_type = if expr.value {
      builtin_types.true_type
    } else {
      builtin_types.false_type
    };
    self.check_constant_best_type(&expr.base, best_type, true);
  }

  pub(crate) fn visit_expr_constant_number(&mut self, expr: &AstExprConstantNumber) {
    #[cfg(debug_assertions)]
    {
      let expected_type = self.builtin_types.get().number_type;
      self.assert_constant_subtype(&expr.base, expected_type, true);
    }
    #[cfg(not(debug_assertions))]
    let _ = expr;
  }

  pub(crate) fn visit_expr_constant_integer(&mut self, expr: &AstExprConstantInteger) {
    #[cfg(debug_assertions)]
    {
      let expected_type = self.builtin_types.get().integer_type;
      self.assert_constant_subtype(&expr.base, expected_type, true);
    }
    #[cfg(not(debug_assertions))]
    let _ = expr;
  }

  pub(crate) fn visit_expr_constant_string(&mut self, expr: &AstExprConstantString) {
    // strings use specialized inference logic for singleton type_arguments,
    // which can lead to real type errors here.
    // SAFETY: (*expr).value 的字节缓冲由 AST arena 持有，随安全引用同寿，仅拷贝。
    let string_data = String::from_utf8_lossy(expr.value.as_bytes()).into_owned();

    // C++: module->internalTypes.addType(SingletonType{StringSingleton{...}})
    // module_mut() 访问器收口 arena 写入，NotNull 契约与 SAFETY 见访问器定义。
    let best_type = self
      .module_mut()
      .internal_types
      .add_type(SingletonType::new(SingletonVariant::V1(
        StringSingleton::new(string_data),
      )));
    self.check_constant_best_type(&expr.base, best_type, false);
  }

  pub fn visit_expr_local(&mut self, _expr: &AstExprLocal) {
    // TODO!
  }

  pub(crate) fn visit_expr_global(&mut self, expr: &AstExprGlobal) {
    // 栈元素已句柄化（#24 字段面）：`self.stack.last()` 拷出的 `Handle<Scope>`
    // 由 push_stack 自 module.ast_scopes 放入，Scope 实体存活于 module.scopes，
    // 覆盖整个 check 会话；expect 先保证栈非空后才拷出句柄，`get()` 只读借用
    // 不再需要 unsafe（解引用契约集中在 `arena_handle`）。
    {
      let scope = *self.stack.last().expect(
        "check 入口已压入模块根 scope，visit 全程栈恒非空（cpp TypeChecker2 stack 不变式）",
      );
      let name = expr.name;

      if scope
        .get()
        .lookup_symbol(Symbol::from_global(name))
        .is_none()
      {
        let name_string = name.as_str_or_empty().to_string();
        self.report_error_type_error_data_location(
          UnknownSymbol::new(name_string, Context::Binding).into(),
          &expr.base.base.location,
        );
      } else if scope.get().should_warn_global(name.as_str_or_empty())
        && !self.warned_globals.contains_str(name.as_str_or_empty())
      {
        // 仅在真正上报时才物化键（should_warn_global/warned_globals 查询零分配，见 r7-rc-4 借用口）。
        let name_string = name.as_str_or_empty().to_string();
        self.report_error_type_error_data_location(
          UnknownSymbol::new(name_string.clone(), Context::Binding).into(),
          &expr.base.base.location,
        );
        self.warned_globals.insert(name_string);
      }
    }
  }

  pub fn visit_expr_varargs(&mut self, _expr: &AstExprVarargs) {
    // TODO: Implement visit_expr_varargs logic
  }

  pub(crate) fn visit_expr_call(&mut self, call: &AstExprCall) {
    let mut _flipper: Option<InConditionalContext> = None;

    // SAFETY: call.func 是 parser 随 &AstExprCall 建于 arena 的子指针；
    // InConditionalContext::new 暂存 type_context 字段地址并在块/函数尾
    // 析构回写，借用不越出本函数。args 数组遍历走 iter_nodes 门面。
    unsafe {
      // We want to preserve the existing conditional context if we are in a `typeof` call.
      if !match_type_of(call) {
        _flipper = Some(InConditionalContext::new(
          &mut self.type_context,
          TypeContext::Default,
        ));
      }

      self.visit_expr(&*call.func, ValueContext::RValue);

      if match_assert(call) && call.args.size > 0 {
        {
          // C++: `InConditionalContext flipper(&typeContext);` (TypeChecker2.cpp:1843)
          // uses the default `newValue = TypeContext::Condition` so that the first
          // argument of `assert(...)` is checked in a conditional context (refinements
          // like `assert(typeof(x) == "table")` apply to property accesses inside it).
          let _flipper = InConditionalContext::new(&mut self.type_context, TypeContext::Condition);
          // 首实参：iter_nodes 收口 args[0] 的解引用（size>0 守卫在前，取不到仅为不可达）
          self.visit_expr(
            call.args.iter_nodes().next().expect("args.size > 0"),
            ValueContext::RValue,
          );
        }

        // 其余实参：cpp `for (i = 1; i < args.size; ++i)` 同序，skip(1) 保持迭代顺序
        for arg in call.args.iter_nodes().skip(1) {
          self.visit_expr(arg, ValueContext::RValue);
        }
      } else {
        for arg in call.args.iter_nodes() {
          self.visit_expr(arg, ValueContext::RValue);
        }
      }

      self.visit_call(call);
    }
  }

  pub fn visit_expr_index_name(&mut self, index_name: &AstExprIndexName, context: ValueContext) {
    let prop_name = index_name.index.as_str_or_empty();
    let ast_index_expr_ty = self.builtin_types_ref().string_type;
    // expr 已句柄化恒非空：.get() 安全借用派生 &AstExpr。
    self.visit_expr_name(
      index_name.expr.get(),
      index_name.base.base.location,
      prop_name,
      context,
      ast_index_expr_ty,
    );
  }

  pub fn visit_expr_index_expr(&mut self, index_expr: &AstExprIndexExpr, context: ValueContext) {
    // expr/index 已句柄化恒非空：.get() 安全借用，后续判别与查询全部走安全引用。
    let (target, index) = (index_expr.expr.get(), index_expr.index.get());
    if let Some(index_as_constant_string) = ast_node_try_as::<AstExprConstantString>(&index.base) {
      let ast_index_expr_type = self.lookup_type(index);
      let string_value = index_as_constant_string
        .value
        .as_bytes()
        .iter()
        .map(|&b| b as char)
        .collect::<String>();

      // expr 已句柄化恒非空：.get() 安全借用派生 &AstExpr。
      self.visit_expr_name(
        index_expr.expr.get(),
        index_expr.base.base.location,
        &string_value,
        context,
        ast_index_expr_type,
      );
      return;
    }

    self.visit_expr(target, ValueContext::RValue);
    self.visit_expr(index, ValueContext::RValue);

    let expr_type = follow_type::follow(self.lookup_type(target));
    let index_type = follow_type::follow(self.lookup_type(index));

    if let Some(tt) = get_type::get::<TableType>(expr_type) {
      if let Some(indexer) = &tt.indexer {
        self.test_is_subtype_type_id_type_id_location(
          index_type,
          indexer.index_type,
          index.base.location,
        );
        if fflag::LuauReadOnlyIndexers.get()
          && context == ValueContext::LValue
          && indexer.is_read_only
        {
          let err = PropertyAccessViolation {
            table: expr_type,
            key: "indexer".to_string(),
            context: property_access_violation::Context::CannotWrite,
          };
          self.report_error_type_error_data_location(
            TypeErrorData::PropertyAccessViolation(err),
            &index_expr.base.base.location,
          );
        }
      } else {
        let err = CannotExtendTable {
          table_type: expr_type,
          context: cannot_extend_table::Context::Indexer,
          prop: "indexer??".to_string(),
        };
        self.report_error_type_error_data_location(
          TypeErrorData::CannotExtendTable(err),
          &index_expr.base.base.location,
        );
      }
    } else if let Some(mt) = get_type::get::<MetatableType>(expr_type) {
      self.type_checker_2_index_expr_metatable_helper(index_expr, mt, expr_type, index_type);
    } else if let Some(cls) = get_type::get::<ExternType>(expr_type) {
      if let Some(indexer) = &cls.indexer {
        self.test_is_subtype_type_id_type_id_location(
          index_type,
          indexer.index_type,
          index.base.location,
        );
      } else {
        let err = DynamicPropertyLookupOnExternTypesUnsafe { ty: expr_type };
        self.report_error_type_error_data_location(
          TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(err),
          &index_expr.base.base.location,
        );
      }
    } else if get_type::get::<UnionType>(expr_type).is_some() && is_optional(expr_type) {
      // Safety: should_suppress_errors 是 unsafe fn，要求 normalizer 指针有效且
      // 独占。传入的是 `&mut self.normalizer` 强转的 *mut——借自 &mut self 的
      // 唯一可变借用，在调用期内无其他 alias（expr_type 是 Copy 的 TypeId）。
      let suppression = unsafe { should_suppress_errors(&mut self.normalizer, expr_type) };
      match suppression.value {
        Value::DoNotSuppress | Value::NormalizationFailed => {
          if matches!(suppression.value, Value::NormalizationFailed) {
            self.report_error_type_error_data_location(
              TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
              &index_expr.base.base.location,
            );
          }
          self.report_error_type_error_data_location(
            TypeErrorData::OptionalValueAccess(OptionalValueAccess {
              optional: expr_type,
            }),
            &index_expr.base.base.location,
          );
        }
        Value::Suppress => {}
      }
    } else if let Some(ut) = get_type::get::<UnionType>(expr_type) {
      // if all of the type_arguments are a table type, the union must be
      // a table, and so we shouldn't error.
      // C++ `for (TypeId ty : ut)` 短路遍历 — UnionTypeIterator 防环并展平嵌套 union。
      let all_tables = begin_union_type(ut).all(|ty| get_table_type(ty).is_some());

      if !all_tables {
        // Safety: 同上口径——union 非全表分支里再次以 self 独占的 normalizer
        // 可变借用调用 unsafe fn should_suppress_errors，指针临时且无 alias。
        let suppression = unsafe { should_suppress_errors(&mut self.normalizer, expr_type) };
        match suppression.value {
          Value::DoNotSuppress | Value::NormalizationFailed => {
            if matches!(suppression.value, Value::NormalizationFailed) {
              self.report_error_type_error_data_location(
                TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                &index_expr.base.base.location,
              );
            }
            self.report_error_type_error_data_location(
              TypeErrorData::NotATable(NotATable { ty: expr_type }),
              &index_expr.base.base.location,
            );
          }
          Value::Suppress => {}
        }
      }
    } else if let Some(it) = get_type::get::<IntersectionType>(expr_type) {
      // C++ `for (TypeId part : it)` 短路遍历 — IntersectionTypeIterator 防环并展平。
      let any_table = begin_intersection_type(it).any(|part| get_table_type(part).is_some());

      if !any_table {
        self.report_error_type_error_data_location(
          TypeErrorData::NotATable(NotATable { ty: expr_type }),
          &index_expr.base.base.location,
        );
      }
    } else if get_type::get::<NeverType>(expr_type).is_none()
      && !self.is_error_suppressing_location_type_id(index_expr.base.base.location, expr_type)
    {
      self.report_error_type_error_data_location(
        TypeErrorData::NotATable(NotATable { ty: expr_type }),
        &index_expr.base.base.location,
      );
    }
  }

  pub fn visit_expr_function(&mut self, function: &AstExprFunction) {
    let fn_ref = function;
    let location = fn_ref.base.base.location;

    // InConditionalContext flipper(&typeContext, TypeContext::Default);
    // Safety: InConditionalContext::new 以裸指针暂存 self.type_context 并在 drop
    // 时回写旧值；_flipper 是本函数局部 RAII，在 `&mut self` 借用结束前析构，
    // 期间 checker 独占 self，指针对象不会失效或被并发写。
    let _flipper = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

    // auto StackPusher = pushStack(fn);
    // AstNode 子对象在偏移 0，指针值与 push_stack 期望一致。
    let _stack_pusher = self.push_stack(from_ref(function).cast_mut().as_ast_node());

    self.visit_generics_nodes(&fn_ref.generics, &fn_ref.generic_packs);

    let inferred_fn_ty = self.lookup_type(&function.base);
    self.function_decl_stack.push(inferred_fn_ty);

    // std::shared_ptr<const NormalizedType> normalizedFnTy = normalizer.normalize(inferredFnTy);
    // 归一化失败（过于复杂）对应 C++ 的空指针分支
    let mut normalized_fn_ty = self.normalizer.try_normalize(inferred_fn_ty);

    if normalized_fn_ty.is_none() {
      self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
    } else if get_type::get::<ErrorType>(
      normalized_fn_ty
        .as_ref()
        .expect("if 支已判 is_none，else-if 链内必为 Some")
        .errors,
    )
    .is_some()
    {
      // If we have an error type, we don't want to do anything else involving the normalized type
      normalized_fn_ty = None;
    } else if !normalized_fn_ty
      .as_ref()
      .expect("if 支已判 is_none，else-if 链内必为 Some")
      .has_functions()
    {
      self.report_error_type_error_data_location(
        InternalError::new(format!(
          "Internal error: Lambda has non-function type {}",
          to_string_type_id(inferred_fn_ty)
        ))
        .into(),
        &location,
      );
      self.function_decl_stack.pop();
      return;
    } else {
      let normalized = normalized_fn_ty
        .as_ref()
        .expect("if 支已判 is_none，else 链尾必为 Some");
      if normalized.functions.parts.size() != 1 {
        self.report_error_type_error_data_location(
          InternalError::new(format!(
            "Unexpected: Lambda has unexpected type {}",
            to_string_type_id(inferred_fn_ty)
          ))
          .into(),
          &location,
        );
        self.function_decl_stack.pop();
        return;
      }

      let inferred_ftv_ty = normalized.functions.parts.front();
      // C++ LUAU_ASSERT(inferredFtv)（TypeChecker2.cpp:2065-2066）。
      let inferred_ftv = get_type::get::<FunctionType>(inferred_ftv_ty)
        .expect("cpp LUAU_ASSERT(inferredFtv)：单成分归一化函数的 front 必为 FunctionType");

      // There is no way to write an annotation for the self argument, so we
      // cannot do anything to check it.
      let mut arg_it = begin(inferred_ftv.arg_types);
      let arg_end = end(inferred_ftv.arg_types);
      if !fn_ref.self_.is_null() {
        arg_it.advance();
      }

      // args 元素解引用收口在 iter_nodes（只读遍历，cpp 同序，break 位置不变）。
      for arg in fn_ref.args.iter_nodes() {
        if arg_it == arg_end {
          break;
        }

        let inferred_arg_ty = *arg_it.current();
        let arg_ref = arg;

        // SAFETY: annotation 判空后即 arena 存活的 AstType；同一借用供访问与查询。
        let annotation = unsafe { arg_ref.annotation.as_ref() };
        if let Some(annotation) = annotation {
          // we need to typecheck any argument annotations themselves.
          self.visit_type(annotation);
          let annotated_arg_ty = self.lookup_annotation(annotation);

          self.test_is_subtype_type_id_type_id_location(
            inferred_arg_ty,
            annotated_arg_ty,
            arg_ref.location,
          );
        }

        // Some Luau constructs can result in an argument type being
        // reduced to never by inference. In this case, we want to
        // report an error at the function, instead of reporting an
        // error at every callsite.
        if get_type::get::<NeverType>(follow_type::follow(inferred_arg_ty)).is_some() {
          // If the annotation simplified to never, we don't want to
          // even look at contributors.
          let mut explicitly_never = false;
          if let Some(annotation) = annotation {
            let annotated_arg_ty = self.lookup_annotation(annotation);
            explicitly_never = get_type::get::<NeverType>(annotated_arg_ty).is_some();
          }

          // Not following here is deliberate.
          // module_ref() 为安全访问器，internalTypes 裸指针由构造契约保证有效。
          let contributors = self
            .module_ref()
            .upper_bound_contributors
            .find(&inferred_arg_ty)
            .cloned();
          if let Some(contributors) = contributors
            && !explicitly_never
          {
            let arg_name = arg_ref.name.as_str_or_empty().to_string();
            self.report_error_type_error_data_location(
              GenericError::new(format!(
                "Parameter '{}' has been reduced to never. This function is not callable with any possible value.",
                arg_name
              ))
              .into(),
              &arg_ref.location,
            );
            for (site, component) in contributors {
              self.report_error_type_error_data_location(
                ExtraInformation::new(format!(
                  "Parameter '{}' is required to be a subtype of '{}' here.",
                  arg_name,
                  to_string_type_id(component)
                ))
                .into(),
                &site,
              );
            }
          }
        }

        arg_it.advance();
      }

      // we need to typecheck the vararg annotation, if it exists.
      // SAFETY: vararg_annotation/return_annotation 判空后即 arena 存活的
      if fn_ref.vararg {
        self.visit_type_pack(fn_ref.vararg_annotation.as_ref());
      }

      // Safety: fn_ref.body 是 &AstExprFunction 形参的 arena 子指针（parser 对
      // 无函数体的语法错误节点也会补空 block），传给 get_fallthrough 仅作
      // 指针身份查表，且返回前不与其它借用冲突。
      let reaches_implicit_return = !self
        .type_checker_2_get_fallthrough(fn_ref.body.cast::<AstStat>().as_ptr())
        .is_null();
      if reaches_implicit_return
        && !self.allows_no_return_values(follow_type_pack::follow(inferred_ftv.ret_types))
      {
        // Safety: get_end_location 为 unsafe fn，要求 function 指向存活
        // AstExprFunction；此处传入的就是本方法的安全形参 `function` 本身。
        let end_location = unsafe { self.get_end_location(function) };
        self.report_error_type_error_data_location(
          FunctionExitsWithoutReturning {
            expected_return_type: inferred_ftv.ret_types,
          }
          .into(),
          &end_location,
        );
      }
    }

    self.visit_stat_block(&fn_ref.body);

    // we need to typecheck the return annotation itself, if it exists.
    self.visit_type_pack(fn_ref.return_annotation.as_ref());

    // If the function type has a function annotation, we need to see if we can suggest an annotation
    if let Some(normalized) = normalized_fn_ty.as_ref() {
      let part = normalized.functions.parts.front();
      self.type_checker_2_suggest_annotations(function, part);
    }

    self.function_decl_stack.pop();
  }

  pub(crate) fn visit_expr_table(&mut self, expr: &AstExprTable) {
    // SAFETY: InConditionalContext::new 拿到的是 &mut self 借用下 type_context
    // 字段的地址，_in_context 在块结束前析构回写，借用不越过本函数；表项
    // key/value 是 &AstExprTable 自有的 arena 子指针（key 判空，value 必建）。
    unsafe {
      let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

      for item in expr.items.iter() {
        if !item.key.is_null() {
          self.visit_expr(&*item.key, ValueContext::RValue);
        }
        self.visit_expr(&*item.value, ValueContext::RValue);
      }
    }
  }

  pub fn visit_expr_unary(&mut self, expr: &AstExprUnary) {
    // Safety: 闭包内 InConditionalContext::new 暂存 self.type_context 字段地址；
    // in_context 的 Option 存活到本函数末尾（见尾部 `let _ = in_context;`），
    // drop 回写发生前 &mut self 借用始终有效，无第二个持有者。
    let in_context = (expr.op != AstExprUnaryOp::Not)
      .then(|| InConditionalContext::new(&mut self.type_context, TypeContext::Default));
    let _ = &in_context;

    // expr 已句柄化：get() 即安全只读借用（parser 必建操作数），
    // 后续访问/查询全部走该借用。
    let operand = expr.expr.get();
    self.visit_expr(operand, ValueContext::RValue);

    let operand_type = self.lookup_type(operand);
    let result_type = self.lookup_type(&expr.base);

    if self.is_error_suppressing_location_type_id(operand.base.location, operand_type) {
      return;
    }

    const K_UNARY_OP_METAMETHODS: [(AstExprUnaryOp, &str); 2] = [
      (AstExprUnaryOp::Minus, "__unm"),
      (AstExprUnaryOp::Len, "__len"),
    ];

    for (op, metamethod) in &K_UNARY_OP_METAMETHODS {
      if *op == expr.op {
        // Safety: find_metatable_entry 是 safe fn，但需按 C++ 传 NotNull 指针；
        // `&mut (*self.module).errors` 解引用的是构造契约保证有效的 module 字段，
        // 可变借用仅限本次调用。self.builtin_types.as_ptr() 以裸指针按值传递（Copy），
        // 与 module 借用不相交，故不用安全访问器以免 &mut self 全量冲突。
        let mm = unsafe {
          find_metatable_entry(
            Handle::from_ptr(self.builtin_types.as_ptr()),
            &mut (*self.module).errors,
            operand_type,
            metamethod,
            expr.base.base.location,
          )
        };

        if let Some(mm_ty) = mm {
          if let Some(ftv) = get_type::get::<FunctionType>(follow_type::follow(mm_ty)) {
            if let Some(ret) = first(ftv.ret_types, false) {
              if expr.op == AstExprUnaryOp::Len {
                self.test_is_subtype_type_id_type_id_location(
                  follow_type::follow(ret),
                  self.builtin_types_ref().number_type,
                  expr.base.base.location,
                );
              }
            } else {
              self.report_error_type_error_data_location(
                TypeErrorData::GenericError(GenericError::new(alloc::format!(
                  "Metamethod '{}' must return a value",
                  metamethod
                ))),
                &expr.base.base.location,
              );
            }

            if first(ftv.arg_types, false).is_none() {
              self.report_error_type_error_data_location(
                TypeErrorData::GenericError(GenericError::new(
                  "__unm metamethod must accept one argument".to_string(),
                )),
                &expr.base.base.location,
              );
              return;
            }

            // Safety: 两次 add_type_pack 写入 module 自带 internal_types arena；
            // `&mut (*self.module).internal_types` 借用止于本块，arena 与
            // builtinTypes/operand_type 读出的 TypeId 互不冲突（C++ 同序构造）。
            let (expected_args, expected_ret) = unsafe {
              let arena = &mut (*self.module).internal_types;
              (
                arena.add_type_pack_initializer_list_type_id(&[operand_type]),
                arena.add_type_pack_initializer_list_type_id(&[result_type]),
              )
            };
            // Safety: 紧随其后的第二次 arena 借用，仅用上面块内产出的 TypeId 值，
            // self.module 指针有效性同由构造契约保证。
            let expected_function = unsafe {
              (*self.module)
                .internal_types
                .add_type(FunctionType::function_type_new(
                  expected_args,
                  expected_ret,
                  None,
                  false,
                ))
            };

            if !self.test_is_subtype_type_id_type_id_location(
              mm_ty,
              expected_function,
              expr.base.base.location,
            ) {
              return;
            }
          }
          return;
        }
        break;
      }
    }

    match expr.op {
      AstExprUnaryOp::Len => {
        let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
        let mut recursion_count = 0;
        // C++ 中 nty 为空时 isInhabited(nullptr) 命中 HitLimits → 报错返回
        let Some(nty) = self.normalizer.try_normalize(operand_type) else {
          self.report_error_type_error_data_location(
            NormalizationTooComplex::default().into_type_error_data(),
            &expr.base.base.location,
          );
          return;
        };

        if nty.should_suppress_errors() {
          return;
        }

        match self.normalizer.is_inhabited_normalized_type(&nty) {
          NormalizationResult::True => {}
          NormalizationResult::False => return,
          NormalizationResult::HitLimits => {
            self.report_error_type_error_data_location(
              NormalizationTooComplex::default().into_type_error_data(),
              &expr.base.base.location,
            );
            return;
          }
        }

        if !has_length(operand_type, &mut seen, &mut recursion_count) {
          if is_optional(operand_type) {
            self.report_error_type_error_data_location(
              OptionalValueAccess {
                optional: operand_type,
              }
              .into_type_error_data(),
              &expr.base.base.location,
            );
          } else {
            self.report_error_type_error_data_location(
              NotATable { ty: operand_type }.into_type_error_data(),
              &expr.base.base.location,
            );
          }
        }
      }
      AstExprUnaryOp::Minus => {
        // A negated integer literal is folded into one constant by the compiler, so it never negates anything.
        if matches!(operand.as_expr_ref(), AstExprRef::ConstantInteger(_)) {
          self.test_is_subtype_type_id_type_id_location(
            operand_type,
            self.builtin_types_ref().integer_type,
            expr.base.base.location,
          );
        } else {
          self.test_is_subtype_type_id_type_id_location(
            operand_type,
            self.builtin_types_ref().number_type,
            expr.base.base.location,
          );
        }
      }
      AstExprUnaryOp::Not => {}
    }
  }

  pub fn visit_expr_binary(
    &mut self,
    expr: &AstExprBinary,
    override_key: Option<&AstStatCompoundAssign>,
  ) {
    let op = expr.op;
    // Safety: 短路求值/相等比较之外才创建 RAII 守卫；type_context 字段地址在
    // &mut self 有效期内稳定，in_context 于函数尾显式 drop 前一直保持独占。
    let in_context = (!matches!(
      op,
      AstExprBinaryOp::And
        | AstExprBinaryOp::Or
        | AstExprBinaryOp::CompareEq
        | AstExprBinaryOp::CompareNe
    ))
    .then(|| InConditionalContext::new(&mut self.type_context, TypeContext::Default));

    // left/right 已句柄化（fake 节点场景下由复合赋值的 var/value 传入，同源同寿），
    // get() 即安全只读借用换 &AstExpr。
    let (left, right) = (expr.left.get(), expr.right.get());

    if fflag::LuauLValueCompoundAssignmentVisitLhs.get() {
      // In compound assignments, the left side is both read-from and written-to, so we have to visit it in both contexts.
      // cpp 用 `overrideKey && overrideKey->is<AstStatCompoundAssign>()` 判别；
      // 全仓只有复合赋值调用点会传入该键，Option 类型即该判别的编码形态。
      if override_key.is_some() {
        self.visit_expr(left, ValueContext::LValue);
      }
    }

    self.visit_expr(left, ValueContext::RValue);
    self.visit_expr(right, ValueContext::RValue);

    // stack 顶部的 Scope 句柄由 push_stack 放入（Scope 归 module.scopes 持有），
    // expect 确保非空后经 `as_ptr` 还原裸句柄值传参（仅取指针、不解引用）。
    let scope = self
      .stack
      .last()
      .expect("check 入口已压入模块根 scope，visit 全程栈恒非空（cpp TypeChecker2 stack 不变式）")
      .as_ptr();

    let is_equality = matches!(
      expr.op,
      AstExprBinaryOp::CompareEq | AstExprBinaryOp::CompareNe
    );
    let is_comparison = is_comparison_op(expr.op);
    let is_logical = matches!(expr.op, AstExprBinaryOp::And | AstExprBinaryOp::Or);

    let mut left_type = follow_type::follow(self.lookup_type(left));
    let right_type = follow_type::follow(self.lookup_type(right));
    let expected_result = follow_type::follow(self.lookup_type(&expr.base));
    if get_type::get::<TypeFunctionInstanceType>(expected_result).is_some() {
      self.check_for_internal_type_function(expected_result, expr.base.base.location);
      return;
    }

    if expr.op == AstExprBinaryOp::Or {
      // Safety: strip_nil 为 unsafe fn：要求 arena/builtinTypes 在其返回类型
      // 使用期内有效。module arena 由构造契约持有且 checker 独占，
      // `&mut (*self.module).internal_types` 借用止于本调用。
      left_type = unsafe {
        strip_nil(
          Handle::from_ptr(self.builtin_types.as_ptr()),
          &mut (*self.module).internal_types,
          left_type,
        )
      };
    }

    let norm_left = self.normalizer.try_normalize(left_type);
    let norm_right = self.normalizer.try_normalize(right_type);

    let is_string_operation = norm_left
      .as_ref()
      .map_or_else(|| is_string(left_type), |norm| norm.is_subtype_of_string())
      && norm_right
        .as_ref()
        .map_or_else(|| is_string(right_type), |norm| norm.is_subtype_of_string());

    left_type = follow_type::follow(left_type);
    if get_type::get::<AnyType>(left_type).is_some()
      || get_type::get::<ErrorType>(left_type).is_some()
      || get_type::get::<NeverType>(left_type).is_some()
      || get_type::get::<AnyType>(right_type).is_some()
      || get_type::get::<ErrorType>(right_type).is_some()
      || get_type::get::<NeverType>(right_type).is_some()
      || norm_left
        .as_ref()
        .is_some_and(|norm| norm.should_suppress_errors())
      || norm_right
        .as_ref()
        .is_some_and(|norm| norm.should_suppress_errors())
    {
      return;
    }

    if (get_type::get::<BlockedType>(left_type).is_some()
      || get_type::get::<FreeType>(left_type).is_some()
      || get_type::get::<GenericType>(left_type).is_some())
      && !is_equality
      && !is_logical
    {
      // get_identifier_of_base_var（清单外）收裸指针；left 已句柄化，经 as_ptr 桥接。
      let name = get_identifier_of_base_var(expr.left.as_ptr());

      self.report_error_type_error_data_location(
        TypeErrorData::CannotInferBinaryOperation(CannotInferBinaryOperation::new(
          expr.op,
          name,
          if is_comparison {
            OpKind::Comparison
          } else {
            OpKind::Operation
          },
        )),
        &expr.base.base.location,
      );
      return;
    }

    let types_have_intersection = self
      .normalizer
      .is_intersection_inhabited_type_id_type_id(left_type, right_type);

    if types_have_intersection == NormalizationResult::HitLimits {
      self.report_error_type_error_data_location(
        NormalizationTooComplex::default().into_type_error_data(),
        &expr.base.base.location,
      );
      return;
    }

    if is_equality || is_comparison {
      if !is_ok_to_compare(
        &mut self.normalizer,
        types_have_intersection,
        norm_left.as_deref(),
        norm_right.as_deref(),
      ) {
        self.report_error_type_error_data_location(
          TypeErrorData::CannotCompareUnrelatedTypes(CannotCompareUnrelatedTypes {
            left: left_type,
            right: right_type,
            op: expr.op,
          }),
          &expr.base.base.location,
        );
        return;
      }

      let either_expr_is_nil = norm_left.as_ref().is_some_and(|norm| norm.is_nil())
        || norm_right.as_ref().is_some_and(|norm| norm.is_nil());

      if is_equality && either_expr_is_nil {
        return;
      }
    }

    if is_logical || (is_comparison && is_string_operation) {
      return;
    }

    let metamethod = op_to_meta_table_entry(expr.op);
    if !metamethod.is_empty() {
      // builtin_types 经安全访问器收口 NotNull 解引用；两次调用共享同一 &。
      let bts = self.builtin_types_ref();
      let (left_mt, right_mt) = (
        get_metatable_type_id_not_null_builtin_types(left_type, bts),
        get_metatable_type_id_not_null_builtin_types(right_type, bts),
      );
      let mut matches = left_mt == right_mt;

      if is_equality && !matches {
        if !matches
          && right_mt.is_some()
          && let Some(utv) = get_type::get::<UnionType>(left_type)
        {
          // C++ `testUnion` lambda 内 `for (TypeId option : utv)`——
          // UnionTypeIterator 展平嵌套 union 并 follow。
          for option in begin_union_type(utv) {
            if get_metatable_type_id_not_null_builtin_types(
              follow_type::follow(option),
              self.builtin_types_ref(),
            ) == right_mt
            {
              matches = true;
              break;
            }
          }
        }

        if !matches
          && left_mt.is_some()
          && let Some(utv) = get_type::get::<UnionType>(right_type)
        {
          for option in begin_union_type(utv) {
            if get_metatable_type_id_not_null_builtin_types(
              follow_type::follow(option),
              self.builtin_types_ref(),
            ) == left_mt
            {
              matches = true;
              break;
            }
          }
        }
      }

      if get_type::get::<TableType>(left_type).is_none()
        && get_type::get::<TableType>(right_type).is_none()
        && (left_mt.is_none() || right_mt.is_none())
      {
        matches = matches || types_have_intersection != NormalizationResult::False;
      }

      if !matches && is_comparison {
        self.report_error_type_error_data_location(
          TypeErrorData::GenericError(GenericError::new(alloc::format!(
            "Types {} and {} cannot be compared with {} because they do not have the same metatable",
            to_string_type_id(left_type),
            to_string_type_id(right_type),
            to_str(expr.op)
          ))),
          &expr.base.base.location,
        );
        return;
      }

      // Safety: 元方法回退链第一次查找——解引用 module.errors 的可变借用止于
      // 本次 find_metatable_entry（safe fn）调用；builtin_types 按裸值传递。
      let left_mm = unsafe {
        find_metatable_entry(
          Handle::from_ptr(self.builtin_types.as_ptr()),
          &mut (*self.module).errors,
          left_type,
          metamethod,
          expr.base.base.location,
        )
      };
      let right_mm = if left_mm.is_none() {
        // Safety: 仅当左侧无元方法时执行；与上一块对 module.errors 的借用
        // 首尾相接（前者已结束），仍是同一构造契约下的 arena 写。
        unsafe {
          find_metatable_entry(
            Handle::from_ptr(self.builtin_types.as_ptr()),
            &mut (*self.module).errors,
            right_type,
            metamethod,
            expr.base.base.location,
          )
        }
      } else {
        None
      };

      if left_mm.or(right_mm).is_some() {
        return;
      }

      if !is_equality
        && !(is_string_operation && (expr.op == AstExprBinaryOp::Concat || is_comparison))
      {
        if (left_mt.is_some() && !is_string(left_type))
          || (right_mt.is_some() && !is_string(right_type))
        {
          if is_comparison {
            self.report_error_type_error_data_location(
              TypeErrorData::CannotCompareUnrelatedTypes(CannotCompareUnrelatedTypes {
                left: left_type,
                right: right_type,
                op: expr.op,
              }),
              &expr.base.base.location,
            );
          } else {
            self.report_error_type_error_data_location(
              TypeErrorData::GenericError(GenericError::new(alloc::format!(
                "Operator {} is not applicable for '{}' and '{}' because neither type's metatable has a '{}' metamethod",
                to_str(expr.op),
                to_string_type_id(left_type),
                to_string_type_id(right_type),
                metamethod
              ))),
              &expr.base.base.location,
            );
          }
          return;
        } else if left_mt.is_none()
          && right_mt.is_none()
          && (get_type::get::<TableType>(left_type).is_some()
            || get_type::get::<TableType>(right_type).is_some())
        {
          if is_comparison {
            self.report_error_type_error_data_location(
              TypeErrorData::CannotCompareUnrelatedTypes(CannotCompareUnrelatedTypes {
                left: left_type,
                right: right_type,
                op: expr.op,
              }),
              &expr.base.base.location,
            );
          } else {
            self.report_error_type_error_data_location(
              TypeErrorData::GenericError(GenericError::new(alloc::format!(
                "Operator {} is not applicable for '{}' and '{}' because neither type has a metatable",
                to_str(expr.op),
                to_string_type_id(left_type),
                to_string_type_id(right_type)
              ))),
              &expr.base.base.location,
            );
          }
          return;
        }
      }
    }

    match expr.op {
      AstExprBinaryOp::Add
      | AstExprBinaryOp::Sub
      | AstExprBinaryOp::Mul
      | AstExprBinaryOp::Div
      | AstExprBinaryOp::FloorDiv
      | AstExprBinaryOp::Pow
      | AstExprBinaryOp::Mod => {
        let number_type = self.builtin_types_ref().number_type;
        self.test_is_subtype_type_id_type_id_location(left_type, number_type, left.base.location);
        self.test_is_subtype_type_id_type_id_location(right_type, number_type, right.base.location);
      }
      AstExprBinaryOp::Concat => {
        // 构造 `number|string` union 写入 module arena：bts 经安全访问器只读
        // 两个 TypeId，arena 写入经 module_mut() 收口（NotNull 契约见访问器）。
        let bts = self.builtin_types_ref();
        let (num_ty, str_ty) = (bts.number_type, bts.string_type);
        let number_or_string = self.module_mut().internal_types.add_type(UnionType {
          options: vec![num_ty, str_ty],
        });
        self.test_is_subtype_type_id_type_id_location(
          left_type,
          number_or_string,
          left.base.location,
        );
        self.test_is_subtype_type_id_type_id_location(
          right_type,
          number_or_string,
          right.base.location,
        );
      }
      AstExprBinaryOp::CompareGe
      | AstExprBinaryOp::CompareGt
      | AstExprBinaryOp::CompareLe
      | AstExprBinaryOp::CompareLt => {
        if norm_left
          .as_ref()
          .is_some_and(|norm| norm.should_suppress_errors())
        {
          return;
        }

        if norm_left.as_ref().is_some_and(|norm| {
          self.normalizer.is_inhabited_normalized_type(norm) == NormalizationResult::False
        }) {
          return;
        }

        // 关系比较的 number 一致性检查：subtyping 已句柄化（`subtyping_mut` 收口
        // 判空与解引用契约，借用止于本条件式）；builtin_types_ref()
        // 只读一个 TypeId；scope 是 stack 顶拷出的 *mut Scope，仅传值不解引用。
        if self
          .subtyping_mut()
          .is_subtype_type_id_type_id_not_null_scope(
            left_type,
            self.builtin_types_ref().number_type,
            scope,
          )
          .is_subtype
        {
          self.test_is_subtype_type_id_type_id_location(
            right_type,
            self.builtin_types_ref().number_type,
            right.base.location,
          );
          return;
        }

        // 与 number 分支同构，仅期望类型换成 string_type；
        // subtyping/builtin_types 借用都止于本条件式。
        if self
          .subtyping_mut()
          .is_subtype_type_id_type_id_not_null_scope(
            left_type,
            self.builtin_types_ref().string_type,
            scope,
          )
          .is_subtype
        {
          self.test_is_subtype_type_id_type_id_location(
            right_type,
            self.builtin_types_ref().string_type,
            right.base.location,
          );
          return;
        }

        self.report_error_type_error_data_location(
          TypeErrorData::GenericError(GenericError::new(alloc::format!(
            "Types '{}' and '{}' cannot be compared with relational operator {}",
            to_string_type_id(left_type),
            to_string_type_id(right_type),
            to_str(expr.op)
          ))),
          &expr.base.base.location,
        );
        return;
      }
      AstExprBinaryOp::And
      | AstExprBinaryOp::Or
      | AstExprBinaryOp::CompareEq
      | AstExprBinaryOp::CompareNe
      | AstExprBinaryOp::OpCount => {}
    }

    let _ = in_context;
    let _ = types_have_intersection;
  }

  pub(crate) fn visit_expr_type_assertion(&mut self, expr: &AstExprTypeAssertion) {
    // expr/annotation 已句柄化恒非空（parser 必建）：.get() 安全借用，后续全走引用。
    let (inner, annotation) = (expr.expr.get(), expr.annotation.get());
    self.visit_expr(inner, ValueContext::RValue);
    self.visit_type(annotation);

    let annotation_type: TypeId = self.lookup_annotation(annotation);
    let computed_type: TypeId = self.lookup_type(inner);

    // SAFETY: should_suppress_errors 为 unsafe fn，normalizer 裸指针借自
    // &mut self 的唯一可变借用、止于各次调用（与原 C++ 传 &normalizer 同构）。
    let suppression: ErrorSuppression = unsafe {
      should_suppress_errors(&mut self.normalizer as *mut _, computed_type).or_else(
        &should_suppress_errors(&mut self.normalizer as *mut _, annotation_type),
      )
    };

    match suppression.error_suppression_value() {
      Value::Suppress => return,
      Value::NormalizationFailed => {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &expr.base.base.location,
        );
        return;
      }
      Value::DoNotSuppress => {}
    }

    match self.normalizer.is_inhabited_type_id(computed_type) {
      NormalizationResult::True => {}
      NormalizationResult::False => return,
      NormalizationResult::HitLimits => {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &expr.base.base.location,
        );
        return;
      }
    }

    match self
      .normalizer
      .is_intersection_inhabited_type_id_type_id(computed_type, annotation_type)
    {
      NormalizationResult::True => (),
      NormalizationResult::False => {
        self.report_error_type_error_data_location(
          TypeErrorData::TypesAreUnrelated(TypesAreUnrelated {
            left: computed_type,
            right: annotation_type,
          }),
          &expr.base.base.location,
        );
      }
      NormalizationResult::HitLimits => {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &expr.base.base.location,
        );
      }
    }
  }

  pub(crate) fn visit_expr_if_else(&mut self, expr: &AstExprIfElse) {
    // Safety: 外层守卫暂存 type_context 地址；_in_context 在函数返回前 drop
    // 还原，期间 &mut self 独占该字段。
    let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);
    {
      // Safety: 嵌套守卫按 RAII 逆序回写（先 Condition 后 Default），两个
      // InConditionalContext 指向同一字段但持有/归还严格嵌套，无重叠写。
      let _in_context_cond =
        InConditionalContext::new(&mut self.type_context, TypeContext::Condition);
      // condition 已句柄化恒非空：.get() 只读借用出自 expr 存活引用，unsafe 契约随类型消失。
      let condition = expr.condition.get();
      self.visit_expr(condition, ValueContext::RValue);
    }
    // 同上——离开条件上下文后再访两个分支表达式，句柄同源。
    let (true_expr, false_expr) = (expr.true_expr.get(), expr.false_expr.get());
    self.visit_expr(true_expr, ValueContext::RValue);
    self.visit_expr(false_expr, ValueContext::RValue);
  }

  pub fn visit_expr_instantiate(&mut self, explicit_type_instantiation: &AstExprInstantiate) {
    // SAFETY: .expr（被实例化的表达式）为 parser 建于 arena 的子指针，
    // 与父节点同寿。
    let expr = unsafe { &*explicit_type_instantiation.expr };
    self.visit_expr(expr, ValueContext::RValue);
    if fflag::LuauExplicitTypeInstantiationSupport.get() {
      // expr 刚被 value-context 访问，类型记录已就绪；type_arguments 为
      // 节点自有 AstArray（arena 元素指针），check_type_instantiation 只读。
      let fn_ty = self.lookup_type(expr);
      let location = explicit_type_instantiation.base.base.location;
      let type_arguments = explicit_type_instantiation.type_arguments;
      self.check_type_instantiation(expr, fn_ty, &location, type_arguments);
    }
  }

  pub(crate) fn visit_expr_interp_string(&mut self, interp_string: &AstExprInterpString) {
    // Safety: 守卫只暂存 self.type_context 字段地址并在本函数尾 drop；
    // 借用严格包含其生命周期。
    let in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

    // .expressions 是 arena 连续 AstArray，iter_nodes 把插值子表达式指针转引用
    // （解引用契约收口在 AstArray::iter_nodes，元素均由 parser 建于 arena）。
    for expr in interp_string.expressions.iter_nodes() {
      self.visit_expr(expr, ValueContext::RValue);
    }

    drop(in_context);
  }

  pub(crate) fn visit_expr_error(&mut self, expr: &AstExprError) {
    // 错误恢复节点的 expressions 数组元素均为 parser 登记的存活子节点
    // （iter_nodes 内部收口解引用契约）。
    for e in expr.expressions.iter_nodes() {
      self.visit_expr(e, ValueContext::RValue);
    }
  }

  /// 类型标注 RTTI 分发（cpp `visit(AstType*)`）：通过安全的 [`AstTypeRef`] 模式匹配具体类型注解；
  /// 未处理类别保持原有的静默跳过行为。
  pub fn visit_type(&mut self, ty: &AstType) {
    // SAFETY: ast_resolved_types 以指针身份查表（C++ module->astResolvedTypes.find
    // 同构），不解引用键；module_ref() 访问器收口 NotNull 解引用契约。
    let resolved_ty = self
      .module_ref()
      .ast_resolved_types
      .find(&(ty as *const AstType))
      .copied();
    if let Some(resolved_ty) = resolved_ty {
      self.check_for_type_function_inhabitance(follow_type::follow(resolved_ty), ty.base.location);
    }

    match ty.as_type_ref() {
      AstTypeRef::Reference(ty) => self.visit_type_reference(ty),
      AstTypeRef::Table(ty) => self.visit_type_table(ty),
      AstTypeRef::Function(ty) => self.visit_type_function(ty),
      AstTypeRef::Typeof(ty) => self.visit_type_typeof(ty),
      AstTypeRef::Union(ty) => self.visit_type_union(ty),
      AstTypeRef::Intersection(ty) => self.visit_type_intersection(ty),
      AstTypeRef::Group(group) => {
        // SAFETY: group.type_ 是括号类型标注的 arena 子指针（parser 必建）。
        self.visit_type(unsafe { &*group.type_ });
      }
      _ => {}
    }
  }

  pub(crate) fn visit_type_reference(&mut self, ty: &AstTypeReference) {
    let location = ty.base.base.location;

    // No further validation is necessary in this case. The main logic for
    // _luau_print is contained in lookupAnnotation.
    // C++: kLuauPrint, kLuauForceConstraintSolvingIncomplete, kLuauBlockedType.
    if fflag::DebugLuauMagicTypes.get() {
      let magic_name = ty.name.as_str_or_empty();
      if matches!(
        magic_name,
        LUAU_PRINT | LUAU_FORCE_CONSTRAINT_SOLVING_INCOMPLETE | LUAU_BLOCKED_TYPE
      ) {
        return;
      }
    }

    let params = ty.parameters;
    for &param in params.as_slice() {
      // 变体分发替代判空哨兵；`as_pack()` 把非 `Pack` 形态折叠为 `None`，与 cpp
      // 向 `visitTypePack(nullptr)` 传空同值（被调方入口判空早退）。
      match param {
        AstTypeOrPack::Type(ty) => self.visit_type(ty),
        _ => self.visit_type_pack(param.as_pack()),
      }
    }

    let scope_ptr = self.find_innermost_scope(location);
    LUAU_ASSERT!(!scope_ptr.is_null());
    // Safety: find_innermost_scope 沿 push_stack 维护的栈查作用域，根作用域
    // 恒存在（C++ NotNull<Scope> 契约）；Scope 实体归 module.scopes 持有。
    let scope = unsafe { &*scope_ptr };

    let name_str: Name = ty.name.as_str_or_empty().to_string();

    let alias: Option<TypeFun> = if let Some(prefix) = ty.prefix {
      let prefix_str: Name = prefix.as_str_or_empty().to_string();
      scope.lookup_imported_type(&prefix_str, &name_str)
    } else {
      scope.lookup_type(&name_str)
    };

    if let Some(ref alias_ref) = alias {
      let types_required = alias_ref.type_params().len();
      let packs_required = alias_ref.type_pack_params().len();

      let mut types_provided = 0usize;
      let mut extra_types = 0usize;
      let mut packs_provided = 0usize;

      for &param in params.as_slice() {
        // cpp `if (param.type) … else if (param.typePack) …`：Error 形态两臂都不进。
        match param {
          AstTypeOrPack::Type(_) => {
            if packs_provided != 0 {
              self.report_error_type_error_data_location(
                GenericError::new(String::from(
                  "Type parameters must come before type pack parameters",
                ))
                .into(),
                &location,
              );
              continue;
            }

            if types_provided < types_required {
              types_provided += 1;
            } else {
              extra_types += 1;
            }
          }
          AstTypeOrPack::Pack(type_pack) => {
            // `NonNull::as_ptr` 只还原变体载荷（arena 存活节点）的同一地址，
            // 与 cpp `param.typePack` 槽值逐位相同。
            let tp = self.lookup_pack_annotation(NonNull::from(type_pack).as_ptr());
            if tp.is_none() {
              continue;
            }

            let tp_id = tp.expect("上方 is_none 分支已 continue，此处必为 Some");
            // C++ `size(*tp) == 1 && finite(*tp) && first(*tp)`.
            if types_provided < types_required
              && size(tp_id, None) == 1
              && unsafe {
                // SAFETY: 同 size——NO_TXN_LOG 哨兵合法，tp_id 为已命中的 TypePackId。
                finite(tp_id, NO_TXN_LOG)
              }
              && first(tp_id, true).is_some()
            {
              types_provided += 1;
            } else {
              packs_provided += 1;
            }
          }
          AstTypeOrPack::Error => {}
        }
      }

      // If we require type parameters, but no type_arguments are provided and only packs are provided, we report an error.
      if types_required != 0 && types_provided == 0 && packs_provided != 0 {
        self.report_error_type_error_data_location(
          GenericError::new(String::from(
            "Type parameters must come before type pack parameters",
          ))
          .into(),
          &location,
        );
      }

      if extra_types != 0 && packs_provided == 0 {
        // Extra type_arguments are only collected into a pack if a pack is expected
        if packs_required != 0 {
          packs_provided += 1;
        } else {
          types_provided += extra_types;
        }
      }

      // 与原 while 循环等价：自 typesProvided 起扫描余下每个带默认值的形参
      // （无默认值也继续计数推进，行为保持一致）。
      types_provided += alias_ref
        .type_params()
        .iter()
        .skip(types_provided)
        .filter(|param| param.default_value.is_some())
        .count();

      packs_provided += alias_ref
        .type_pack_params()
        .iter()
        .skip(packs_provided)
        .filter(|param| param.default_value.is_some())
        .count();

      // If the type parameter list is explicitly provided, allow an empty type pack to satisfy the expected pack count.
      if extra_types == 0 && packs_provided + 1 == packs_required && ty.has_parameter_list {
        packs_provided += 1;
      }

      if types_provided != types_required || packs_provided != packs_required {
        self.report_error_type_error_data_location(
          IncorrectGenericParameterCount {
            name: name_str.clone(),
            type_fun: alias_ref.clone(),
            actual_parameters: types_provided,
            actual_pack_parameters: packs_provided,
          }
          .into(),
          &location,
        );
      }
    } else if scope.lookup_pack(&name_str).is_some() {
      self.report_error_type_error_data_location(
        SwappedGenericTypeParameter {
          name: String::from(ty.name.as_str_or_empty()),
          kind: SwappedGenericTypeParameter::TYPE,
        }
        .into(),
        &location,
      );
    } else {
      let mut symbol = String::new();
      if let Some(prefix) = ty.prefix {
        let prefix_str = prefix.as_str_or_empty().to_string();
        symbol.push_str(&prefix_str);
        symbol.push('.');
      }
      let name_lossy = ty.name.as_str_or_empty();
      symbol.push_str(name_lossy);

      self.report_error_type_error_data_location(
        UnknownSymbol::new(symbol, Context::Type).into(),
        &location,
      );
    }
  }

  pub(crate) fn visit_type_table(&mut self, table: &AstTypeTable) {
    for prop in table.props.iter() {
      // SAFETY: prop.r#type 是属性类型标注的 arena 子指针（parser 随表节点建于
      // arena），与原实现一致直接转共享引用。
      self.visit_type(unsafe { &*prop.r#type });
    }

    // SAFETY: indexer 由 parser 判空写位（缺失索引器为 null），非空即 arena 存活；
    // 其 index_type/result_type 子指针同寿。
    if let Some(indexer) = unsafe { table.indexer.as_ref() } {
      self.visit_type(unsafe { &*indexer.index_type });
      self.visit_type(unsafe { &*indexer.result_type });
    }
  }

  pub(crate) fn visit_type_function(&mut self, ty: &AstTypeFunction) {
    self.visit_generics(ty.generics, ty.generic_packs);
    self.visit_type_list(&ty.arg_types);
    // SAFETY: return_types 是节点自有的存活 AstTypePack 指针（可空，判空由
    // Option 形态承接），解引用契约与原 unsafe 透传一致。
    self.visit_type_pack(unsafe { ty.return_types.as_ref() });
  }

  pub(crate) fn visit_type_typeof(&mut self, ty: &AstTypeTypeof) {
    // SAFETY: typeof 的 expr 字段由 parser 强制存在（无空值场景），转引用后
    // 透传给 value-context 访问器只读递归。
    let expr = unsafe { &*ty.expr };
    self.visit_expr(expr, ValueContext::RValue);
  }

  pub(crate) fn visit_stat_block(&mut self, block: &AstStatBlock) {
    // SAFETY: push_stack 只按指针身份查 ast_scopes；由存活节点引用派生裸指针有效。
    let _stack_pusher = self.push_stack((block as *const AstStatBlock).cast_mut().as_ast_node());

    // body 数组元素为 parser 随 block 建于 arena 的子语句指针
    // （iter_nodes 内部收口解引用契约）。
    for stat in block.body.iter_nodes() {
      self.visit_stat(stat);
    }
  }

  pub(crate) fn visit_type_union(&mut self, ty: &AstTypeUnion) {
    // .types 成员指针由 parser 随 union 节点建于 arena（iter_nodes 收口解引用）。
    for t in ty.types.iter_nodes() {
      self.visit_type(t);
    }
  }

  pub(crate) fn visit_type_intersection(&mut self, ty: &AstTypeIntersection) {
    // 与 union 分支对称——.types 成员同为 arena 存活的 AstType 指针。
    for t in ty.types.iter_nodes() {
      self.visit_type(t);
    }
  }

  /// 类型包 RTTI 分发（cpp `visit(AstTypePack*)`）：null 早退以 `Option` 编码，
  /// 通过安全的 [`AstTypePackRef`] 模式匹配具体类型包。
  pub fn visit_type_pack(&mut self, pack: Option<&AstTypePack>) {
    let Some(pack) = pack else { return };

    match pack.as_pack_ref() {
      AstTypePackRef::Explicit(pack) => self.visit_type_pack_explicit(pack),
      AstTypePackRef::Variadic(pack) => self.visit_type_pack_variadic(pack),
      AstTypePackRef::Generic(pack) => self.visit_type_pack_generic(pack),
    }
  }

  pub(crate) fn visit_type_pack_explicit(&mut self, tp: &AstTypePackExplicit) {
    let type_list = &tp.type_list;
    // type_list.types 成员与 tail_type 均随 explicit 节点建于 arena；
    // tail_type 判空（可空）由 Option 形态承接，解引用契约同原 unsafe 透传。
    for ty in type_list.types.iter_nodes() {
      // SAFETY: 数组元素为存活 AstType 指针（iter_nodes 内部收口）。
      self.visit_type(ty);
    }

    // SAFETY: 同上——tail_type 可空，非空即 arena 存活 AstTypePack。
    self.visit_type_pack(unsafe { type_list.tail_type.as_ref() });
  }

  pub(crate) fn visit_type_pack_variadic(&mut self, tp: &AstTypePackVariadic) {
    // SAFETY: variadic_type（`...T` 的 T）parser 必建，无空值分支。
    self.visit_type(unsafe { &*tp.variadic_type });
  }

  pub(crate) fn visit_type_pack_generic(&mut self, tp: &AstTypePackGeneric) {
    let location = tp.base.base.location;
    let scope_ptr = self.find_innermost_scope(location);
    LUAU_ASSERT!(!scope_ptr.is_null());
    // Safety: generic pack 作用域查询——find_innermost_scope 沿栈回溯，最坏
    // 落到模块作用域（内部对齐 C++ NotNull<Scope> 语义），&* 转 &Scope 只读。
    let scope = unsafe { &*scope_ptr };

    let name = tp.generic_name.as_str_or_empty().to_string();

    if scope.lookup_pack(&name).is_some() {
      return;
    }

    if scope.lookup_type(&name).is_some() {
      let kind = SwappedGenericTypeParameter::PACK;
      let error = SwappedGenericTypeParameter { name, kind };
      let location_ref = &location;
      self.report_error_type_error_data_location(error.into(), location_ref);
      return;
    }

    let context = Context::Type;
    let error = UnknownSymbol::new(name.to_string(), context);
    let location_ref = &location;
    self.report_error_type_error_data_location(error.into(), location_ref);
  }

  pub(crate) fn visit_stat_if(&mut self, if_statement: &AstStatIf) {
    {
      // C++: `InConditionalContext flipper{&typeContext};` uses the default
      // `newValue = TypeContext::Condition` (TypeUtils.h:45). Visiting an if
      // statement's condition must set the conditional context so that
      // `inConditional(typeContext)` is true while checking the predicate.
      // Safety: 守卫引用 self.type_context 字段地址，块尾先于 if_statement 的
      // 后续借用 drop，回写顺序确定。
      let _flipper = InConditionalContext::new(&mut self.type_context, TypeContext::Condition);
      // SAFETY: condition/thenbody 是 parser 必建于 if 节点 arena 的子指针；
      // elsebody 判空（as_ref）后才派发。
      let condition = &if_statement.condition;
      self.visit_expr(condition, ValueContext::RValue);
    }

    let thenbody = &if_statement.thenbody;
    self.visit_stat_block(thenbody);
    if let Some(elsebody) = if_statement.elsebody.as_ref() {
      self.visit_stat(elsebody);
    }
  }

  pub(crate) fn visit_stat_while(&mut self, while_statement: &AstStatWhile) {
    let condition = &while_statement.condition;
    self.visit_expr(condition, ValueContext::RValue);
    let body = &while_statement.body;
    self.visit_stat_block(body);
  }

  pub(crate) fn visit_stat_repeat(&mut self, repeat_statement: &AstStatRepeat) {
    // body/condition 已句柄化为 Node（parser 必建、arena 先于本指针存在后于
    // check 释放，由句柄契约承载），`.get()` 直出安全引用；先访 body 再访
    // condition 与 C++ 顺序一致（Ast.cpp:707/708）。
    let body = repeat_statement.body.get();
    self.visit_stat_block(body);
    let condition = repeat_statement.condition.get();
    self.visit_expr(condition, ValueContext::RValue);
  }

  pub fn visit_stat_break(&mut self, _stat: &AstStatBreak) {}

  pub fn visit_stat_continue(&mut self, _stat: &AstStatContinue) {}

  pub(crate) fn visit_stat_return(&mut self, ret: &AstStatReturn) {
    let location = ret.base.base.location;

    // SAFETY: find_innermost_scope 的返回值至少是模块作用域（内部从
    // module->getModuleScope() 起步，NotNull 语义），此处直接取其 returnType。
    let scope_ptr = self.find_innermost_scope(location);
    let expected_ret_type = unsafe { (*scope_ptr).return_type };

    let list = ret.list;
    if list.size == 0 {
      let empty_type_pack = self.builtin_types_ref().empty_type_pack;
      self.test_is_subtype_type_pack_id_type_pack_id_location(
        empty_type_pack,
        expected_ret_type,
        location,
      );
      return;
    }

    // C++: extendTypePack(module->internalTypes, builtinTypes, expectedRetType, ret->list.size)
    // SAFETY: extend_type_pack 收 &mut module 字段 arena 与 builtin_types 裸值，
    // 两字段指向不同对象、借用止于本次调用（C++ 同参）。
    let extended_pack = unsafe {
      extend_type_pack(
        &mut (*self.module).internal_types,
        Handle::from_ptr(self.builtin_types.as_ptr()),
        expected_ret_type,
        list.size,
        Vec::new(),
      )
    };

    let mut is_subtype = true;
    let mut actual_head: Vec<TypeId> = Vec::new();
    let mut actual_tail: Option<TypePackId> = None;

    let head = &extended_pack.head;
    let head_len = head.len();

    // 最后一个元素单独处理（last_expr），此处遍历前面的元素并与 head 对齐
    for (idx, expr) in list.iter_nodes().take(list.size - 1).enumerate() {
      if let Some(&expected_ty) = head.get(idx) {
        // SAFETY: test_literal_or_ast_type_is_subtype 为 unsafe fn，要求 expr
        // 指向存活 AstExpr——由 iter_nodes 借用反推同一 arena 指针（身份回推）。
        let subtype_result = unsafe {
          self.test_literal_or_ast_type_is_subtype((expr as *const AstExpr).cast_mut(), expected_ty)
        };
        is_subtype &= subtype_result;
        actual_head.push(expected_ty);
      } else {
        let ty = self.lookup_type(expr);
        actual_head.push(ty);
      }
    }

    let last_idx = list.size - 1;
    let last_expr = list.as_slice()[last_idx];
    if head_len < list.size
      || unsafe { ast_node_is_ptr::<AstExprCall>(last_expr) }
      || unsafe { ast_node_is_ptr::<AstExprVarargs>(last_expr) }
    {
      actual_tail = Some(self.lookup_pack(last_expr));
    } else {
      let last_expected_ty = head[last_idx];
      // SAFETY: 同上——last_expr 为界内末元素指针。
      let subtype_result =
        unsafe { self.test_literal_or_ast_type_is_subtype(last_expr, last_expected_ty) };
      is_subtype &= subtype_result;
      actual_head.push(last_expected_ty);
    }

    if is_subtype {
      let reconstructed_ret_type = self
        .module_mut()
        .internal_types
        .add_type_pack_t(TypePack::new(actual_head, actual_tail));
      self.test_is_subtype_type_pack_id_type_pack_id_location(
        reconstructed_ret_type,
        expected_ret_type,
        location,
      );
    }

    for expr in list.iter_nodes() {
      self.visit_expr(expr, ValueContext::RValue);
    }
  }

  pub(crate) fn visit_stat_expr(&mut self, expr: &AstStatExpr) {
    // expr 已句柄化（node_handle::Node）：`.get()` 即 arena 只读视图，
    // 原手写 unsafe 解引用与其契约注释随类型一并消失。
    self.visit_expr(expr.expr.get(), ValueContext::RValue);
  }

  pub(crate) fn visit_stat_local(&mut self, local: &AstStatLocal) {
    let values = local.values;
    let vars = local.vars;

    let count = max(values.size, vars.size);
    // i 是 local 声明的槽位序号：需对长度不等的 values/vars 平行随机访问、
    // 判末值位置（i == values.size-1）并算剩余变量数，保留索引遍历。
    for i in 0..count {
      // SAFETY: values/vars 连续数组元素是 parser 随 local 声明建于 arena 的
      // 子指针（as_slice().get(i) 收口界内取值）；as_ref 对 null 得 None，
      // 越界槽位与原实现一样按空槽（None）处理，ast_node_is 判型在 None 分支
      // 天然短路（RTTI 头只读）。
      let value: Option<&AstExpr> =
        unsafe { values.as_slice().get(i).copied().and_then(|p| p.as_ref()) };
      let is_pack = value.is_some_and(|value| {
        matches!(value.as_expr_ref(), AstExprRef::Call(_) | AstExprRef::Varargs(_))
      });

      if let Some(value) = value {
        self.visit_expr(value, ValueContext::RValue);
      }

      // values.size is usize; `values.size - 1` matches C++ where size is
      // never 0 in this branch when i could equal size-1.
      if i != values.size.wrapping_sub(1) || !is_pack {
        // SAFETY: 同上——as_slice().get(i) 界内取出的槽位为存活 AstLocal 指针，
        // 越界/null 均折叠为 None，与原行为一致。
        let var: Option<&AstLocal> =
          unsafe { vars.as_slice().get(i).copied().and_then(|p| p.as_ref()) };

        // SAFETY: annotation 判空后即 arena 存活 AstType（可空，None 走跳过分支）。
        if let Some(var) = var
          && let Some(annotation) = unsafe { var.annotation.as_ref() }
        {
          let annotation_type = self.lookup_annotation(annotation);
          // cpp: `if (valueType) testPotentialLiteralIsSubtype(value, annotationType);`
          if let Some(value) = value {
            // value 非空 ⇔ 原 valueType.is_some()；保留 lookupType 的记录存在性断言语义。
            let value_type = self.lookup_type(value);
            let _ = value_type;
            self.test_potential_literal_is_subtype(value, annotation_type);
          }

          self.visit_type(annotation);
        }
      } else if let Some(value) = value {
        // SAFETY: lookup_pack 的 unsafe 形参要求存活 AstExpr；由 value 借用
        // 身份回推同一 arena 指针。
        let value_pack = self.lookup_pack((value as *const AstExpr).cast_mut());
        let mut value_types = TypePack::empty();
        if i < vars.size {
          // SAFETY: extend_type_pack 双指针入参（module 字段 arena 与
          // builtin_types 字段）指向不同对象，借用止于本次调用。
          value_types = unsafe {
            extend_type_pack(
              &mut (*self.module).internal_types,
              Handle::from_ptr(self.builtin_types.as_ptr()),
              value_pack,
              vars.size - i,
              Vec::new(),
            )
          };
        }

        let mut error_location = Location::default();
        // 原 `while j < vars.size {…break}` 循坏的迭代形态：skip(i) 起逐个 var
        // 处理，offset 即原 j-i（head 下标），越界时记录 var.location 并终止。
        for (offset, var) in vars.iter_nodes().skip(i).enumerate() {
          if offset >= value_types.head.len() {
            error_location = var.location;
            break;
          }

          // SAFETY: annotation 判空后即 arena 存活 AstType；(value).base.location
          // 处的 value 也已被外层 `Some(value)` 守卫。
          if let Some(annotation) = unsafe { var.annotation.as_ref() } {
            let var_type = self.lookup_annotation(annotation);
            self.test_is_subtype_type_id_type_id_location(
              value_types.head[offset],
              var_type,
              value.base.location,
            );

            self.visit_type(annotation);
          }
        }

        let remaining_vars = vars.size.saturating_sub(i);
        if value_types.head.len() < remaining_vars {
          // values 非空（走到此分支要求 i < values.size 的 value 存在）；
          // ast_node_is 对裸指针形态只做判空 + 读 RTTI 头，null 安全。
          let last_value = values.as_slice()[values.size - 1];
          let kind = if unsafe { ast_node_is_ptr::<AstExprCall>(last_value) } {
            CountMismatch::FUNCTION_RESULT
          } else {
            CountMismatch::EXPR_LIST_RESULT
          };
          self.report_error_type_error_data_location(
            CountMismatch {
              // We subtract 1 here because the final AST expression is
              // not worth one value.  It is worth 0 or more depending on
              // valueTypes.head
              expected: values.size - 1 + value_types.head.len(),
              maximum: None,
              actual: vars.size,
              context: kind,
              is_variadic: false,
              function: String::new(),
            }
            .into(),
            &error_location,
          );
        }
      }
    }
  }

  pub(crate) fn visit_stat_for(&mut self, for_statement: &AstStatFor) {
    // var 已句柄化为非空 Node（parser 必建循环变量，`.get()` 直出引用——
    // Deref 版死 unsafe 消失），annotation 判空后使用；from/to/body 同为非空
    // 句柄，step 落可空 OptNode（由 Option 承接）。
    let var = for_statement.var.get();
    if let Some(annotation) = unsafe { var.annotation.as_ref() } {
      self.visit_type(annotation);
      let annotated_type = self.lookup_annotation(annotation);
      self.test_is_subtype_type_id_type_id_location(
        self.builtin_types_ref().number_type,
        annotated_type,
        var.location,
      );
    }

    // C++ uses a `checkNumber` lambda over [from, to, step]; inlined here.
    // from/to 已句柄化为非空 Node（Some 恒成立），step 落可空 OptNode：
    // cpp lambda 的 nullptr 判空折叠进 `get()` 的 Option。
    self.check_number(Some(for_statement.from.get()));
    self.check_number(Some(for_statement.to.get()));
    self.check_number(for_statement.step.get());

    // body 已句柄化为非空 Node：`.get()` 直出安全引用。
    let body = for_statement.body.get();
    self.visit_stat_block(body);
  }

  fn check_number(&mut self, expr: Option<&AstExpr>) {
    let Some(expr) = expr else { return };
    self.visit_expr(expr, ValueContext::RValue);
    let expr_type = self.lookup_type(expr);
    self.test_is_subtype_type_id_type_id_location(
      expr_type,
      self.builtin_types_ref().number_type,
      expr.base.location,
    );
  }

  pub fn visit_stat_for_in(&mut self, for_in_statement: &AstStatForIn) {
    let vars = for_in_statement.vars;
    let values = for_in_statement.values;
    let body = for_in_statement.body;

    // for (AstLocal* local : forInStatement->vars)
    //     if (local->annotation)
    //         visit(local->annotation);
    let vars_slice = vars.as_slice();
    let values_slice = values.as_slice();
    // 数组元素解引用收口 iter_nodes（cpp 直接解引用槽位，槽位由 parser 建于 arena）。
    for local in vars.iter_nodes() {
      // SAFETY: annotation 可空，判空后即 arena 存活 AstType；None 分支与原
      // is_null 跳过一致。
      if let Some(annotation) = unsafe { local.annotation.as_ref() } {
        self.visit_type(annotation);
      }
    }

    // for (AstExpr* expr : forInStatement->values)
    //     visit(expr, ValueContext::RValue);
    for expr in values.iter_nodes() {
      self.visit_expr(expr, ValueContext::RValue);
    }

    // visit(forInStatement->body);
    // body 已句柄化为非空 Node：`.get()` 直出安全引用，判空门面随类型折叠。
    self.visit_stat_block(body.get());

    // if (!forInStatement->vars.size || !forInStatement->values.size) return;
    if vars_slice.is_empty() || values_slice.is_empty() {
      return;
    }

    // NotNull<Scope> scope = stack.back(); —— 栈元素已句柄化，`scope` 为栈顶
    // `Handle<Scope>`（expect 保证非空栈后拷出）。
    let scope = *self
      .stack
      .last()
      .expect("check 入口已压入模块根 scope，visit 全程栈恒非空（cpp TypeChecker2 stack 不变式）");

    // std::vector<TypeId> variableTypes;
    // for (AstLocal* var : forInStatement->vars)
    // {
    //     std::optional<TypeId> ty = scope->lookup(var);
    //     LUAU_ASSERT(ty);
    //     variableTypes.emplace_back(*ty);
    // }
    let mut variable_types: Vec<TypeId> = Vec::new();
    for &var in vars_slice {
      // scope 是 stack 顶句柄（push_stack 自 module.ast_scopes 取的
      // NotNull 指针），Scope 实体存活于 module 的作用域容器；lookup_symbol
      // 对 Symbol::from_local(var) 仅做只读查表。
      let ty = scope.get().lookup_symbol(Symbol::from_local(var));
      LUAU_ASSERT!(ty.is_some());
      variable_types
        .push(ty.expect("cpp LUAU_ASSERT(ty)：forin 循环变量已由 loop scope 声明，lookup 必命中"));
    }

    // AstExpr* firstValue = forInStatement->values.data[0];
    // SAFETY: values 非空（上方 is_empty 早退），第 0 项是 parser 建于 arena 的
    // 迭代器首表达式（iter_nodes 收口解引用）；转引用后以指针身份查表沿用同一地址。
    let first_value = values
      .iter_nodes()
      .next()
      .expect("values 非空（上方已早退）");

    // std::vector<TypeId> valueTypes;
    // std::optional<TypePackId> iteratorTail;
    let mut value_types: Vec<TypeId> = Vec::new();
    let mut iterator_tail: Option<TypePackId> = None;

    // TypePackId* retPack = module->astTypePacks.find(firstValue);
    // if (retPack) { auto [head, tail] = flatten(*retPack); valueTypes = head; iteratorTail = tail; }
    // else valueTypes.emplace_back(lookupType(firstValue));
    // 以指针身份查 astTypePacks；module_ref() 为安全访问器。
    let ret_pack = self
      .module_ref()
      .ast_type_packs
      .find(&(first_value as *const AstExpr))
      .copied();
    if let Some(ret_pack) = ret_pack {
      let (head, tail) = flatten_type_pack_id(ret_pack);
      value_types = head;
      iterator_tail = tail;
    } else {
      let ty = self.lookup_type(first_value);
      value_types.push(ty);
    }

    // TypeId* resolvedTy = module->astForInNextTypes.find(firstValue);
    // if (resolvedTy && (!retPack || valueTypes.size() > 1))
    //     valueTypes[0] = *resolvedTy;
    // 同为安全访问器 module_ref() 上的指针身份查表。
    let resolved_ty = self
      .module_ref()
      .ast_for_in_next_types
      .find(&(first_value as *const AstExpr).cast::<AstNode>())
      .copied();
    if let Some(resolved_ty) = resolved_ty
      && (ret_pack.is_none() || value_types.len() > 1)
    {
      value_types[0] = resolved_ty;
    }

    if values_slice.len() > 2 {
      // cpp `for (i = 1; i < size - 1; ++i)` 同序：skip(1).take(len-2) 覆盖中段
      // 迭代器表达式（如 pairs(t) 的 t），iter_nodes 收口元素解引用。
      for expr in values.iter_nodes().skip(1).take(values.size - 2) {
        let ty = self.lookup_type(expr);
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
    // 经安全访问器 module_mut() 收口裸指针解引用，行为与原 unsafe 相同。
    let iterator_pack = self
      .module_mut()
      .internal_types
      .add_type_pack_vector_type_id_optional_type_pack_id(value_types, iterator_tail);

    // TypePack iteratorTypes = extendTypePack(arena, builtinTypes, iteratorPack, 3);
    // Safety: extend_type_pack 为 unsafe fn。arena 可变借用与 builtin_types
    // 裸指针指向 checker 的两个不相交字段，按裸指针拆分传入以避免 &mut self
    // 整借冲突；两者生命周期均覆盖返回值的使用（C++ 同参数）。
    let iterator_types = unsafe {
      let arena = &mut (*self.module).internal_types;
      extend_type_pack(
        arena,
        Handle::from_ptr(self.builtin_types.as_ptr()),
        iterator_pack,
        3,
        Vec::new(),
      )
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
    let iterator_ty = follow_type::follow(iterator_types.head[0]);

    // std::shared_ptr<const NormalizedType> iteratorNorm = normalizer.normalize(iteratorTy);
    // 归一化过于复杂时与 C++ 一致报错（不返回，后续按空指针分支处理）
    let iterator_norm = self.normalizer.try_normalize(iterator_ty);
    if iterator_norm.is_none() {
      let loc = Self::first_value_location(first_value);
      self.report_error_type_error_data_location(NormalizationTooComplex::default().into(), &loc);
    }

    if let Some(next_fn) = get_type::get::<FunctionType>(iterator_ty) {
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
    } else if let Some(ttv) = get_type::get::<TableType>(iterator_ty) {
      // if ((vars.size == 1 || vars.size == 2) && ttv->indexer)
      if (vars.size == 1 || vars.size == 2)
        && let Some(indexer) = ttv.indexer.as_ref()
      {
        // testIsSubtype(variableTypes[0], ttv->indexer->index_type, vars.data[0]->location);
        // SAFETY: vars 非空（函数开头 is_empty 早退），首/次槽位是 arena 内存活的
        // AstLocal（iter_nodes 收口解引用），只读其 location。
        let mut var_locs = vars.iter_nodes().map(|v| v.location);
        let v0_loc = var_locs.next().expect("vars 非空（上方已早退）");
        self.test_is_subtype_type_id_type_id_location(
          variable_types[0],
          indexer.index_type,
          v0_loc,
        );
        if variable_types.len() == 2 {
          // variable_types.len()==2 蕴含 vars.size==2，次槽位同样存活。
          let v1_loc = var_locs.next().expect("vars.size == 2");
          self.test_is_subtype_type_id_type_id_location(
            variable_types[1],
            indexer.index_result_type,
            v1_loc,
          );
        }
      } else {
        // reportError(GenericError{"..."}, forInStatement->values.data[0]->location);
        let loc = Self::first_value_location(first_value);
        self.report_error_type_error_data_location(
          GenericError::new("Cannot iterate over a table without indexer".to_string()).into(),
          &loc,
        );
      }
    } else if get_type::get::<AnyType>(iterator_ty).is_some()
      || get_type::get::<ErrorType>(iterator_ty).is_some()
      || get_type::get::<NeverType>(iterator_ty).is_some()
    {
      // nothing
    } else if is_optional(iterator_ty)
      && !iterator_norm
        .as_ref()
        .is_some_and(|n| n.should_suppress_errors())
    {
      // reportError(OptionalValueAccess{iteratorTy}, forInStatement->values.data[0]->location);
      let loc = first_value.base.location;
      self.report_error_type_error_data_location(
        OptionalValueAccess {
          optional: iterator_ty,
        }
        .into(),
        &loc,
      );
    } else if let Some(iter_mm_ty) = unsafe {
      // Safety: 解引用 module.errors 的可变借用止于 find_metatable_entry（safe
      // fn）调用；builtin_types 按裸值传递，两字段互不相交。
      find_metatable_entry(
        Handle::from_ptr(self.builtin_types.as_ptr()),
        &mut (*self.module).errors,
        iterator_ty,
        "__iter",
        Self::first_value_location(first_value),
      )
    } {
      // Instantiation instantiation{TxnLog::empty(), &arena, builtinTypes, TypeLevel{}, scope};
      // Safety: instantiation_new 是 safe fn（参数收句柄），unsafe 仅在把
      // module.internal_types 取可变引用包成句柄；instantiation 的存活期短于
      // 本作用域的 module/scope（TxnLog::empty() 为无主哨兵，同 C++）。
      let mut instantiation = unsafe {
        Instantiation::instantiation_new(
          TxnLog::empty(),
          Some(Handle::from_mut(&mut (*self.module).internal_types)),
          self.builtin_types,
          TypeLevel::default(),
          scope.as_ptr(),
        )
      };

      // if (std::optional<TypeId> instantiatedIterMmTy = instantiate(builtinTypes, NotNull{&arena}, limits, scope, *iterMmTy))
      // Safety: instantiate 本身是 safe fn，unsafe 只在三处指针操作：
      // self.builtin_types.get() 读 NotNull 语义字段（构造期注入、会话内有效）；
      // &mut (*self.module).internal_types 与上一借用同源自救（同一条语句里
      // 读两个裸指针字段，借用检查器无法证明 builtin_types 与 module 不相交，
      // 故按 C++ 多指针传参口径拆分）；scope 来自上方 find_innermost_scope。
      if let Some(instantiated_iter_mm_ty) = unsafe {
        instantiate(
          self.builtin_types.get(),
          &mut (*self.module).internal_types,
          self.limits_ref(),
          scope.as_ptr(),
          iter_mm_ty,
        )
      } {
        // if (const FunctionType* iterMmFtv = get<FunctionType>(*instantiatedIterMmTy))
        if let Some(iter_mm_ftv) = get_type::get::<FunctionType>(instantiated_iter_mm_ty) {
          let iter_mm_arg_types = iter_mm_ftv.arg_types;
          let iter_mm_ret_types = iter_mm_ftv.ret_types;

          // TypePackId argPack = arena.addTypePack({iteratorTy});
          // 经安全访问器 module_mut() 收口裸指针解引用，行为与原 unsafe 相同。
          let arg_pack = self
            .module_mut()
            .internal_types
            .add_type_pack_vector_type_id_optional_type_pack_id(vec![iterator_ty], None);
          // testIsSubtype(argPack, iterMmFtv->argTypes, forInStatement->values.data[0]->location);
          self.test_is_subtype_type_pack_id_type_pack_id_location(
            arg_pack,
            iter_mm_arg_types,
            Self::first_value_location(first_value),
          );

          // TypePack mmIteratorTypes = extendTypePack(arena, builtinTypes, iterMmFtv->retTypes, 3);
          // Safety: extend_type_pack 为 unsafe fn，收 &mut arena 与 *mut
          // BuiltinTypes 两个指针。arena 借自 (*self.module) 字段、
          // self.builtin_types.as_ptr() 是另一独立 NotNull 字段，二者指向不同对象；
          // 借用在调用语句内结束，instantiation 内保存的旧 arena 裸指针
          // 此时未被并发使用（对应 C++ 同一 FreeTypeVars 顺序传参）。
          let mm_iterator_types = unsafe {
            let arena = &mut (*self.module).internal_types;
            extend_type_pack(
              arena,
              Handle::from_ptr(self.builtin_types.as_ptr()),
              iter_mm_ret_types,
              3,
              Vec::new(),
            )
          };

          // if (mmIteratorTypes.head.size() == 0)
          if mm_iterator_types.head.is_empty() {
            let loc = Self::first_value_location(first_value);
            self.report_error_type_error_data_location(
              GenericError::new("__iter must return at least one value".to_string()).into(),
              &loc,
            );
            return;
          }

          // TypeId nextFn = follow(mmIteratorTypes.head[0]);
          let next_fn = follow_type::follow(mm_iterator_types.head[0]);

          // if (std::optional<TypeId> instantiatedNextFn = instantiation.substitute(nextFn))
          if let Some(instantiated_next_fn) = instantiation.base.substitute_type_id(next_fn) {
            // std::vector<TypeId> instantiatedIteratorTypes = mmIteratorTypes.head;
            // instantiatedIteratorTypes[0] = *instantiatedNextFn;
            let mut instantiated_iterator_types = mm_iterator_types.head;
            instantiated_iterator_types[0] = instantiated_next_fn;

            // if (const FunctionType* nextFtv = get<FunctionType>(*instantiatedNextFn))
            if let Some(next_ftv) = get_type::get::<FunctionType>(instantiated_next_fn) {
              self.check_function_for_in(
                next_ftv,
                instantiated_iterator_types,
                true,
                for_in_statement,
                first_value,
                &variable_types,
              );
            } else if !self.is_error_suppressing_location_type_id(
              Self::first_value_location(first_value),
              instantiated_next_fn,
            ) {
              let loc = Self::first_value_location(first_value);
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
            let loc = Self::first_value_location(first_value);
            self
              .report_error_type_error_data_location(UnificationTooComplex::default().into(), &loc);
          }
        } else if !self.is_error_suppressing_location_type_id(
          Self::first_value_location(first_value),
          instantiated_iter_mm_ty,
        ) {
          // reportError(CannotCallNonFunction{*iterMmTy}, forInStatement->values.data[0]->location);
          let loc = Self::first_value_location(first_value);
          self.report_error_type_error_data_location(
            CannotCallNonFunction { ty: iter_mm_ty }.into(),
            &loc,
          );
        }
      } else {
        // reportError(UnificationTooComplex{}, forInStatement->values.data[0]->location);
        let loc = Self::first_value_location(first_value);
        self.report_error_type_error_data_location(UnificationTooComplex::default().into(), &loc);
      }
    } else if iterator_norm.as_ref().is_some_and(|n| n.has_tables()) {
      // Ok. All tables can be iterated.
    } else if !iterator_norm
      .as_ref()
      .is_some_and(|n| n.should_suppress_errors())
    {
      // reportError(CannotCallNonFunction{iteratorTy}, forInStatement->values.data[0]->location);
      let loc = Self::first_value_location(first_value);
      self.report_error_type_error_data_location(
        CannotCallNonFunction { ty: iterator_ty }.into(),
        &loc,
      );
    }
  }

  /// C++ `getLocation(firstValue)` 的引用形态：全部调用点取自 ForIn 的
  /// values 数组第 0 项（或经其函数契约传入的同一借用），只读 base.location。
  fn first_value_location(first_value: &AstExpr) -> Location {
    first_value.base.location
  }

  /// C++ `getLocation(forInStatement->values)` — span from the first value's begin
  /// to the last value's end (`Ast.h:1715`). Returns the all-zero span when empty.
  fn values_location(for_in_statement: &AstStatForIn) -> Location {
    let slice = for_in_statement.values.as_slice();
    match (slice.first(), slice.last()) {
      // Safety: first/last 是 for_in_statement.values 切片的头尾元素——
      // parser 随 AstStatForIn 在 arena 建好的表达式节点，只读各自
      // base.location 拼出区间。
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
    first_value: &AstExpr,
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
    // Safety: extend_type_pack 为 unsafe fn：arena 可变借用取自
    // (*self.module) 字段、builtin_types 为独立指针字段，二者指向不同对象，
    // 借用止于本次调用；迭代器返回类型 iter_ftv_ret_types 由调用方
    // checkFunction 契约保证已在本模块类型池中。
    let expected_variable_types = unsafe {
      let arena = &mut (*self.module).internal_types;
      extend_type_pack(
        arena,
        Handle::from_ptr(self.builtin_types.as_ptr()),
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
        let loc = first_value.base.location;
        self.report_error_type_error_data_location(
          GenericError::new("next() does not return enough values".to_string()).into(),
          &loc,
        );
      }
      return;
    }

    // if (get<ErrorType>(follow(flattenPack(iterFtv->argTypes)))) return;
    let flattened = self.flatten_pack(iter_ftv_arg_types);
    if get_type::get::<ErrorType>(follow_type::follow(flattened)).is_some() {
      return;
    }

    // auto [minCount, maxCount] = getParameterExtents(TxnLog::empty(), iterFtv->argTypes, true);
    // Safety: get_parameter_extents 是 unsafe fn，要求 log 可读。传入
    // TxnLog::empty()——static OnceLock 支撑的 *const 哨兵，永不失效（对应
    // C++ 默认真实参 TxnLog* log 的用法）；tp 为 TypePackId 值索引，其
    // 解析前提由上面 flatten_pack 同一数据源已成立。
    let (min_count, _max_count) =
      unsafe { get_parameter_extents(TxnLog::empty(), iter_ftv_arg_types, true) };

    // TypePack flattenedArgTypes = extendTypePack(arena, builtinTypes, iterFtv->argTypes, 2);
    // Safety: 同 expected_variable_types 的构造口径——unsafe 仅覆盖
    // extend_type_pack 的两指针入参（module 字段 arena + builtin_types 字段），
    // 这次展开的是迭代器参数包、补齐到 2 元。
    let _flattened_arg_types = unsafe {
      let arena = &mut (*self.module).internal_types;
      extend_type_pack(
        arena,
        Handle::from_ptr(self.builtin_types.as_ptr()),
        iter_ftv_arg_types,
        2,
        Vec::new(),
      )
    };
    // size_t firstIterationArgCount = iterTys.empty() ? 0 : iterTys.size() - 1;
    let first_iteration_arg_count = iter_tys.len().saturating_sub(1);
    // size_t actualArgCount = expectedVariableTypes.head.size();
    let actual_arg_count = expected_variable_types.head.len();

    // cpp 的两个分支（firstIterationArgCount < minCount，以及 else if
    // actualArgCount < minCount）报错体逐字相同、且都以 return 收尾，
    // 这里合并为一次判定，消除重复块（TypeChecker2.cpp:1066-1083）。
    if first_iteration_arg_count < min_count || actual_arg_count < min_count {
      if is_mm {
        let loc = Self::values_location(for_in_statement);
        self.report_error_type_error_data_location(
          GenericError::new("__iter metamethod must return (next[, table[, state]])".to_string())
            .into(),
          &loc,
        );
      } else {
        let loc = first_value.base.location;
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
    let iter_func = follow_type::follow(iter_tys[0]);

    // std::vector<TypeId> prospectiveArgTypes = std::vector(iterTys.begin() + 1, iterTys.end());
    let mut prospective_arg_types: Vec<TypeId> = iter_tys[1..].to_vec();
    // if (const TypePack* iterFuncArgs = get<TypePack>(follow(iterFtv->argTypes));
    //     iterFuncArgs && iterFuncArgs->head.size() > prospectiveArgTypes.size())
    //     prospectiveArgTypes.resize(iterFuncArgs->head.size(), builtinTypes->nil_type);
    if let Some(iter_func_args) =
      get_type_pack::get::<TypePack>(follow_type_pack::follow(iter_ftv_arg_types))
      && iter_func_args.head.len() > prospective_arg_types.len()
    {
      // 经 builtin_types_ref() 安全访问器取值，替代裸指针解引用（行为不变）。
      prospective_arg_types.resize(iter_func_args.head.len(), self.builtin_types_ref().nil_type);
    }
    // const TypePackId prospectiveArgs = arena.addTypePack(prospectiveArgTypes, std::nullopt);
    // 经安全访问器 module_mut() 收口，原 unsafe 只是 (*self.module) 单字段解引用。
    let prospective_args = self
      .module_mut()
      .internal_types
      .add_type_pack_vector_type_id_optional_type_pack_id(prospective_arg_types, None);

    // std::vector<TypeId> prospectiveRetTypes = {};
    let mut prospective_ret_types: Vec<TypeId> = Vec::new();
    // if (variableTypes.size() > 0)
    //     prospectiveRetTypes.emplace_back(arena.addType(UnionType{{variableTypes[0], builtinTypes->nil_type}}));
    if !variable_types.is_empty() {
      // 变量首类型 ∪ nil 的 union 写入 module arena：nil_type 先经安全访问器
      // 取出，arena 经 module_mut() 收口，整块不再需要 unsafe。
      let nil_ty = self.builtin_types_ref().nil_type;
      let added = self.module_mut().internal_types.add_type(UnionType {
        options: vec![variable_types[0], nil_ty],
      });
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
      get_type_pack::get::<TypePack>(follow_type_pack::follow(iter_ftv_ret_types))
      && iter_func_rets.head.len() > prospective_ret_types.len()
    {
      // 参数侧按 TypePack head 长度补 nil、返回侧补 any——这里补 any：
      // any_type 经安全访问器读取，无裸指针解引用残留。
      prospective_ret_types.resize(iter_func_rets.head.len(), self.builtin_types_ref().any_type);
    }
    // const TypePackId prospectiveRets = arena.addTypePack(prospectiveRetTypes, builtinTypes->anyTypePack);
    // 同上收口：any_type_pack 先经访问器取值，arena 借用经 module_mut()。
    let any_type_pack = self.builtin_types_ref().any_type_pack;
    let prospective_rets = self
      .module_mut()
      .internal_types
      .add_type_pack_vector_type_id_optional_type_pack_id(
        prospective_ret_types,
        Some(any_type_pack),
      );

    // const TypeId prospectiveFunction = arena.addType(FunctionType{prospectiveArgs, prospectiveRets, std::nullopt, isMm});
    // 最后一步：prospectiveFunction 组合刚登记的两个 TypePackId（值为索引，
    // 不持有借用），arena 经 module_mut() 收口后此块无 unsafe。
    let prospective_function =
      self
        .module_mut()
        .internal_types
        .add_type(FunctionType::function_type_new(
          prospective_args,
          prospective_rets,
          None,
          is_mm,
        ));

    // testIsSubtypeForInStat(iterFunc, prospectiveFunction, *forInStatement);
    self.test_is_subtype_for_in_stat(iter_func, prospective_function, for_in_statement);
  }

  pub fn visit_stat_assign(&mut self, assign: &AstStatAssign) {
    // iter_nodes 收口 vars/values arena 数组元素的解引用契约（cpp 双层同 zip）。
    for (lhs, rhs) in assign.vars.iter_nodes().zip(assign.values.iter_nodes()) {
      self.visit_expr(lhs, ValueContext::LValue);
      let lhs_type = self.lookup_type(lhs);

      self.visit_expr(rhs, ValueContext::RValue);
      let rhs_type = self.lookup_type(rhs);

      if get_type::get::<NeverType>(lhs_type).is_some() {
        self.report_errors_from_assigning_to_never(lhs, rhs_type);
        continue;
      }

      // SAFETY: test_literal_or_ast_type_is_subtype 为 unsafe fn，要求 expr
      // 指向存活 AstExpr——由 iter_nodes 借用的身份回推同一 arena 指针。
      if unsafe {
        self.test_literal_or_ast_type_is_subtype((rhs as *const AstExpr).cast_mut(), lhs_type)
      } && let Some(binding_type) = self.get_binding_type(lhs)
      {
        // SAFETY: 同一 rhs 指针回推，字面量窄化复检与被调契约一致。
        unsafe {
          self.test_literal_or_ast_type_is_subtype((rhs as *const AstExpr).cast_mut(), binding_type)
        };
      }
    }
  }

  pub(crate) fn visit_stat_compound_assign(&mut self, stat: &AstStatCompoundAssign) {
    let location = stat.base.base.location;
    let op = stat.op;
    let var = stat.var;
    let value = stat.value;

    // C++: AstExprBinary fake{stat->location, stat->op, stat->var, stat->value}; visit(&fake, stat);
    // var/value 已句柄化：与 AstExprBinary.left/right 同型，原
    // `NonNull::new(..).expect(..)` 非空重建与其契约注释一并消失。
    let fake = AstExprBinary::new(location, op, var, value);
    self.visit_expr_binary(&fake, Some(stat));

    // SAFETY: ast_compound_assign_result_types 以指针身份查表（写入方为
    // constraint 生成，同址键），不解引用；module_ref() 收口 NotNull 借用。
    let result_ty = self
      .module_ref()
      .ast_compound_assign_result_types
      .find(&(from_ref(stat).cast::<AstStat>()))
      .copied();

    if self.module_ref().constraint_generation_did_not_complete && result_ty.is_none() {
      return;
    }

    LUAU_ASSERT!(result_ty.is_some());
    // Safety: cpp LUAU_ASSERT(result_ty)：constraint 生成期已为该复合赋值登记结果类型。
    let result_ty =
      result_ty.expect("cpp LUAU_ASSERT(result_ty)：constraint 生成期已为该复合赋值登记结果类型");
    // SAFETY: var 是复合赋值目标（&AstStatCompoundAssign 的 arena 子指针），
    // var 已句柄化：`.get()` 即 arena 只读视图，原 unsafe 解引用消失。
    let var_ty = self.lookup_type(var.get());

    self.test_is_subtype_type_id_type_id_location(result_ty, var_ty, location);
  }

  pub(crate) fn visit_stat_function(&mut self, stat: &AstStatFunction) {
    // name（函数名表达式）与 func（函数体字面量）已句柄化为 Node（parser 必建，
    // 非空由类型层承载），`.get()` 直出安全引用，与原 unsafe 透传等价。
    let name = stat.name.get();
    let func = stat.func.get();

    self.visit_expr(name, ValueContext::LValue);
    self.visit_expr_function(func);

    if fflag::LuauCheckFunctionStatementTypes.get() {
      // name 刚被 LValue 访问、func 刚被访问，两者的类型记录都已建立，只读比对。
      let lhs_type = self.lookup_type(name);
      let rhs_type = self.lookup_type(&func.base);
      let location = func.base.base.location;
      self.test_is_subtype_type_id_type_id_location(rhs_type, lhs_type, location);
    }

    if fflag::DebugLuauWarnOnUnannotatedTopLevelFunctions.get() && self.stack.len() == 1 {
      // name 已句柄化，`.get()` 即安全只读视图，仅读其 location。
      let name_location = stat.name.get().base.location;
      self.check_function_annotations(func, AnnotationCheckMode::Function, name_location);
    }
  }

  pub(crate) fn visit_stat_local_function(&mut self, stat: &AstStatLocalFunction) {
    // func/name 已句柄化为 Node（parser 必建非空由类型层承载），`.get()` 直出
    // 安全引用，与原 unsafe 透传语义一致。
    let func = stat.func.get();
    self.visit_expr_function(func);

    if fflag::DebugLuauWarnOnUnannotatedTopLevelFunctions.get() && self.stack.len() == 1 {
      let name_location = stat.name.get().location;
      self.check_function_annotations(func, AnnotationCheckMode::Function, name_location);
    }
  }

  pub fn visit_type_list(&mut self, type_list: &AstTypeList) {
    // types 数组成员由 parser 随父节点建于 arena（iter_nodes 收口解引用契约）；
    // tail_type 可空，判空由 Option 形态承接后交给 pack 访问器。
    for ty in type_list.types.iter_nodes() {
      self.visit_type(ty);
    }

    // SAFETY: tail_type 为空或指向存活 AstTypePack（父节点自有子指针）。
    self.visit_type_pack(unsafe { type_list.tail_type.as_ref() });
  }

  pub(crate) fn visit_stat_type_alias(&mut self, stat: &AstStatTypeAlias) {
    // We will not visit type aliases that do not have an associated scope,
    // this means that (probably) this was a duplicate type alias or a
    // type alias with an illegal name (like `typeof`).
    // 经安全访问器 module_ref() 收口单字段解引用，行为与原 unsafe 相同：
    // ast_scopes 以 AstNode 指针身份为键，引用反推指针与登记时同址。
    if !self
      .module_ref()
      .ast_scopes
      .contains_key(&(from_ref(stat).cast::<AstNode>()))
    {
      return;
    }

    // SAFETY: find_innermost_scope 至少返回模块作用域，此处判空兜底 C++ NotNull；
    // Scope 实体归 module.scopes 持有，is_invalid_type_alias 只读递归检查。
    let scope = self.find_innermost_scope(stat.base.base.location);
    if !scope.is_null()
      && let Some(loc) = unsafe { (*scope).is_invalid_type_alias(stat.name.as_str_or_empty()) }
    {
      self.report_error_type_error_data_location(
        TypeErrorData::RecursiveRestraintViolation(RecursiveRestraintViolation::default()),
        &loc,
      );
    }

    // generics/generic_packs 是别名声明泛型参数表（arena 数组），只读遍历登记符号。
    self.visit_generics(stat.generics, stat.generic_packs);

    // SAFETY: type_ptr 为别名右值 AstType（parser 必建），转共享引用后访问。
    let type_ptr = unsafe { &*stat.type_ptr };
    self.visit_type(type_ptr);
  }

  pub fn visit_stat_type_function(&mut self, stat: &AstStatTypeFunction) {
    // SAFETY: .body 为 function 型必建的函数体字面量（非空 arena 子指针），
    // &* 仅换取 &AstExprFunction 供只读访问。
    self.visit_expr_function(unsafe { &*stat.body });
  }

  pub(crate) fn visit_stat_declare_function(&mut self, stat: &AstStatDeclareFunction) {
    self.visit_generics(stat.generics, stat.generic_packs);
    // params 是声明节点自有的 AstTypeList 字段（& 只读遍历其 arena 数组）；
    // ret_types 为节点自有的存活 AstTypePack 指针，可空由 Option 承接。
    self.visit_type_list(&stat.params);
    // SAFETY: ret_types 判空即 arena 存活（原 unsafe 透传同一契约）。
    self.visit_type_pack(unsafe { stat.ret_types.as_ref() });
  }

  pub(crate) fn visit_stat_declare_global(&mut self, stat: &AstStatDeclareGlobal) {
    // SAFETY: .type_ 为声明标注的存活 AstType 指针（parser 必建）。
    let type_ = unsafe { &*stat.type_ };
    self.visit_type(type_);
  }

  pub(crate) fn visit_stat_declare_extern_type(&mut self, stat: &AstStatDeclareExternType) {
    // props 数组每项的 ty 是属性类型标注的 arena 指针（iter 只读遍历）。
    for prop in stat.props.iter() {
      // SAFETY: prop.ty 随 extern 声明建于 arena，转引用与原透传等价。
      self.visit_type(unsafe { &*prop.ty });
    }
  }

  pub fn visit_stat_class(&mut self, stat: &AstStatClass) {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());

    // members 借用只延至循环结束；成员按 get_if 实际类别探测（property/method）。
    for member in stat.members.iter() {
      if let Some(prop) = member.get_if::<AstClassProperty>() {
        // SAFETY: prop.ty 判空后即 arena 存活 AstType。
        if let Some(ty) = unsafe { prop.ty.as_ref() } {
          self.visit_type(ty);
        }
      } else if let Some(method) = member.get_if::<AstClassMethod>() {
        // SAFETY: method.function 是 parser 为类方法必建的函数体字面量；
        // visit_constructor 为 unsafe fn，要求 stat/method 沿类成员 arena 链
        // 存活——两者均借自 &AstStatClass 形参的存活子节点，构造器检查只读
        // 位置与类型字段；check_function_annotations 同样只读该函数体字面量
        // 的标注字段与 location（cpp `TypeChecker2.cpp:1493-1499` 同构）。
        unsafe {
          self.visit_expr_function(&*method.function);
          if method.function_name == "__init" {
            self.check_function_annotations(
              &*method.function,
              AnnotationCheckMode::Constructor,
              method.name_location,
            );
            self.visit_constructor(from_ref(stat).cast_mut(), method);
          } else {
            self.check_function_annotations(
              &*method.function,
              AnnotationCheckMode::Method,
              method.name_location,
            );
          }
        }
      } else {
        LUAU_ASSERT!(false);
      }
    }
  }

  pub(crate) fn visit_stat_error(&mut self, stat: &AstStatError) {
    // 错误恢复节点的 expressions/statements 数组元素皆为 parser 登记的
    // 存活子节点（iter_nodes 收口解引用契约）。
    for e in stat.expressions.iter_nodes() {
      self.visit_expr(e, ValueContext::RValue);
    }
    for s in stat.statements.iter_nodes() {
      self.visit_stat(s);
    }
  }

  /// 表达式 RTTI 分发（cpp `visit(AstExpr*, ValueContext)`）：通过安全的
  /// [`AstExprRef`] 模式匹配具体表达式类型。
  pub fn visit_expr(&mut self, expr: &AstExpr, context: ValueContext) {
    let node = &expr.base;
    // SAFETY: push_stack 只按指针身份查 ast_scopes 并压/弹栈帧；AstNode 是
    // #[repr(C)] 各节点的 0 号字段，从存活 &AstExpr 反推的裸指针同址有效。
    let _pusher = self.push_stack((node as *const AstNode).cast_mut());

    match expr.as_expr_ref() {
      AstExprRef::Group(expr) => self.visit_expr_group(expr, context),
      AstExprRef::ConstantNil(expr) => self.visit_expr_constant_nil(expr),
      AstExprRef::ConstantBool(expr) => {
        self.visit_expr_constant_bool(expr);
      }
      AstExprRef::ConstantNumber(expr) => {
        self.visit_expr_constant_number(expr);
      }
      AstExprRef::ConstantInteger(expr) => {
        self.visit_expr_constant_integer(expr);
      }
      AstExprRef::ConstantString(expr) => {
        self.visit_expr_constant_string(expr);
      }
      AstExprRef::Local(expr) => self.visit_expr_local(expr),
      AstExprRef::Global(expr) => self.visit_expr_global(expr),
      AstExprRef::Varargs(expr) => self.visit_expr_varargs(expr),
      AstExprRef::Call(expr) => self.visit_expr_call(expr),
      AstExprRef::IndexName(expr) => {
        self.visit_expr_index_name(expr, context);
      }
      AstExprRef::IndexExpr(expr) => {
        self.visit_expr_index_expr(expr, context);
      }
      AstExprRef::Function(expr) => self.visit_expr_function(expr),
      AstExprRef::Table(expr) => self.visit_expr_table(expr),
      AstExprRef::Unary(expr) => self.visit_expr_unary(expr),
      AstExprRef::Binary(expr) => {
        // override_key 传 None——对应 C++ `visit(expr, nullptr)` 默认实参，
        // 仅复合赋值调用点会带键（见 visit_stat_compound_assign）。
        self.visit_expr_binary(expr, None);
      }
      AstExprRef::TypeAssertion(expr) => {
        self.visit_expr_type_assertion(expr);
      }
      AstExprRef::IfElse(expr) => self.visit_expr_if_else(expr),
      AstExprRef::Instantiate(expr) => self.visit_expr_instantiate(expr),
      AstExprRef::InterpString(expr) => self.visit_expr_interp_string(expr),
      AstExprRef::Error(expr) => self.visit_expr_error(expr),
    }
  }
}
