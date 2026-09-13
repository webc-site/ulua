//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Error.h:605:type_error_data`
//! Source: `Analysis/include/Luau/Error.h:605-670` (hand-ported)
use crate::records::{
  ambiguous_function_call::AmbiguousFunctionCall,
  built_in_type_function_error::BuiltInTypeFunctionError,
  cannot_assign_to_never::CannotAssignToNever, cannot_call_non_function::CannotCallNonFunction,
  cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls,
  cannot_compare_unrelated_types::CannotCompareUnrelatedTypes,
  cannot_extend_table::CannotExtendTable,
  cannot_infer_binary_operation::CannotInferBinaryOperation,
  checked_function_call_error::CheckedFunctionCallError,
  checked_function_incorrect_args::CheckedFunctionIncorrectArgs, code_too_complex::CodeTooComplex,
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
  generic_type_pack_count_mismatch::GenericTypePackCountMismatch, illegal_require::IllegalRequire,
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
  type_instantiation_count_mismatch::TypeInstantiationCountMismatch, type_mismatch::TypeMismatch,
  type_pack_mismatch::TypePackMismatch, types_are_unrelated::TypesAreUnrelated,
  unapplied_type_function::UnappliedTypeFunction,
  unexpected_array_like_table_item::UnexpectedArrayLikeTableItem,
  unexpected_type_in_subtyping::UnexpectedTypeInSubtyping,
  unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping,
  unification_too_complex::UnificationTooComplex,
  uninhabited_type_function::UninhabitedTypeFunction,
  uninhabited_type_pack_function::UninhabitedTypePackFunction,
  unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp, unknown_property::UnknownProperty,
  unknown_require::UnknownRequire, unknown_symbol::UnknownSymbol,
  user_defined_type_function_error::UserDefinedTypeFunctionError,
  where_clause_needed::WhereClauseNeeded,
};

