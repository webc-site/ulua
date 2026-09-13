//! Faithful port of `copyError` (Analysis/src/Error.cpp:1463-1705).
//!
//! The C++ original is a function template whose body is a chain of
//! `if constexpr (std::is_same_v<T, ...>)` branches, so the active branch is
//! selected at compile time from the concrete error type `T` the caller
//! instantiates it with. The faithful Rust equivalent of that compile-time
//! type switch is a trait: `copy_error<T>` forwards to `T`'s `CopyError`
//! implementation, and each branch of the original `if constexpr` cascade
//! becomes one `impl CopyError for <ErrorType>` whose body is exactly that
//! branch (empty for the error kinds that hold no `TypeId`/`TypePackId`).
//!
//! `::Luau::clone(ty, destArena, cloneState)` is an overload set keyed on the
//! argument type; the ports preserve that as three separate free functions
//! (`TypeId`, `TypePackId`, `TypeFun`), wrapped below as `clone_type`,
//! `clone_pack` and `clone_type_fun`.

/// `clone(TypeId, ...)` overload.
use alloc::sync::Arc;

use crate::{
  functions::{clone_clone::clone, clone_clone_alt_b, clone_clone_alt_c},
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
    type_arena::TypeArena, type_fun::TypeFun,
    type_instantiation_count_mismatch::TypeInstantiationCountMismatch, type_mismatch::TypeMismatch,
    type_pack_mismatch::TypePackMismatch, types_are_unrelated::TypesAreUnrelated,
    unapplied_type_function::UnappliedTypeFunction,
    unexpected_array_like_table_item::UnexpectedArrayLikeTableItem,
    unexpected_type_in_subtyping::UnexpectedTypeInSubtyping,
    unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping,
    unification_too_complex::UnificationTooComplex,
    uninhabited_type_function::UninhabitedTypeFunction,
    uninhabited_type_pack_function::UninhabitedTypePackFunction,
    unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    unknown_property::UnknownProperty, unknown_require::UnknownRequire,
    unknown_symbol::UnknownSymbol, user_defined_type_function_error::UserDefinedTypeFunctionError,
    where_clause_needed::WhereClauseNeeded,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};
fn clone_type(ty: TypeId, dest_arena: &mut TypeArena, clone_state: &mut CloneState) -> TypeId {
  unsafe { clone_clone_alt_b::clone(ty, dest_arena, clone_state) }
}

/// `clone(TypePackId, ...)` overload.
fn clone_pack(
  tp: TypePackId,
  dest_arena: &mut TypeArena,
  clone_state: &mut CloneState,
) -> TypePackId {
  unsafe { clone(tp, dest_arena, clone_state) }
}

/// `clone(TypeFun, ...)` overload.
fn clone_type_fun(
  tf: &TypeFun,
  dest_arena: &mut TypeArena,
  clone_state: &mut CloneState,
) -> TypeFun {
  clone_clone_alt_c::clone(tf, dest_arena, clone_state)
}

