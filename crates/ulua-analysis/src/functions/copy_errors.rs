use core::ptr::null;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::copy_error::copy_error,
  records::{builtin_types::BuiltinTypes, clone_state::CloneState, type_arena::TypeArena},
  type_aliases::{error_vec::ErrorVec, type_error_data::TypeErrorData},
};
pub fn copy_errors(
  errors: &mut ErrorVec,
  dest_arena: &mut TypeArena,
  builtin_types: &BuiltinTypes,
) {
  let mut clone_state = CloneState {
    builtin_types: builtin_types as *const BuiltinTypes as *mut BuiltinTypes,
    seen_types: DenseHashMap::new(null()),
    seen_type_packs: DenseHashMap::new(null()),
  };

  let types_arena = &dest_arena.types;
  let type_packs_arena = &dest_arena.type_packs;
  LUAU_ASSERT!(!types_arena.is_frozen());
  LUAU_ASSERT!(!type_packs_arena.is_frozen());

  for error in errors {
    match &mut error.data {
      TypeErrorData::TypeMismatch(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnknownSymbol(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnknownProperty(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::NotATable(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CannotExtendTable(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CannotCompareUnrelatedTypes(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::OnlyTablesCanHaveMethods(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::DuplicateTypeDefinition(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CountMismatch(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::FunctionDoesNotTakeSelf(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::FunctionRequiresSelf(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::OccursCheckFailed(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnknownRequire(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::IncorrectGenericParameterCount(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::SyntaxError(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CodeTooComplex(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnificationTooComplex(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnknownPropButFoundLikeProp(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::GenericError(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::InternalError(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::ConstraintSolvingIncompleteError(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::CannotCallNonFunction(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::ExtraInformation(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::DeprecatedApiUsed(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::ModuleHasCyclicDependency(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::IllegalRequire(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::FunctionExitsWithoutReturning(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::DuplicateGenericParameter(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CannotAssignToNever(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CannotInferBinaryOperation(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::MissingProperties(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::SwappedGenericTypeParameter(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::OptionalValueAccess(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::MissingUnionProperty(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::TypesAreUnrelated(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::NormalizationTooComplex(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::TypePackMismatch(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::UninhabitedTypeFunction(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UninhabitedTypePackFunction(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::WhereClauseNeeded(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::PackWhereClauseNeeded(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CheckedFunctionCallError(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::NonStrictFunctionDefinitionError(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::PropertyAccessViolation(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CheckedFunctionIncorrectArgs(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnexpectedTypeInSubtyping(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnexpectedTypePackInSubtyping(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::ExplicitFunctionAnnotationRecommended(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::UserDefinedTypeFunctionError(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::BuiltInTypeFunctionError(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::ReservedIdentifier(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnexpectedArrayLikeTableItem(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::CannotCheckDynamicStringFormatCalls(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::GenericTypeCountMismatch(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::GenericTypePackCountMismatch(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::MultipleNonviableOverloads(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::RecursiveRestraintViolation(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::GenericBoundsMismatch(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::UnappliedTypeFunction(e) => copy_error(e, dest_arena, &mut clone_state),
      TypeErrorData::InstantiateGenericsOnNonFunction(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::TypeInstantiationCountMismatch(e) => {
        copy_error(e, dest_arena, &mut clone_state)
      }
      TypeErrorData::AmbiguousFunctionCall(e) => copy_error(e, dest_arena, &mut clone_state),
    }
  }
}