// 63 members -> custom enum; ORDER preserves C++ Variant positions.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeErrorData {
  TypeMismatch(TypeMismatch),
  UnknownSymbol(UnknownSymbol),
  UnknownProperty(UnknownProperty),
  NotATable(NotATable),
  CannotExtendTable(CannotExtendTable),
  CannotCompareUnrelatedTypes(CannotCompareUnrelatedTypes),
  OnlyTablesCanHaveMethods(OnlyTablesCanHaveMethods),
  DuplicateTypeDefinition(DuplicateTypeDefinition),
  CountMismatch(CountMismatch),
  FunctionDoesNotTakeSelf(FunctionDoesNotTakeSelf),
  FunctionRequiresSelf(FunctionRequiresSelf),
  OccursCheckFailed(OccursCheckFailed),
  UnknownRequire(UnknownRequire),
  IncorrectGenericParameterCount(IncorrectGenericParameterCount),
  SyntaxError(SyntaxError),
  CodeTooComplex(CodeTooComplex),
  UnificationTooComplex(UnificationTooComplex),
  UnknownPropButFoundLikeProp(UnknownPropButFoundLikeProp),
  GenericError(GenericError),
  InternalError(InternalError),
  ConstraintSolvingIncompleteError(ConstraintSolvingIncompleteError),
  CannotCallNonFunction(CannotCallNonFunction),
  ExtraInformation(ExtraInformation),
  DeprecatedApiUsed(DeprecatedApiUsed),
  ModuleHasCyclicDependency(ModuleHasCyclicDependency),
  IllegalRequire(IllegalRequire),
  FunctionExitsWithoutReturning(FunctionExitsWithoutReturning),
  DuplicateGenericParameter(DuplicateGenericParameter),
  CannotAssignToNever(CannotAssignToNever),
  CannotInferBinaryOperation(CannotInferBinaryOperation),
  MissingProperties(MissingProperties),
  SwappedGenericTypeParameter(SwappedGenericTypeParameter),
  OptionalValueAccess(OptionalValueAccess),
  MissingUnionProperty(MissingUnionProperty),
  TypesAreUnrelated(TypesAreUnrelated),
  NormalizationTooComplex(NormalizationTooComplex),
  TypePackMismatch(TypePackMismatch),
  DynamicPropertyLookupOnExternTypesUnsafe(DynamicPropertyLookupOnExternTypesUnsafe),
  UninhabitedTypeFunction(UninhabitedTypeFunction),
  UninhabitedTypePackFunction(UninhabitedTypePackFunction),
  WhereClauseNeeded(WhereClauseNeeded),
  PackWhereClauseNeeded(PackWhereClauseNeeded),
  CheckedFunctionCallError(CheckedFunctionCallError),
  NonStrictFunctionDefinitionError(NonStrictFunctionDefinitionError),
  PropertyAccessViolation(PropertyAccessViolation),
  CheckedFunctionIncorrectArgs(CheckedFunctionIncorrectArgs),
  UnexpectedTypeInSubtyping(UnexpectedTypeInSubtyping),
  UnexpectedTypePackInSubtyping(UnexpectedTypePackInSubtyping),
  ExplicitFunctionAnnotationRecommended(ExplicitFunctionAnnotationRecommended),
  UserDefinedTypeFunctionError(UserDefinedTypeFunctionError),
  BuiltInTypeFunctionError(BuiltInTypeFunctionError),
  ReservedIdentifier(ReservedIdentifier),
  UnexpectedArrayLikeTableItem(UnexpectedArrayLikeTableItem),
  CannotCheckDynamicStringFormatCalls(CannotCheckDynamicStringFormatCalls),
  GenericTypeCountMismatch(GenericTypeCountMismatch),
  GenericTypePackCountMismatch(GenericTypePackCountMismatch),
  MultipleNonviableOverloads(MultipleNonviableOverloads),
  RecursiveRestraintViolation(RecursiveRestraintViolation),
  GenericBoundsMismatch(GenericBoundsMismatch),
  UnappliedTypeFunction(UnappliedTypeFunction),
  InstantiateGenericsOnNonFunction(InstantiateGenericsOnNonFunction),
  TypeInstantiationCountMismatch(TypeInstantiationCountMismatch),
  AmbiguousFunctionCall(AmbiguousFunctionCall),
}

