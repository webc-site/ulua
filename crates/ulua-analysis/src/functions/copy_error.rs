//! Faithful port of `copyError` (Analysis/src/Error.cpp:1463-1705).
//!
//! The C++ original is a function template whose body is a chain of
//! `if constexpr (std::is_same_v<T, ...>)` branches, so the active branch is
//! selected at compile time from the concrete error type `T` the caller
//! instantiates it with. The faithful Rust equivalent of that compile-time
//! type switch is a trait: `copy_error<T>` forwards to `T`'s `CopyError`
//! implementation, and each branch of the original `if constexpr` cascade
//! becomes one `impl CopyError for <ErrorType>` whose body is exactly that
//! branch. Branches that hold no `TypeId`/`TypePackId` are empty in C++, so
//! they rely on the trait's default no-op body via an empty impl marker
//! (`impl_copy_error_noop!`), and only data-carrying kinds carry a real impl.
//!
//! `::Luau::clone(ty, destArena, cloneState)` is an overload set keyed on the
//! argument type; the ports preserve that as three separate free functions
//! (`TypeId`, `TypePackId`, `TypeFun`), wrapped below as `clone_type`,
//! `clone_pack` and `clone_type_fun`.

/// `clone(TypeId, ...)` overload.
use alloc::sync::Arc;

use crate::{
  functions::{clone_clone, clone_clone::clone},
  records::{
    ambiguous_function_call::AmbiguousFunctionCall,
    built_in_type_function_error::BuiltInTypeFunctionError,
    cannot_assign_to_never::CannotAssignToNever, cannot_call_non_function::CannotCallNonFunction,
    cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls,
    cannot_compare_unrelated_types::CannotCompareUnrelatedTypes,
    cannot_extend_table::CannotExtendTable,
    cannot_infer_binary_operation::CannotInferBinaryOperation,
    checked_function_call_error::CheckedFunctionCallError,
    checked_function_incorrect_args::CheckedFunctionIncorrectArgs, clone_state::CloneState,
    code_too_complex::CodeTooComplex,
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    count_mismatch::CountMismatch, deprecated_api_used::DeprecatedApiUsed,
    duplicate_generic_parameter::DuplicateGenericParameter,
    duplicate_type_definition::DuplicateTypeDefinition,
    dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe,
    explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended,
    extra_information::ExtraInformation, function_does_not_take_self::FunctionDoesNotTakeSelf,
    function_exits_without_returning::FunctionExitsWithoutReturning,
    function_requires_self::FunctionRequiresSelf, generic_bounds_mismatch::GenericBoundsMismatch,
    generic_error::GenericError, generic_type_count_mismatch::GenericTypeCountMismatch,
    generic_type_pack_count_mismatch::GenericTypePackCountMismatch,
    illegal_require::IllegalRequire,
    incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction,
    internal_error::InternalError, missing_properties::MissingProperties,
    missing_union_property::MissingUnionProperty,
    module_has_cyclic_dependency::ModuleHasCyclicDependency,
    multiple_nonviable_overloads::MultipleNonviableOverloads,
    non_strict_function_definition_error::NonStrictFunctionDefinitionError,
    normalization_too_complex::NormalizationTooComplex, not_a_table::NotATable,
    occurs_check_failed::OccursCheckFailed, only_tables_can_have_methods::OnlyTablesCanHaveMethods,
    optional_value_access::OptionalValueAccess, pack_where_clause_needed::PackWhereClauseNeeded,
    property_access_violation::PropertyAccessViolation,
    recursive_restraint_violation::RecursiveRestraintViolation,
    reserved_identifier::ReservedIdentifier,
    swapped_generic_type_parameter::SwappedGenericTypeParameter, syntax_error::SyntaxError,
    type_annotation_required::TypeAnnotationRequired, type_arena::TypeArena, type_fun::TypeFun,
    type_instantiation_count_mismatch::TypeInstantiationCountMismatch, type_mismatch::TypeMismatch,
    type_pack_mismatch::TypePackMismatch, types_are_unrelated::TypesAreUnrelated,
    unapplied_type_function::UnappliedTypeFunction,
    unexpected_array_like_table_item::UnexpectedArrayLikeTableItem,
    unexpected_type_in_subtyping::UnexpectedTypeInSubtyping,
    unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping,
    unification_too_complex::UnificationTooComplex,
    uninhabited_type_function::UninhabitedTypeFunction,
    uninhabited_type_pack_function::UninhabitedTypePackFunction,
    uninitialized_field_access::UninitializedFieldAccess,
    unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    unknown_property::UnknownProperty, unknown_require::UnknownRequire,
    unknown_symbol::UnknownSymbol, user_defined_type_function_error::UserDefinedTypeFunctionError,
    where_clause_needed::WhereClauseNeeded,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};
fn clone_type(ty: TypeId, dest_arena: &mut TypeArena, clone_state: &mut CloneState) -> TypeId {
  // `ty` 是错误对象里借用自源类型 arena 的存活句柄，copy_error 全程在 arena 存活期内调用。
  clone_clone::clone_type_id(ty, dest_arena, clone_state)
}

/// `clone(TypePackId, ...)` overload.
fn clone_pack(
  tp: TypePackId,
  dest_arena: &mut TypeArena,
  clone_state: &mut CloneState,
) -> TypePackId {
  clone(tp, dest_arena, clone_state)
}

/// `clone(TypeFun, ...)` overload.
fn clone_type_fun(
  tf: &TypeFun,
  dest_arena: &mut TypeArena,
  clone_state: &mut CloneState,
) -> TypeFun {
  clone_clone::clone_type_fun(tf, dest_arena, clone_state)
}

/// Faithful realization of the `if constexpr` type switch in `copyError`.
///
/// 持有 `TypeId`/`TypePackId` 的错误种类覆写 `copy_error_impl` 做深拷贝；
/// C++ 中分支体为空的种类直接用默认 no-op 实现。
pub trait CopyError {
  /// 默认分支体：无类型字段的错误种类在 C++ `copyError` 里为空操作。
  fn copy_error_impl(&mut self, _: &mut TypeArena, _: &mut CloneState) {}
}

/// 为分支体为空的错误种类批量启用 `CopyError`（空 impl 标记，走默认 no-op）。
macro_rules! impl_copy_error_noop {
  ($($t:ident),* $(,)?) => {
    $(impl CopyError for $t {})*
  };
}

impl_copy_error_noop! {
  UnknownSymbol, DuplicateTypeDefinition, CountMismatch, FunctionDoesNotTakeSelf,
  FunctionRequiresSelf, OccursCheckFailed, UnknownRequire, SyntaxError, CodeTooComplex,
  UnificationTooComplex, GenericError, InternalError, ConstraintSolvingIncompleteError,
  ExtraInformation, DeprecatedApiUsed, ModuleHasCyclicDependency, IllegalRequire,
  DuplicateGenericParameter, CannotInferBinaryOperation, SwappedGenericTypeParameter,
  NormalizationTooComplex, CheckedFunctionIncorrectArgs, UserDefinedTypeFunctionError,
  BuiltInTypeFunctionError, UnexpectedArrayLikeTableItem, ReservedIdentifier,
  CannotCheckDynamicStringFormatCalls, GenericTypeCountMismatch, GenericTypePackCountMismatch,
  MultipleNonviableOverloads, RecursiveRestraintViolation, InstantiateGenericsOnNonFunction,
  UnappliedTypeFunction, UninitializedFieldAccess,
}

/// 为“仅逐字段深拷贝 `TypeId`/`TypePackId`”的同构分支批量生成 `CopyError` impl。
///
/// 每条 `类型 { 字段 = 克隆门面, .. }` 展开为一枚与手写字段序、赋值序逐字等价的
/// impl；空操作走 [`impl_copy_error_noop!`]，带嵌套错误/循环/引用载荷的分支
/// （TypeMismatch、MissingUnionProperty 等）保留手写实现。
macro_rules! impl_copy_error_clones {
  ($($t:ident { $($field:ident = $clone:ident),* $(,)? }),* $(,)?) => {
    $(
      impl CopyError for $t {
        fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
          $(self.$field = $clone(self.$field, dest_arena, clone_state);)*
        }
      }
    )*
  };
}