/// Faithful realization of the `if constexpr` type switch in `copyError`.
///
/// One `impl` per error kind, each carrying exactly the branch body from the
/// C++ source.
pub trait CopyError {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState);
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
fn visit_error_data(
  data: &mut TypeErrorData,
  dest_arena: &mut TypeArena,
  clone_state: &mut CloneState,
) {
  match data {
    TypeErrorData::TypeMismatch(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnknownSymbol(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnknownProperty(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::NotATable(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CannotExtendTable(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CannotCompareUnrelatedTypes(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::OnlyTablesCanHaveMethods(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::DuplicateTypeDefinition(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CountMismatch(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::FunctionDoesNotTakeSelf(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::FunctionRequiresSelf(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::OccursCheckFailed(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnknownRequire(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::IncorrectGenericParameterCount(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::SyntaxError(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CodeTooComplex(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnificationTooComplex(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnknownPropButFoundLikeProp(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::GenericError(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::InternalError(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::ConstraintSolvingIncompleteError(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CannotCallNonFunction(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::ExtraInformation(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::DeprecatedApiUsed(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::ModuleHasCyclicDependency(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::IllegalRequire(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::FunctionExitsWithoutReturning(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::DuplicateGenericParameter(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CannotAssignToNever(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CannotInferBinaryOperation(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::MissingProperties(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::SwappedGenericTypeParameter(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::OptionalValueAccess(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::MissingUnionProperty(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::TypesAreUnrelated(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::NormalizationTooComplex(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::TypePackMismatch(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(e) => {
      copy_error(e, dest_arena, clone_state)
    }
    TypeErrorData::UninhabitedTypeFunction(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UninhabitedTypePackFunction(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::WhereClauseNeeded(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::PackWhereClauseNeeded(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CheckedFunctionCallError(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::NonStrictFunctionDefinitionError(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::PropertyAccessViolation(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CheckedFunctionIncorrectArgs(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnexpectedTypeInSubtyping(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnexpectedTypePackInSubtyping(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::ExplicitFunctionAnnotationRecommended(e) => {
      copy_error(e, dest_arena, clone_state)
    }
    TypeErrorData::UserDefinedTypeFunctionError(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::BuiltInTypeFunctionError(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::ReservedIdentifier(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnexpectedArrayLikeTableItem(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::CannotCheckDynamicStringFormatCalls(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::GenericTypeCountMismatch(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::GenericTypePackCountMismatch(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::MultipleNonviableOverloads(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::RecursiveRestraintViolation(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::GenericBoundsMismatch(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::UnappliedTypeFunction(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::InstantiateGenericsOnNonFunction(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::TypeInstantiationCountMismatch(e) => copy_error(e, dest_arena, clone_state),
    TypeErrorData::AmbiguousFunctionCall(e) => copy_error(e, dest_arena, clone_state),
  }
}

// ---------------------------------------------------------------------------
// One impl per `if constexpr` branch of `copyError`.
// ---------------------------------------------------------------------------

impl CopyError for TypeMismatch {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.wanted_type = clone_type(self.wanted_type, dest_arena, clone_state);
    self.given_type = clone_type(self.given_type, dest_arena, clone_state);

    if let Some(error) = self.error.as_mut() {
      visit_error_data(&mut Arc::make_mut(error).data, dest_arena, clone_state);
    }
  }
}

impl CopyError for UnknownSymbol {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for UnknownProperty {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.table = clone_type(self.table, dest_arena, clone_state);
  }
}

impl CopyError for NotATable {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.ty = clone_type(self.ty, dest_arena, clone_state);
  }
}

impl CopyError for CannotExtendTable {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.table_type = clone_type(self.table_type, dest_arena, clone_state);
  }
}

impl CopyError for CannotCompareUnrelatedTypes {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.left = clone_type(self.left, dest_arena, clone_state);
    self.right = clone_type(self.right, dest_arena, clone_state);
  }
}

impl CopyError for OnlyTablesCanHaveMethods {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.table_type = clone_type(self.table_type, dest_arena, clone_state);
  }
}

impl CopyError for DuplicateTypeDefinition {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for CountMismatch {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for FunctionDoesNotTakeSelf {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for FunctionRequiresSelf {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for OccursCheckFailed {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for UnknownRequire {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for IncorrectGenericParameterCount {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.type_fun = clone_type_fun(&self.type_fun, dest_arena, clone_state);
  }
}

impl CopyError for SyntaxError {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for CodeTooComplex {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for UnificationTooComplex {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for UnknownPropButFoundLikeProp {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.table = clone_type(self.table, dest_arena, clone_state);
  }
}

impl CopyError for GenericError {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for InternalError {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for ConstraintSolvingIncompleteError {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for CannotCallNonFunction {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.ty = clone_type(self.ty, dest_arena, clone_state);
  }
}

impl CopyError for ExtraInformation {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for DeprecatedApiUsed {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for ModuleHasCyclicDependency {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for IllegalRequire {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for FunctionExitsWithoutReturning {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.expected_return_type = clone_pack(self.expected_return_type, dest_arena, clone_state);
  }
}

impl CopyError for DuplicateGenericParameter {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for CannotInferBinaryOperation {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for MissingProperties {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.super_type = clone_type(self.super_type, dest_arena, clone_state);
    self.sub_type = clone_type(self.sub_type, dest_arena, clone_state);
  }
}

impl CopyError for SwappedGenericTypeParameter {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for OptionalValueAccess {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.optional = clone_type(self.optional, dest_arena, clone_state);
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

impl CopyError for TypesAreUnrelated {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.left = clone_type(self.left, dest_arena, clone_state);
    self.right = clone_type(self.right, dest_arena, clone_state);
  }
}

impl CopyError for NormalizationTooComplex {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for TypePackMismatch {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.wanted_tp = clone_pack(self.wanted_tp, dest_arena, clone_state);
    self.given_tp = clone_pack(self.given_tp, dest_arena, clone_state);
  }
}

impl CopyError for DynamicPropertyLookupOnExternTypesUnsafe {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.ty = clone_type(self.ty, dest_arena, clone_state);
  }
}

impl CopyError for UninhabitedTypeFunction {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.ty = clone_type(self.ty, dest_arena, clone_state);
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

impl CopyError for UninhabitedTypePackFunction {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.tp = clone_pack(self.tp, dest_arena, clone_state);
  }
}

impl CopyError for WhereClauseNeeded {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.ty = clone_type(self.ty, dest_arena, clone_state);
  }
}

impl CopyError for PackWhereClauseNeeded {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.tp = clone_pack(self.tp, dest_arena, clone_state);
  }
}

impl CopyError for CheckedFunctionCallError {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.expected = clone_type(self.expected, dest_arena, clone_state);
    self.passed = clone_type(self.passed, dest_arena, clone_state);
  }
}

impl CopyError for NonStrictFunctionDefinitionError {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.argument_type = clone_type(self.argument_type, dest_arena, clone_state);
  }
}

impl CopyError for PropertyAccessViolation {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.table = clone_type(self.table, dest_arena, clone_state);
  }
}

impl CopyError for CheckedFunctionIncorrectArgs {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for UnexpectedTypeInSubtyping {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.ty = clone_type(self.ty, dest_arena, clone_state);
  }
}

impl CopyError for UnexpectedTypePackInSubtyping {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.tp = clone_pack(self.tp, dest_arena, clone_state);
  }
}

impl CopyError for UserDefinedTypeFunctionError {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for BuiltInTypeFunctionError {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for CannotAssignToNever {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.rhs_type = clone_type(self.rhs_type, dest_arena, clone_state);

    for ty in self.cause.iter_mut() {
      *ty = clone_type(*ty, dest_arena, clone_state);
    }
  }
}

impl CopyError for UnexpectedArrayLikeTableItem {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for ReservedIdentifier {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for CannotCheckDynamicStringFormatCalls {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for GenericTypeCountMismatch {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for GenericTypePackCountMismatch {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for MultipleNonviableOverloads {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for RecursiveRestraintViolation {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
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

impl CopyError for InstantiateGenericsOnNonFunction {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for TypeInstantiationCountMismatch {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.function_type = clone_type(self.function_type, dest_arena, clone_state);
  }
}

impl CopyError for UnappliedTypeFunction {
  fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for AmbiguousFunctionCall {
  fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
    self.function = clone_type(self.function, dest_arena, clone_state);
    self.arguments = clone_pack(self.arguments, dest_arena, clone_state);
  }
}