impl TypeErrorData {
  /// C++ `v.index()` — the member's position in the Variant<...> list.
  pub fn index(&self) -> i32 {
    match self {
      TypeErrorData::TypeMismatch(_) => 0,
      TypeErrorData::UnknownSymbol(_) => 1,
      TypeErrorData::UnknownProperty(_) => 2,
      TypeErrorData::NotATable(_) => 3,
      TypeErrorData::CannotExtendTable(_) => 4,
      TypeErrorData::CannotCompareUnrelatedTypes(_) => 5,
      TypeErrorData::OnlyTablesCanHaveMethods(_) => 6,
      TypeErrorData::DuplicateTypeDefinition(_) => 7,
      TypeErrorData::CountMismatch(_) => 8,
      TypeErrorData::FunctionDoesNotTakeSelf(_) => 9,
      TypeErrorData::FunctionRequiresSelf(_) => 10,
      TypeErrorData::OccursCheckFailed(_) => 11,
      TypeErrorData::UnknownRequire(_) => 12,
      TypeErrorData::IncorrectGenericParameterCount(_) => 13,
      TypeErrorData::SyntaxError(_) => 14,
      TypeErrorData::CodeTooComplex(_) => 15,
      TypeErrorData::UnificationTooComplex(_) => 16,
      TypeErrorData::UnknownPropButFoundLikeProp(_) => 17,
      TypeErrorData::GenericError(_) => 18,
      TypeErrorData::InternalError(_) => 19,
      TypeErrorData::ConstraintSolvingIncompleteError(_) => 20,
      TypeErrorData::CannotCallNonFunction(_) => 21,
      TypeErrorData::ExtraInformation(_) => 22,
      TypeErrorData::DeprecatedApiUsed(_) => 23,
      TypeErrorData::ModuleHasCyclicDependency(_) => 24,
      TypeErrorData::IllegalRequire(_) => 25,
      TypeErrorData::FunctionExitsWithoutReturning(_) => 26,
      TypeErrorData::DuplicateGenericParameter(_) => 27,
      TypeErrorData::CannotAssignToNever(_) => 28,
      TypeErrorData::CannotInferBinaryOperation(_) => 29,
      TypeErrorData::MissingProperties(_) => 30,
      TypeErrorData::SwappedGenericTypeParameter(_) => 31,
      TypeErrorData::OptionalValueAccess(_) => 32,
      TypeErrorData::MissingUnionProperty(_) => 33,
      TypeErrorData::TypesAreUnrelated(_) => 34,
      TypeErrorData::NormalizationTooComplex(_) => 35,
      TypeErrorData::TypePackMismatch(_) => 36,
      TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(_) => 37,
      TypeErrorData::UninhabitedTypeFunction(_) => 38,
      TypeErrorData::UninhabitedTypePackFunction(_) => 39,
      TypeErrorData::WhereClauseNeeded(_) => 40,
      TypeErrorData::PackWhereClauseNeeded(_) => 41,
      TypeErrorData::CheckedFunctionCallError(_) => 42,
      TypeErrorData::NonStrictFunctionDefinitionError(_) => 43,
      TypeErrorData::PropertyAccessViolation(_) => 44,
      TypeErrorData::CheckedFunctionIncorrectArgs(_) => 45,
      TypeErrorData::UnexpectedTypeInSubtyping(_) => 46,
      TypeErrorData::UnexpectedTypePackInSubtyping(_) => 47,
      TypeErrorData::ExplicitFunctionAnnotationRecommended(_) => 48,
      TypeErrorData::UserDefinedTypeFunctionError(_) => 49,
      TypeErrorData::BuiltInTypeFunctionError(_) => 50,
      TypeErrorData::ReservedIdentifier(_) => 51,
      TypeErrorData::UnexpectedArrayLikeTableItem(_) => 52,
      TypeErrorData::CannotCheckDynamicStringFormatCalls(_) => 53,
      TypeErrorData::GenericTypeCountMismatch(_) => 54,
      TypeErrorData::GenericTypePackCountMismatch(_) => 55,
      TypeErrorData::MultipleNonviableOverloads(_) => 56,
      TypeErrorData::RecursiveRestraintViolation(_) => 57,
      TypeErrorData::GenericBoundsMismatch(_) => 58,
      TypeErrorData::UnappliedTypeFunction(_) => 59,
      TypeErrorData::InstantiateGenericsOnNonFunction(_) => 60,
      TypeErrorData::TypeInstantiationCountMismatch(_) => 61,
      TypeErrorData::AmbiguousFunctionCall(_) => 62,
    }
  }
}

/// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
pub trait TypeErrorDataMember: Sized {
  fn get_if(v: &TypeErrorData) -> Option<&Self>;
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self>;
}