/// `template<typename T> void copyError(T& e, TypeArena& destArena, CloneState& cloneState)`.
pub fn copy_error<T: CopyError>(
  e: &mut T,
  dest_arena: &mut TypeArena,
  clone_state: &mut CloneState,
) {
  e.copy_error_impl(dest_arena, clone_state);
}

/// Re-dispatch over a nested `TypeErrorData`, mirroring the C++
/// `visit(visitErrorData, e.error->data)` recursion. Selects the concrete
/// branch for the active variant and forwards to its `CopyError` impl.
///
/// 65 个变体的转发规则完全同构（`copy_error(载荷)`），用宏展开同构 match 臂；
/// 宏生成的 match 受编译器穷尽性检查约束，`TypeErrorData` 新增变体而未在此
/// 登记时直接编译失败，不会静默漏派发。
pub(crate) fn visit_error_data(
  data: &mut TypeErrorData,
  dest_arena: &mut TypeArena,
  clone_state: &mut CloneState,
) {
  macro_rules! forward_copy_error {
    ($($variant:ident),* $(,)?) => {
      match data {
        $(TypeErrorData::$variant(e) => copy_error(e, dest_arena, clone_state),)*
      }
    };
  }
  forward_copy_error! {
    TypeMismatch, UnknownSymbol, UnknownProperty, NotATable, CannotExtendTable,
    CannotCompareUnrelatedTypes, OnlyTablesCanHaveMethods, DuplicateTypeDefinition,
    CountMismatch, FunctionDoesNotTakeSelf, FunctionRequiresSelf, OccursCheckFailed,
    UnknownRequire, IncorrectGenericParameterCount, SyntaxError, CodeTooComplex,
    UnificationTooComplex, UnknownPropButFoundLikeProp, GenericError, InternalError,
    ConstraintSolvingIncompleteError, CannotCallNonFunction, ExtraInformation, DeprecatedApiUsed,
    ModuleHasCyclicDependency, IllegalRequire, FunctionExitsWithoutReturning,
    DuplicateGenericParameter, CannotAssignToNever, CannotInferBinaryOperation, MissingProperties,
    SwappedGenericTypeParameter, OptionalValueAccess, MissingUnionProperty, TypesAreUnrelated,
    NormalizationTooComplex, TypePackMismatch, DynamicPropertyLookupOnExternTypesUnsafe,
    UninhabitedTypeFunction, UninhabitedTypePackFunction, WhereClauseNeeded, PackWhereClauseNeeded,
    CheckedFunctionCallError, NonStrictFunctionDefinitionError, PropertyAccessViolation,
    CheckedFunctionIncorrectArgs, UnexpectedTypeInSubtyping, UnexpectedTypePackInSubtyping,
    ExplicitFunctionAnnotationRecommended, UserDefinedTypeFunctionError, BuiltInTypeFunctionError,
    ReservedIdentifier, UnexpectedArrayLikeTableItem, CannotCheckDynamicStringFormatCalls,
    GenericTypeCountMismatch, GenericTypePackCountMismatch, MultipleNonviableOverloads,
    RecursiveRestraintViolation, GenericBoundsMismatch, UnappliedTypeFunction,
    InstantiateGenericsOnNonFunction, TypeInstantiationCountMismatch, AmbiguousFunctionCall,
    UninitializedFieldAccess, TypeAnnotationRequired,
  }
}

