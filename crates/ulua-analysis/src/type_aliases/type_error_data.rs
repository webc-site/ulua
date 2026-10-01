//! Source: `Analysis/include/Luau/Error.h:605-670` (hand-ported)
use crate::{
  macros::variant_enum,
  records::{
    ambiguous_function_call::AmbiguousFunctionCall,
    built_in_type_function_error::BuiltInTypeFunctionError,
    cannot_assign_to_never::CannotAssignToNever, cannot_call_non_function::CannotCallNonFunction,
    cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls,
    cannot_compare_unrelated_types::CannotCompareUnrelatedTypes,
    cannot_extend_table::CannotExtendTable,
    cannot_infer_binary_operation::CannotInferBinaryOperation,
    checked_function_call_error::CheckedFunctionCallError,
    checked_function_incorrect_args::CheckedFunctionIncorrectArgs,
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
    type_annotation_required::TypeAnnotationRequired,
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
};

// 65 members -> custom enum; ORDER preserves C++ Variant positions.
variant_enum! {
  #[derive(Debug, Clone, PartialEq)]
  pub enum from TypeErrorData {
    TypeMismatch => TypeMismatch,
    UnknownSymbol => UnknownSymbol,
    UnknownProperty => UnknownProperty,
    NotATable => NotATable,
    CannotExtendTable => CannotExtendTable,
    CannotCompareUnrelatedTypes => CannotCompareUnrelatedTypes,
    OnlyTablesCanHaveMethods => OnlyTablesCanHaveMethods,
    DuplicateTypeDefinition => DuplicateTypeDefinition,
    CountMismatch => CountMismatch,
    FunctionDoesNotTakeSelf => FunctionDoesNotTakeSelf,
    FunctionRequiresSelf => FunctionRequiresSelf,
    OccursCheckFailed => OccursCheckFailed,
    UnknownRequire => UnknownRequire,
    IncorrectGenericParameterCount => IncorrectGenericParameterCount,
    SyntaxError => SyntaxError,
    CodeTooComplex => CodeTooComplex,
    UnificationTooComplex => UnificationTooComplex,
    UnknownPropButFoundLikeProp => UnknownPropButFoundLikeProp,
    GenericError => GenericError,
    InternalError => InternalError,
    ConstraintSolvingIncompleteError => ConstraintSolvingIncompleteError,
    CannotCallNonFunction => CannotCallNonFunction,
    ExtraInformation => ExtraInformation,
    DeprecatedApiUsed => DeprecatedApiUsed,
    ModuleHasCyclicDependency => ModuleHasCyclicDependency,
    IllegalRequire => IllegalRequire,
    FunctionExitsWithoutReturning => FunctionExitsWithoutReturning,
    DuplicateGenericParameter => DuplicateGenericParameter,
    CannotAssignToNever => CannotAssignToNever,
    CannotInferBinaryOperation => CannotInferBinaryOperation,
    MissingProperties => MissingProperties,
    SwappedGenericTypeParameter => SwappedGenericTypeParameter,
    OptionalValueAccess => OptionalValueAccess,
    MissingUnionProperty => MissingUnionProperty,
    TypesAreUnrelated => TypesAreUnrelated,
    NormalizationTooComplex => NormalizationTooComplex,
    TypePackMismatch => TypePackMismatch,
    DynamicPropertyLookupOnExternTypesUnsafe => DynamicPropertyLookupOnExternTypesUnsafe,
    UninhabitedTypeFunction => UninhabitedTypeFunction,
    UninhabitedTypePackFunction => UninhabitedTypePackFunction,
    WhereClauseNeeded => WhereClauseNeeded,
    PackWhereClauseNeeded => PackWhereClauseNeeded,
    CheckedFunctionCallError => CheckedFunctionCallError,
    NonStrictFunctionDefinitionError => NonStrictFunctionDefinitionError,
    PropertyAccessViolation => PropertyAccessViolation,
    CheckedFunctionIncorrectArgs => CheckedFunctionIncorrectArgs,
    UnexpectedTypeInSubtyping => UnexpectedTypeInSubtyping,
    UnexpectedTypePackInSubtyping => UnexpectedTypePackInSubtyping,
    ExplicitFunctionAnnotationRecommended => ExplicitFunctionAnnotationRecommended,
    UserDefinedTypeFunctionError => UserDefinedTypeFunctionError,
    BuiltInTypeFunctionError => BuiltInTypeFunctionError,
    ReservedIdentifier => ReservedIdentifier,
    UnexpectedArrayLikeTableItem => UnexpectedArrayLikeTableItem,
    CannotCheckDynamicStringFormatCalls => CannotCheckDynamicStringFormatCalls,
    GenericTypeCountMismatch => GenericTypeCountMismatch,
    GenericTypePackCountMismatch => GenericTypePackCountMismatch,
    MultipleNonviableOverloads => MultipleNonviableOverloads,
    RecursiveRestraintViolation => RecursiveRestraintViolation,
    GenericBoundsMismatch => GenericBoundsMismatch,
    UnappliedTypeFunction => UnappliedTypeFunction,
    InstantiateGenericsOnNonFunction => InstantiateGenericsOnNonFunction,
    TypeInstantiationCountMismatch => TypeInstantiationCountMismatch,
    AmbiguousFunctionCall => AmbiguousFunctionCall,
    UninitializedFieldAccess => UninitializedFieldAccess,
    TypeAnnotationRequired => TypeAnnotationRequired,
  }

  /// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
  ///
  /// 成员表顺序与枚举声明一致，便于对照 C++ Variant 位置；本枚举变体名与成员
  /// 结构体同名。`from` 面即 cpp `TypeErrorData(T)` 的隐式转换。
  pub trait TypeErrorDataMember;
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