impl TypeErrorDataMember for TypeMismatch {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::TypeMismatch(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::TypeMismatch(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnknownSymbol {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnknownSymbol(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnknownSymbol(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnknownProperty {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnknownProperty(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnknownProperty(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for NotATable {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::NotATable(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::NotATable(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CannotExtendTable {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CannotExtendTable(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CannotExtendTable(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CannotCompareUnrelatedTypes {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CannotCompareUnrelatedTypes(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CannotCompareUnrelatedTypes(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for OnlyTablesCanHaveMethods {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::OnlyTablesCanHaveMethods(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::OnlyTablesCanHaveMethods(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for DuplicateTypeDefinition {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::DuplicateTypeDefinition(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::DuplicateTypeDefinition(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CountMismatch {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CountMismatch(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CountMismatch(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for FunctionDoesNotTakeSelf {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::FunctionDoesNotTakeSelf(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::FunctionDoesNotTakeSelf(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for FunctionRequiresSelf {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::FunctionRequiresSelf(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::FunctionRequiresSelf(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for OccursCheckFailed {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::OccursCheckFailed(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::OccursCheckFailed(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnknownRequire {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnknownRequire(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnknownRequire(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for IncorrectGenericParameterCount {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::IncorrectGenericParameterCount(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::IncorrectGenericParameterCount(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for SyntaxError {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::SyntaxError(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::SyntaxError(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CodeTooComplex {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CodeTooComplex(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CodeTooComplex(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnificationTooComplex {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnificationTooComplex(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnificationTooComplex(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnknownPropButFoundLikeProp {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnknownPropButFoundLikeProp(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnknownPropButFoundLikeProp(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for GenericError {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::GenericError(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::GenericError(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for InternalError {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::InternalError(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::InternalError(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for ConstraintSolvingIncompleteError {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::ConstraintSolvingIncompleteError(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::ConstraintSolvingIncompleteError(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CannotCallNonFunction {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CannotCallNonFunction(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CannotCallNonFunction(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for ExtraInformation {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::ExtraInformation(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::ExtraInformation(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for DeprecatedApiUsed {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::DeprecatedApiUsed(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::DeprecatedApiUsed(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for ModuleHasCyclicDependency {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::ModuleHasCyclicDependency(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::ModuleHasCyclicDependency(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for IllegalRequire {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::IllegalRequire(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::IllegalRequire(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for FunctionExitsWithoutReturning {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::FunctionExitsWithoutReturning(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::FunctionExitsWithoutReturning(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for DuplicateGenericParameter {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::DuplicateGenericParameter(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::DuplicateGenericParameter(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CannotAssignToNever {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CannotAssignToNever(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CannotAssignToNever(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CannotInferBinaryOperation {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CannotInferBinaryOperation(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CannotInferBinaryOperation(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for MissingProperties {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::MissingProperties(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::MissingProperties(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for SwappedGenericTypeParameter {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::SwappedGenericTypeParameter(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::SwappedGenericTypeParameter(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for OptionalValueAccess {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::OptionalValueAccess(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::OptionalValueAccess(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for MissingUnionProperty {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::MissingUnionProperty(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::MissingUnionProperty(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for TypesAreUnrelated {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::TypesAreUnrelated(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::TypesAreUnrelated(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for NormalizationTooComplex {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::NormalizationTooComplex(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::NormalizationTooComplex(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for TypePackMismatch {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::TypePackMismatch(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::TypePackMismatch(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for DynamicPropertyLookupOnExternTypesUnsafe {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UninhabitedTypeFunction {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UninhabitedTypeFunction(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UninhabitedTypeFunction(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UninhabitedTypePackFunction {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UninhabitedTypePackFunction(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UninhabitedTypePackFunction(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for WhereClauseNeeded {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::WhereClauseNeeded(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::WhereClauseNeeded(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for PackWhereClauseNeeded {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::PackWhereClauseNeeded(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::PackWhereClauseNeeded(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CheckedFunctionCallError {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CheckedFunctionCallError(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CheckedFunctionCallError(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for NonStrictFunctionDefinitionError {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::NonStrictFunctionDefinitionError(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::NonStrictFunctionDefinitionError(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for PropertyAccessViolation {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::PropertyAccessViolation(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::PropertyAccessViolation(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CheckedFunctionIncorrectArgs {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CheckedFunctionIncorrectArgs(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CheckedFunctionIncorrectArgs(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnexpectedTypeInSubtyping {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnexpectedTypeInSubtyping(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnexpectedTypeInSubtyping(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnexpectedTypePackInSubtyping {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnexpectedTypePackInSubtyping(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnexpectedTypePackInSubtyping(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for ExplicitFunctionAnnotationRecommended {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::ExplicitFunctionAnnotationRecommended(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::ExplicitFunctionAnnotationRecommended(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UserDefinedTypeFunctionError {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UserDefinedTypeFunctionError(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UserDefinedTypeFunctionError(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for BuiltInTypeFunctionError {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::BuiltInTypeFunctionError(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::BuiltInTypeFunctionError(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for ReservedIdentifier {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::ReservedIdentifier(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::ReservedIdentifier(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnexpectedArrayLikeTableItem {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnexpectedArrayLikeTableItem(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnexpectedArrayLikeTableItem(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for CannotCheckDynamicStringFormatCalls {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::CannotCheckDynamicStringFormatCalls(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::CannotCheckDynamicStringFormatCalls(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for GenericTypeCountMismatch {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::GenericTypeCountMismatch(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::GenericTypeCountMismatch(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for GenericTypePackCountMismatch {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::GenericTypePackCountMismatch(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::GenericTypePackCountMismatch(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for MultipleNonviableOverloads {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::MultipleNonviableOverloads(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::MultipleNonviableOverloads(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for RecursiveRestraintViolation {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::RecursiveRestraintViolation(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::RecursiveRestraintViolation(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for GenericBoundsMismatch {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::GenericBoundsMismatch(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::GenericBoundsMismatch(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for UnappliedTypeFunction {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::UnappliedTypeFunction(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::UnappliedTypeFunction(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for InstantiateGenericsOnNonFunction {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::InstantiateGenericsOnNonFunction(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::InstantiateGenericsOnNonFunction(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for TypeInstantiationCountMismatch {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::TypeInstantiationCountMismatch(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::TypeInstantiationCountMismatch(x) => Some(x),
      _ => None,
    }
  }
}

impl TypeErrorDataMember for AmbiguousFunctionCall {
  fn get_if(v: &TypeErrorData) -> Option<&Self> {
    match v {
      TypeErrorData::AmbiguousFunctionCall(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypeErrorData) -> Option<&mut Self> {
    match v {
      TypeErrorData::AmbiguousFunctionCall(x) => Some(x),
      _ => None,
    }
  }
}

// === Auto-generated From<ConcreteError> conversions (faithful: each error variant wraps exactly its struct) ===
impl From<TypeMismatch> for TypeErrorData {
  fn from(value: TypeMismatch) -> Self {
    TypeErrorData::TypeMismatch(value)
  }
}
impl From<UnknownSymbol> for TypeErrorData {
  fn from(value: UnknownSymbol) -> Self {
    TypeErrorData::UnknownSymbol(value)
  }
}
impl From<UnknownProperty> for TypeErrorData {
  fn from(value: UnknownProperty) -> Self {
    TypeErrorData::UnknownProperty(value)
  }
}
impl From<NotATable> for TypeErrorData {
  fn from(value: NotATable) -> Self {
    TypeErrorData::NotATable(value)
  }
}
impl From<CannotExtendTable> for TypeErrorData {
  fn from(value: CannotExtendTable) -> Self {
    TypeErrorData::CannotExtendTable(value)
  }
}
impl From<CannotCompareUnrelatedTypes> for TypeErrorData {
  fn from(value: CannotCompareUnrelatedTypes) -> Self {
    TypeErrorData::CannotCompareUnrelatedTypes(value)
  }
}
impl From<OnlyTablesCanHaveMethods> for TypeErrorData {
  fn from(value: OnlyTablesCanHaveMethods) -> Self {
    TypeErrorData::OnlyTablesCanHaveMethods(value)
  }
}
impl From<DuplicateTypeDefinition> for TypeErrorData {
  fn from(value: DuplicateTypeDefinition) -> Self {
    TypeErrorData::DuplicateTypeDefinition(value)
  }
}
impl From<CountMismatch> for TypeErrorData {
  fn from(value: CountMismatch) -> Self {
    TypeErrorData::CountMismatch(value)
  }
}
impl From<FunctionDoesNotTakeSelf> for TypeErrorData {
  fn from(value: FunctionDoesNotTakeSelf) -> Self {
    TypeErrorData::FunctionDoesNotTakeSelf(value)
  }
}
impl From<FunctionRequiresSelf> for TypeErrorData {
  fn from(value: FunctionRequiresSelf) -> Self {
    TypeErrorData::FunctionRequiresSelf(value)
  }
}
impl From<OccursCheckFailed> for TypeErrorData {
  fn from(value: OccursCheckFailed) -> Self {
    TypeErrorData::OccursCheckFailed(value)
  }
}
impl From<UnknownRequire> for TypeErrorData {
  fn from(value: UnknownRequire) -> Self {
    TypeErrorData::UnknownRequire(value)
  }
}
impl From<IncorrectGenericParameterCount> for TypeErrorData {
  fn from(value: IncorrectGenericParameterCount) -> Self {
    TypeErrorData::IncorrectGenericParameterCount(value)
  }
}
impl From<SyntaxError> for TypeErrorData {
  fn from(value: SyntaxError) -> Self {
    TypeErrorData::SyntaxError(value)
  }
}
impl From<CodeTooComplex> for TypeErrorData {
  fn from(value: CodeTooComplex) -> Self {
    TypeErrorData::CodeTooComplex(value)
  }
}
impl From<UnificationTooComplex> for TypeErrorData {
  fn from(value: UnificationTooComplex) -> Self {
    TypeErrorData::UnificationTooComplex(value)
  }
}
impl From<UnknownPropButFoundLikeProp> for TypeErrorData {
  fn from(value: UnknownPropButFoundLikeProp) -> Self {
    TypeErrorData::UnknownPropButFoundLikeProp(value)
  }
}
impl From<GenericError> for TypeErrorData {
  fn from(value: GenericError) -> Self {
    TypeErrorData::GenericError(value)
  }
}
impl From<InternalError> for TypeErrorData {
  fn from(value: InternalError) -> Self {
    TypeErrorData::InternalError(value)
  }
}
impl From<ConstraintSolvingIncompleteError> for TypeErrorData {
  fn from(value: ConstraintSolvingIncompleteError) -> Self {
    TypeErrorData::ConstraintSolvingIncompleteError(value)
  }
}
impl From<CannotCallNonFunction> for TypeErrorData {
  fn from(value: CannotCallNonFunction) -> Self {
    TypeErrorData::CannotCallNonFunction(value)
  }
}
impl From<ExtraInformation> for TypeErrorData {
  fn from(value: ExtraInformation) -> Self {
    TypeErrorData::ExtraInformation(value)
  }
}
impl From<DeprecatedApiUsed> for TypeErrorData {
  fn from(value: DeprecatedApiUsed) -> Self {
    TypeErrorData::DeprecatedApiUsed(value)
  }
}
impl From<ModuleHasCyclicDependency> for TypeErrorData {
  fn from(value: ModuleHasCyclicDependency) -> Self {
    TypeErrorData::ModuleHasCyclicDependency(value)
  }
}
impl From<IllegalRequire> for TypeErrorData {
  fn from(value: IllegalRequire) -> Self {
    TypeErrorData::IllegalRequire(value)
  }
}
impl From<FunctionExitsWithoutReturning> for TypeErrorData {
  fn from(value: FunctionExitsWithoutReturning) -> Self {
    TypeErrorData::FunctionExitsWithoutReturning(value)
  }
}
impl From<DuplicateGenericParameter> for TypeErrorData {
  fn from(value: DuplicateGenericParameter) -> Self {
    TypeErrorData::DuplicateGenericParameter(value)
  }
}
impl From<CannotAssignToNever> for TypeErrorData {
  fn from(value: CannotAssignToNever) -> Self {
    TypeErrorData::CannotAssignToNever(value)
  }
}
impl From<CannotInferBinaryOperation> for TypeErrorData {
  fn from(value: CannotInferBinaryOperation) -> Self {
    TypeErrorData::CannotInferBinaryOperation(value)
  }
}
impl From<MissingProperties> for TypeErrorData {
  fn from(value: MissingProperties) -> Self {
    TypeErrorData::MissingProperties(value)
  }
}
impl From<SwappedGenericTypeParameter> for TypeErrorData {
  fn from(value: SwappedGenericTypeParameter) -> Self {
    TypeErrorData::SwappedGenericTypeParameter(value)
  }
}
impl From<OptionalValueAccess> for TypeErrorData {
  fn from(value: OptionalValueAccess) -> Self {
    TypeErrorData::OptionalValueAccess(value)
  }
}
impl From<MissingUnionProperty> for TypeErrorData {
  fn from(value: MissingUnionProperty) -> Self {
    TypeErrorData::MissingUnionProperty(value)
  }
}
impl From<TypesAreUnrelated> for TypeErrorData {
  fn from(value: TypesAreUnrelated) -> Self {
    TypeErrorData::TypesAreUnrelated(value)
  }
}
impl From<NormalizationTooComplex> for TypeErrorData {
  fn from(value: NormalizationTooComplex) -> Self {
    TypeErrorData::NormalizationTooComplex(value)
  }
}
impl From<TypePackMismatch> for TypeErrorData {
  fn from(value: TypePackMismatch) -> Self {
    TypeErrorData::TypePackMismatch(value)
  }
}
impl From<DynamicPropertyLookupOnExternTypesUnsafe> for TypeErrorData {
  fn from(value: DynamicPropertyLookupOnExternTypesUnsafe) -> Self {
    TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(value)
  }
}
impl From<UninhabitedTypeFunction> for TypeErrorData {
  fn from(value: UninhabitedTypeFunction) -> Self {
    TypeErrorData::UninhabitedTypeFunction(value)
  }
}
impl From<UninhabitedTypePackFunction> for TypeErrorData {
  fn from(value: UninhabitedTypePackFunction) -> Self {
    TypeErrorData::UninhabitedTypePackFunction(value)
  }
}
impl From<WhereClauseNeeded> for TypeErrorData {
  fn from(value: WhereClauseNeeded) -> Self {
    TypeErrorData::WhereClauseNeeded(value)
  }
}
impl From<PackWhereClauseNeeded> for TypeErrorData {
  fn from(value: PackWhereClauseNeeded) -> Self {
    TypeErrorData::PackWhereClauseNeeded(value)
  }
}
impl From<CheckedFunctionCallError> for TypeErrorData {
  fn from(value: CheckedFunctionCallError) -> Self {
    TypeErrorData::CheckedFunctionCallError(value)
  }
}
impl From<NonStrictFunctionDefinitionError> for TypeErrorData {
  fn from(value: NonStrictFunctionDefinitionError) -> Self {
    TypeErrorData::NonStrictFunctionDefinitionError(value)
  }
}
impl From<PropertyAccessViolation> for TypeErrorData {
  fn from(value: PropertyAccessViolation) -> Self {
    TypeErrorData::PropertyAccessViolation(value)
  }
}
impl From<CheckedFunctionIncorrectArgs> for TypeErrorData {
  fn from(value: CheckedFunctionIncorrectArgs) -> Self {
    TypeErrorData::CheckedFunctionIncorrectArgs(value)
  }
}
impl From<UnexpectedTypeInSubtyping> for TypeErrorData {
  fn from(value: UnexpectedTypeInSubtyping) -> Self {
    TypeErrorData::UnexpectedTypeInSubtyping(value)
  }
}
impl From<UnexpectedTypePackInSubtyping> for TypeErrorData {
  fn from(value: UnexpectedTypePackInSubtyping) -> Self {
    TypeErrorData::UnexpectedTypePackInSubtyping(value)
  }
}
impl From<ExplicitFunctionAnnotationRecommended> for TypeErrorData {
  fn from(value: ExplicitFunctionAnnotationRecommended) -> Self {
    TypeErrorData::ExplicitFunctionAnnotationRecommended(value)
  }
}
impl From<UserDefinedTypeFunctionError> for TypeErrorData {
  fn from(value: UserDefinedTypeFunctionError) -> Self {
    TypeErrorData::UserDefinedTypeFunctionError(value)
  }
}
impl From<BuiltInTypeFunctionError> for TypeErrorData {
  fn from(value: BuiltInTypeFunctionError) -> Self {
    TypeErrorData::BuiltInTypeFunctionError(value)
  }
}
impl From<ReservedIdentifier> for TypeErrorData {
  fn from(value: ReservedIdentifier) -> Self {
    TypeErrorData::ReservedIdentifier(value)
  }
}
impl From<UnexpectedArrayLikeTableItem> for TypeErrorData {
  fn from(value: UnexpectedArrayLikeTableItem) -> Self {
    TypeErrorData::UnexpectedArrayLikeTableItem(value)
  }
}
impl From<CannotCheckDynamicStringFormatCalls> for TypeErrorData {
  fn from(value: CannotCheckDynamicStringFormatCalls) -> Self {
    TypeErrorData::CannotCheckDynamicStringFormatCalls(value)
  }
}
impl From<GenericTypeCountMismatch> for TypeErrorData {
  fn from(value: GenericTypeCountMismatch) -> Self {
    TypeErrorData::GenericTypeCountMismatch(value)
  }
}
impl From<GenericTypePackCountMismatch> for TypeErrorData {
  fn from(value: GenericTypePackCountMismatch) -> Self {
    TypeErrorData::GenericTypePackCountMismatch(value)
  }
}
impl From<MultipleNonviableOverloads> for TypeErrorData {
  fn from(value: MultipleNonviableOverloads) -> Self {
    TypeErrorData::MultipleNonviableOverloads(value)
  }
}
impl From<RecursiveRestraintViolation> for TypeErrorData {
  fn from(value: RecursiveRestraintViolation) -> Self {
    TypeErrorData::RecursiveRestraintViolation(value)
  }
}
impl From<GenericBoundsMismatch> for TypeErrorData {
  fn from(value: GenericBoundsMismatch) -> Self {
    TypeErrorData::GenericBoundsMismatch(value)
  }
}
impl From<UnappliedTypeFunction> for TypeErrorData {
  fn from(value: UnappliedTypeFunction) -> Self {
    TypeErrorData::UnappliedTypeFunction(value)
  }
}
impl From<InstantiateGenericsOnNonFunction> for TypeErrorData {
  fn from(value: InstantiateGenericsOnNonFunction) -> Self {
    TypeErrorData::InstantiateGenericsOnNonFunction(value)
  }
}
impl From<TypeInstantiationCountMismatch> for TypeErrorData {
  fn from(value: TypeInstantiationCountMismatch) -> Self {
    TypeErrorData::TypeInstantiationCountMismatch(value)
  }
}
impl From<AmbiguousFunctionCall> for TypeErrorData {
  fn from(value: AmbiguousFunctionCall) -> Self {
    TypeErrorData::AmbiguousFunctionCall(value)
  }
}

/// C++ idiom: error structs flow into TypeErrorData. Mirrors `.into()` but keeps the explicit `into_type_error_data()` call sites used by the translation.
pub trait IntoTypeErrorData {
  fn into_type_error_data(self) -> TypeErrorData;
}

impl<T> IntoTypeErrorData for T
where
  T: Into<TypeErrorData>,
{
  fn into_type_error_data(self) -> TypeErrorData {
    self.into()
  }
}