// ---------------------------------------------------------------------------
// One impl per data-carrying `if constexpr` branch of `copyError`.
// ---------------------------------------------------------------------------
// 以下 25 个分支体只把 `TypeId`/`TypePackId` 字段逐个克隆，形状完全同构，
// 收口为 [`impl_copy_error_clones!`] 登记表；其余分支保留手写 impl。
impl_copy_error_clones! {
  UnknownProperty { table = clone_type },
  NotATable { ty = clone_type },
  CannotExtendTable { table_type = clone_type },
  CannotCompareUnrelatedTypes { left = clone_type, right = clone_type },
  OnlyTablesCanHaveMethods { table_type = clone_type },
  UnknownPropButFoundLikeProp { table = clone_type },
  CannotCallNonFunction { ty = clone_type },
  FunctionExitsWithoutReturning { expected_return_type = clone_pack },
  MissingProperties { super_type = clone_type, sub_type = clone_type },
  OptionalValueAccess { optional = clone_type },
  TypesAreUnrelated { left = clone_type, right = clone_type },
  TypePackMismatch { wanted_tp = clone_pack, given_tp = clone_pack },
  DynamicPropertyLookupOnExternTypesUnsafe { ty = clone_type },
  UninhabitedTypeFunction { ty = clone_type },
  UninhabitedTypePackFunction { tp = clone_pack },
  WhereClauseNeeded { ty = clone_type },
  PackWhereClauseNeeded { tp = clone_pack },
  CheckedFunctionCallError { expected = clone_type, passed = clone_type },
  NonStrictFunctionDefinitionError { argument_type = clone_type },
  PropertyAccessViolation { table = clone_type },
  UnexpectedTypeInSubtyping { ty = clone_type },
  UnexpectedTypePackInSubtyping { tp = clone_pack },
  TypeInstantiationCountMismatch { function_type = clone_type },
  AmbiguousFunctionCall { function = clone_type, arguments = clone_pack },
  TypeAnnotationRequired { inferred_ty = clone_type },
}

impl CopyError for TypeMismatch {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.wanted_type = clone_type(self.wanted_type, dest_arena, clone_state);
    self.given_type = clone_type(self.given_type, dest_arena, clone_state);

    if let Some(error) = self.error.as_mut() {
      visit_error_data(&mut Arc::make_mut(error).data, dest_arena, clone_state);
    }
  }
}

impl CopyError for IncorrectGenericParameterCount {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.type_fun = clone_type_fun(&self.type_fun, dest_arena, clone_state);
  }
}

impl CopyError for MissingUnionProperty {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.r#type = clone_type(self.r#type, dest_arena, clone_state);

    for ty in self.missing.iter_mut() {
      *ty = clone_type(*ty, dest_arena, clone_state);
    }
  }
}

impl CopyError for CannotAssignToNever {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.rhs_type = clone_type(self.rhs_type, dest_arena, clone_state);

    for ty in self.cause.iter_mut() {
      *ty = clone_type(*ty, dest_arena, clone_state);
    }
  }
}

impl CopyError for GenericBoundsMismatch {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    for lower_bound in self.lower_bounds.iter_mut() {
      *lower_bound = clone_type(*lower_bound, dest_arena, clone_state);
    }
    for upper_bound in self.upper_bounds.iter_mut() {
      *upper_bound = clone_type(*upper_bound, dest_arena, clone_state);
    }
  }
}

impl CopyError for ExplicitFunctionAnnotationRecommended {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.recommended_return = clone_type(self.recommended_return, dest_arena, clone_state);
    for (_, t) in self.recommended_args.iter_mut() {
      *t = clone_type(*t, dest_arena, clone_state);
    }
  }
}
