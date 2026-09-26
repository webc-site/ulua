use alloc::{format, string::String};
use core::fmt::Write;

use ulua_ast::functions::to_string_ast::to_str_binary as to_str;
use ulua_common::{fflag, fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{context_error::Context, op_kind::OpKind, reason::Reason},
  functions::{
    begin_type::begin_union_type,
    follow_type,
    get_definition_module_name::get_definition_module_name,
    get_table_type::get_table_type,
    get_type,
    sep_join::SepWriter,
    to_human_readable_index::to_human_readable_index,
    to_string_detailed_to_string::to_string_detailed,
    to_string_error::to_string_type_error_type_error_to_string_options,
    to_string_to_string::{
      to_string_type_id, to_string_type_id_to_string_options_mut, to_string_type_pack_id,
      to_string_type_pack_id_to_string_options_mut,
    },
    to_string_type_function_error::to_string_type_function_error,
    wrong_number_of_args_string::wrong_number_of_args_string,
  },
  records::{
    ambiguous_function_call::AmbiguousFunctionCall,
    built_in_type_function_error::BuiltInTypeFunctionError,
    cannot_assign_to_never::CannotAssignToNever,
    cannot_call_non_function::CannotCallNonFunction,
    cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls,
    cannot_compare_unrelated_types::CannotCompareUnrelatedTypes,
    cannot_extend_table::{CannotExtendTable, Context as CannotExtendTableContext},
    cannot_infer_binary_operation::CannotInferBinaryOperation,
    checked_function_call_error::CheckedFunctionCallError,
    checked_function_incorrect_args::CheckedFunctionIncorrectArgs,
    code_too_complex::CodeTooComplex,
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    count_mismatch::{CountMismatch, CountMismatchContext},
    deprecated_api_used::DeprecatedApiUsed,
    duplicate_generic_parameter::DuplicateGenericParameter,
    duplicate_type_definition::DuplicateTypeDefinition,
    dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe,
    error_converter::ErrorConverter,
    explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended,
    extern_type::ExternType,
    extra_information::ExtraInformation,
    function_does_not_take_self::FunctionDoesNotTakeSelf,
    function_exits_without_returning::FunctionExitsWithoutReturning,
    function_requires_self::FunctionRequiresSelf,
    function_type::FunctionType,
    generic_bounds_mismatch::GenericBoundsMismatch,
    generic_error::GenericError,
    generic_type_count_mismatch::GenericTypeCountMismatch,
    generic_type_pack_count_mismatch::GenericTypePackCountMismatch,
    illegal_require::IllegalRequire,
    incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction,
    internal_error::InternalError,
    missing_properties::{Context as MissingPropertiesContext, MissingProperties},
    missing_union_property::MissingUnionProperty,
    module_has_cyclic_dependency::ModuleHasCyclicDependency,
    multiple_nonviable_overloads::MultipleNonviableOverloads,
    never_type::NeverType,
    non_strict_function_definition_error::NonStrictFunctionDefinitionError,
    normalization_too_complex::NormalizationTooComplex,
    not_a_table::NotATable,
    occurs_check_failed::OccursCheckFailed,
    only_tables_can_have_methods::OnlyTablesCanHaveMethods,
    optional_value_access::OptionalValueAccess,
    pack_where_clause_needed::PackWhereClauseNeeded,
    primitive_type::PrimitiveType,
    property_access_violation::{self, PropertyAccessViolation},
    recursive_restraint_violation::RecursiveRestraintViolation,
    reserved_identifier::ReservedIdentifier,
    swapped_generic_type_parameter::SwappedGenericTypeParameter,
    syntax_error::SyntaxError,
    table_type::TableType,
    to_string_options::ToStringOptions,
    type_annotation_required::TypeAnnotationRequired,
    type_error_to_string_options::TypeErrorToStringOptions,
    type_function_instance_type::TypeFunctionInstanceType,
    type_instantiation_count_mismatch::TypeInstantiationCountMismatch,
    type_mismatch::TypeMismatch,
    type_pack_mismatch::TypePackMismatch,
    types_are_unrelated::TypesAreUnrelated,
    unapplied_type_function::UnappliedTypeFunction,
    unexpected_array_like_table_item::UnexpectedArrayLikeTableItem,
    unexpected_type_in_subtyping::UnexpectedTypeInSubtyping,
    unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping,
    unification_too_complex::UnificationTooComplex,
    uninhabited_type_function::UninhabitedTypeFunction,
    uninhabited_type_pack_function::UninhabitedTypePackFunction,
    uninitialized_field_access::UninitializedFieldAccess,
    union_type::UnionType,
    unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    unknown_property::UnknownProperty,
    unknown_require::UnknownRequire,
    unknown_symbol::{Context as UnknownSymbolContext, UnknownSymbol},
    user_defined_type_function_error::UserDefinedTypeFunctionError,
    where_clause_needed::WhereClauseNeeded,
  },
  type_aliases::{
    error_type::ErrorType, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl<'a> ErrorConverter<'a> {
  pub fn convert(&self, data: &TypeErrorData) -> String {
    match data {
      TypeErrorData::TypeMismatch(e) => self.operator_call_40(e),
      TypeErrorData::UnknownSymbol(e) => self.operator_call_45(e),
      TypeErrorData::UnknownProperty(e) => self.operator_call_43(e),
      TypeErrorData::NotATable(e) => self.operator_call_34(e),
      TypeErrorData::CannotExtendTable(e) => self.operator_call_15(e),
      TypeErrorData::CannotCompareUnrelatedTypes(e) => self.operator_call_14(e),
      TypeErrorData::OnlyTablesCanHaveMethods(e) => self.operator_call_36(e),
      TypeErrorData::DuplicateTypeDefinition(e) => self.operator_call_22(e),
      TypeErrorData::CountMismatch(e) => self.operator_call_19(e),
      TypeErrorData::FunctionDoesNotTakeSelf(e) => self.operator_call_24(e),
      TypeErrorData::FunctionRequiresSelf(e) => self.operator_call_26(e),
      TypeErrorData::OccursCheckFailed(e) => self.operator_call_35(e),
      TypeErrorData::UnknownRequire(e) => self.operator_call_44(e),
      TypeErrorData::IncorrectGenericParameterCount(e) => self.operator_call_29(e),
      TypeErrorData::SyntaxError(e) => self.operator_call_39(e),
      TypeErrorData::CodeTooComplex(e) => self.operator_call_17(e),
      TypeErrorData::UnificationTooComplex(e) => self.operator_call_41(e),
      TypeErrorData::UnknownPropButFoundLikeProp(e) => self.operator_call_42(e),
      TypeErrorData::GenericError(e) => self.operator_call_27(e),
      TypeErrorData::InternalError(e) => self.operator_call_30(e),
      TypeErrorData::ConstraintSolvingIncompleteError(e) => self.operator_call_18(e),
      TypeErrorData::CannotCallNonFunction(e) => self.operator_call_13(e),
      TypeErrorData::ExtraInformation(e) => self.operator_call_23(e),
      TypeErrorData::DeprecatedApiUsed(e) => self.operator_call_20(e),
      TypeErrorData::ModuleHasCyclicDependency(e) => self.operator_call_33(e),
      TypeErrorData::IllegalRequire(e) => self.operator_call_28(e),
      TypeErrorData::FunctionExitsWithoutReturning(e) => self.operator_call_25(e),
      TypeErrorData::DuplicateGenericParameter(e) => self.operator_call_21(e),
      TypeErrorData::CannotAssignToNever(e) => self.operator_call_3(e),
      TypeErrorData::CannotInferBinaryOperation(e) => self.operator_call_16(e),
      TypeErrorData::MissingProperties(e) => self.operator_call_31(e),
      TypeErrorData::SwappedGenericTypeParameter(e) => self.operator_call_38(e),
      TypeErrorData::OptionalValueAccess(e) => self.operator_call_37(e),
      TypeErrorData::MissingUnionProperty(e) => self.operator_call_32(e),
      TypeErrorData::TypesAreUnrelated(e) => self.operator_call_55(e),
      TypeErrorData::NormalizationTooComplex(e) => self.operator_call_48(e),
      TypeErrorData::TypePackMismatch(e) => self.operator_call_54(e),
      TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(e) => self.operator_call_7(e),
      TypeErrorData::UninhabitedTypeFunction(e) => self.operator_call_60(e),
      TypeErrorData::UninhabitedTypePackFunction(e) => self.operator_call_61(e),
      TypeErrorData::WhereClauseNeeded(e) => self.operator_call_63(e),
      TypeErrorData::PackWhereClauseNeeded(e) => self.operator_call_49(e),
      TypeErrorData::CheckedFunctionCallError(e) => self.operator_call_5(e),
      TypeErrorData::NonStrictFunctionDefinitionError(e) => self.operator_call_47(e),
      TypeErrorData::PropertyAccessViolation(e) => self.operator_call_50(e),
      TypeErrorData::CheckedFunctionIncorrectArgs(e) => self.operator_call_6(e),
      TypeErrorData::UnexpectedTypeInSubtyping(e) => self.operator_call_58(e),
      TypeErrorData::UnexpectedTypePackInSubtyping(e) => self.operator_call_59(e),
      TypeErrorData::ExplicitFunctionAnnotationRecommended(e) => self.operator_call_8(e),
      TypeErrorData::UserDefinedTypeFunctionError(e) => self.operator_call_62(e),
      TypeErrorData::BuiltInTypeFunctionError(e) => self.operator_call_2(e),
      TypeErrorData::ReservedIdentifier(e) => self.operator_call_52(e),
      TypeErrorData::UnexpectedArrayLikeTableItem(e) => self.operator_call_57(e),
      TypeErrorData::CannotCheckDynamicStringFormatCalls(e) => self.operator_call_4(e),
      TypeErrorData::GenericTypeCountMismatch(e) => self.operator_call_10(e),
      TypeErrorData::GenericTypePackCountMismatch(e) => self.operator_call_11(e),
      TypeErrorData::MultipleNonviableOverloads(e) => self.operator_call_46(e),
      TypeErrorData::RecursiveRestraintViolation(e) => self.operator_call_51(e),
      TypeErrorData::GenericBoundsMismatch(e) => self.operator_call_9(e),
      TypeErrorData::UnappliedTypeFunction(e) => self.operator_call_56(e),
      TypeErrorData::InstantiateGenericsOnNonFunction(e) => self.operator_call_12(e),
      TypeErrorData::TypeInstantiationCountMismatch(e) => self.operator_call_53(e),
      TypeErrorData::AmbiguousFunctionCall(e) => self.operator_call(e),
      TypeErrorData::UninitializedFieldAccess(e) => self.operator_call_64(e),
      TypeErrorData::TypeAnnotationRequired(e) => self.operator_call_65(e),
    }
  }

  pub fn operator_call_40(&self, tm: &TypeMismatch) -> String {
    let given_type_name = to_string_type_id(tm.given_type);
    let wanted_type_name = to_string_type_id(tm.wanted_type);

    let mut result = String::new();

    let quote = |s: &str| -> String { format!("'{}'", s) };

    let construct_error_message = |given_type: String,
                                   wanted_type: String,
                                   given_module: Option<String>,
                                   wanted_module: Option<String>|
     -> String {
      let given = if let Some(ref gm) = given_module {
        format!("{} from {}", quote(&given_type), quote(gm))
      } else {
        quote(&given_type)
      };

      let wanted = if let Some(ref wm) = wanted_module {
        format!("{} from {}", quote(&wanted_type), quote(wm))
      } else {
        quote(&wanted_type)
      };

      let luau_indent_type_mismatch_max_type_length =
        fint::LuauIndentTypeMismatchMaxTypeLength.get() as usize;

      let follow_wanted = follow_type::follow(tm.wanted_type);
      let wanted_never = get_type::get::<NeverType>(follow_wanted);
      if wanted_never.is_some() {
        if given_type.len() <= luau_indent_type_mismatch_max_type_length {
          return format!("Expected this to be unreachable, but got {}", given);
        } else {
          return format!("Expected this to be unreachable, but got\n\t{}", given);
        }
      }

      if tm.context == Context::InvariantContext {
        if given_type.len() <= luau_indent_type_mismatch_max_type_length
          || wanted_type.len() <= luau_indent_type_mismatch_max_type_length
        {
          return format!("Expected this to be exactly {}, but got {}", wanted, given);
        } else {
          return format!(
            "Expected this to be exactly\n\t{}\nbut got\n\t{}",
            wanted, given
          );
        }
      }

      if given_type.len() <= luau_indent_type_mismatch_max_type_length
        || wanted_type.len() <= luau_indent_type_mismatch_max_type_length
      {
        format!("Expected this to be {}, but got {}", wanted, given)
      } else {
        format!("Expected this to be\n\t{}\nbut got\n\t{}", wanted, given)
      }
    };

    if given_type_name == wanted_type_name {
      let given_definition_module = get_definition_module_name(tm.given_type);
      let wanted_definition_module = get_definition_module_name(tm.wanted_type);

      if let (Some(ref given_mod), Some(ref wanted_mod)) =
        (given_definition_module, wanted_definition_module)
      {
        if let Some(file_resolver) = self.file_resolver_ref() {
          let given_module_name = file_resolver.get_human_readable_module_name(given_mod);
          let wanted_module_name = file_resolver.get_human_readable_module_name(wanted_mod);

          result = construct_error_message(
            given_type_name.clone(),
            wanted_type_name.clone(),
            Some(given_module_name),
            Some(wanted_module_name),
          );
        } else {
          result = construct_error_message(
            given_type_name.clone(),
            wanted_type_name.clone(),
            Some(given_mod.to_string()),
            Some(wanted_mod.to_string()),
          );
        }
      }
    }

    if result.is_empty() {
      result = construct_error_message(
        given_type_name.clone(),
        wanted_type_name.clone(),
        None,
        None,
      );
    }

    if let Some(ref err) = tm.error {
      result.push_str("\ncaused by:\n  ");

      if !tm.reason.is_empty() {
        result.push_str(&tm.reason);
        result.push('\n');
      }

      let opts = TypeErrorToStringOptions {
        file_resolver: self.file_resolver,
      };
      result.push_str(&to_string_type_error_type_error_to_string_options(
        err, opts,
      ));
    } else if !tm.reason.is_empty() {
      result.push_str("; ");
      result.push_str(&tm.reason);
    }

    result
  }

  pub fn operator_call_28(&self, e: &IllegalRequire) -> String {
    let mut result = String::from("Cannot require module ");
    result.push_str(e.module_name());
    result.push_str(": ");
    result.push_str(e.reason());
    result
  }

  pub fn operator_call_31(&self, e: &MissingProperties) -> String {
    let sub_type_str = to_string_type_id(e.sub_type());
    let super_type_str = to_string_type_id(e.super_type());

    let mut s = String::from("Table type '");
    s.push_str(&sub_type_str);
    s.push_str("' not compatible with type '");
    s.push_str(&super_type_str);
    s.push_str("' because the former");

    match e.context() {
      MissingPropertiesContext::Missing => {
        s.push_str(" is missing field");
      }
      MissingPropertiesContext::Extra => {
        s.push_str(" has extra field");
      }
    }

    if e.properties().len() > 1 {
      s.push('s');
    }

    s.push(' ');

    let properties = e.properties();
    let last = properties.len().saturating_sub(1);
    for (i, prop) in properties.iter().enumerate() {
      if i > 0 {
        s.push_str(", ");
      }

      if i > 0 && i == last {
        s.push_str("and ");
      }

      s.push('\'');
      s.push_str(prop);
      s.push('\'');
    }

    s
  }

  pub fn operator_call_21(&self, e: &DuplicateGenericParameter) -> String {
    let mut result = String::from("Duplicate type parameter '");
    result.push_str(e.parameter_name());
    result.push('\'');
    result
  }

  pub fn operator_call_16(&self, e: &CannotInferBinaryOperation) -> String {
    let mut result = String::from("Unknown type used in ");
    result.push_str(to_str(e.op()));

    match e.kind() {
      OpKind::Comparison => {
        result.push_str(" comparison");
      }
      OpKind::Operation => {
        result.push_str(" operation");
      }
    }

    if let Some(suggested) = e.suggested_to_annotate() {
      result.push_str("; consider adding a type annotation to '");
      result.push_str(suggested);
      result.push('\'');
    }

    result
  }

  pub fn operator_call_38(&self, e: &SwappedGenericTypeParameter) -> String {
    match e.kind {
      SwappedGenericTypeParameter::TYPE => {
        let mut result = String::from("Variadic type parameter '");
        result.push_str(&e.name);
        result.push_str("...' is used as a regular generic type; consider changing '");
        result.push_str(&e.name);
        result.push_str("...' to '");
        result.push_str(&e.name);
        result.push_str("' in the generic argument list");
        result
      }
      SwappedGenericTypeParameter::PACK => {
        let mut result = String::from("Generic type '");
        result.push_str(&e.name);
        result.push_str("' is used as a variadic type parameter; consider changing '");
        result.push_str(&e.name);
        result.push_str("' to '");
        result.push_str(&e.name);
        result.push_str("...' in the generic argument list");
        result
      }
    }
  }

  pub fn operator_call_37(&self, e: &OptionalValueAccess) -> String {
    let ty = to_string_type_id(e.optional);
    String::from("Value of type '") + &ty + "' could be nil"
  }

  pub fn operator_call_32(&self, e: &MissingUnionProperty) -> String {
    let mut ss = String::from("Key '");
    ss.push_str(e.key());
    ss.push_str("' is missing from ");

    {
      let mut writer = SepWriter::new(&mut ss, ", ");
      for ty in e.missing() {
        writer.push(&format!("'{}'", to_string_type_id(*ty)));
      }
    }

    ss.push_str(" in the type '");
    ss.push_str(&to_string_type_id(e.r#type()));
    ss.push('\'');

    ss
  }

  pub fn operator_call_55(&self, e: &TypesAreUnrelated) -> String {
    let opts = ToStringOptions::default();
    let left_str = to_string_type_id_to_string_options_mut(e.left, opts);
    let right_str = to_string_type_id(e.right);
    format!(
      "Cannot cast '{}' into '{}' because the types are unrelated",
      left_str, right_str
    )
  }

  pub fn operator_call_48(&self, _error: &NormalizationTooComplex) -> String {
    String::from("Code is too complex to typecheck! Consider simplifying the code around this area")
  }

  pub fn operator_call_54(&self, e: &TypePackMismatch) -> String {
    let wanted_str = to_string_type_pack_id(e.wanted_tp);
    let given_str = to_string_type_pack_id(e.given_tp);
    let mut ss =
      String::from("Expected this to be '") + &wanted_str + "', but got '" + &given_str + "'";

    if !e.reason.is_empty() {
      ss += "; ";
      ss += &e.reason;
    }

    ss
  }

  pub fn operator_call_7(&self, e: &DynamicPropertyLookupOnExternTypesUnsafe) -> String {
    "Attempting a dynamic property access on type '".to_owned()
      + &to_string_type_id(e.ty)
      + "' is unsafe and may cause exceptions at runtime"
  }

  pub fn operator_call_60(&self, e: &UninhabitedTypeFunction) -> String {
    let Some(tfit_ref) = get_type::get::<TypeFunctionInstanceType>(e.ty) else {
      LUAU_ASSERT!(false);
      return format!(
        "Internal error: Unexpected type {} flagged as an uninhabited type function.",
        to_string_type_id(e.ty)
      );
    };

    // SAFETY: function 指向 TypeFunctionInstance 内部的存活 TypedFunction。
    let function_name = unsafe { &(*tfit_ref.function.as_ptr()).name };

    // "types 用逗号连接；pack types 再以 ", " 前缀追加"，与 C++ 输出逐字节一致
    let append_argument_list =
      |result: &mut String, type_arguments: &[TypeId], pack_arguments: &[TypePackId]| {
        {
          let mut writer = SepWriter::new(result, ", ");
          for arg in type_arguments {
            writer.push(&to_string_type_id(*arg).to_string());
          }
        }
        // pack 参数逐个以 ", " 前缀追加（cpp 同形，不并入首元素旗标）。
        for pack_arg in pack_arguments {
          let _ = write!(result, ", {}", to_string_type_pack_id(*pack_arg));
        }
      };

    // unary operators
    if let Some(unary_string) = find_unary_op(function_name) {
      let mut result = format!("Operator '{unary_string}' could not be applied to ");

      if tfit_ref.type_arguments.len() == 1 && tfit_ref.pack_arguments.is_empty() {
        let _ = write!(
          result,
          "operand of type {}",
          to_string_type_id(tfit_ref.type_arguments[0])
        );

        if function_name != "not" {
          let _ = write!(
            result,
            "; there is no corresponding overload for __{function_name}"
          );
        }
      } else {
        result.push_str("operands of types ");
        append_argument_list(
          &mut result,
          &tfit_ref.type_arguments,
          &tfit_ref.pack_arguments,
        );
      }

      return result;
    }

    // binary operators
    if let Some(binary_string) = find_binary_op(function_name) {
      let mut result =
        format!("Operator '{binary_string}' could not be applied to operands of types ");

      if tfit_ref.type_arguments.len() == 2 && tfit_ref.pack_arguments.is_empty() {
        let _ = write!(
          result,
          "{} and {}",
          to_string_type_id(tfit_ref.type_arguments[0]),
          to_string_type_id(tfit_ref.type_arguments[1])
        );
      } else {
        append_argument_list(
          &mut result,
          &tfit_ref.type_arguments,
          &tfit_ref.pack_arguments,
        );
      }

      let _ = write!(
        result,
        "; there is no corresponding overload for __{function_name}"
      );

      return result;
    }

    // miscellaneous
    if function_name == "keyof" || function_name == "rawkeyof" {
      if tfit_ref.type_arguments.len() == 1 && tfit_ref.pack_arguments.is_empty() {
        return format!(
          "Type '{}' does not have keys, so '{}' is invalid",
          to_string_type_id(tfit_ref.type_arguments[0]),
          to_string_type_id(e.ty)
        );
      } else {
        return format!(
          "Type function instance {} is ill-formed, and thus invalid",
          to_string_type_id(e.ty)
        );
      }
    }

    if function_name == "index" || function_name == "rawget" {
      if tfit_ref.type_arguments.len() != 2 {
        return format!(
          "Type function instance {} is ill-formed, and thus invalid",
          to_string_type_id(e.ty)
        );
      }

      let second_arg = tfit_ref.type_arguments[1];
      if get_type::get::<ErrorType>(second_arg).is_some() {
        return format!(
          "Second argument to {}<{}, _> is not a valid index type",
          function_name,
          to_string_type_id(tfit_ref.type_arguments[0])
        );
      } else {
        return format!(
          "Property '{}' does not exist on type '{}'",
          to_string_type_id(tfit_ref.type_arguments[1]),
          to_string_type_id(tfit_ref.type_arguments[0])
        );
      }
    }

    if is_unreachable_type_function(function_name) {
      return format!(
        "Type function instance {} is uninhabited\nThis is likely to be a bug, please report it at https://github.com/luau-lang/luau/issues",
        to_string_type_id(e.ty)
      );
    }

    // Everything should be specialized above to report a more descriptive error that hopefully does not mention "type functions" explicitly.
    // If we produce this message, it's an indication that we've missed a specialization and it should be fixed!
    format!(
      "Type function instance {} is uninhabited",
      to_string_type_id(e.ty)
    )
  }

  pub fn operator_call_8(&self, e: &ExplicitFunctionAnnotationRecommended) -> String {
    let recommended_return = to_string_type_id(e.recommended_return());
    let mut buf = String::new();
    {
      let mut writer = SepWriter::new(&mut buf, ", ");
      for (arg, type_id) in e.recommended_args() {
        writer.push(&format!("{}: {}", arg, to_string_type_id(*type_id)));
      }
    }
    let arg_annotations = buf;

    if arg_annotations.is_empty() {
      String::from("Consider annotating the return with ") + &recommended_return
    } else {
      String::from("Consider placing the following annotations on the arguments: ")
        + &arg_annotations
        + " or instead annotating the return as "
        + &recommended_return
    }
  }

  pub fn operator_call_61(&self, e: &UninhabitedTypePackFunction) -> String {
    format!(
      "Type pack function instance {} is uninhabited",
      to_string_type_pack_id(e.tp)
    )
  }

  pub fn operator_call_63(&self, e: &WhereClauseNeeded) -> String {
    let ty = format!("{:?}", e.ty);
    String::from("Type function instance ")
      + &ty
      + " depends on generic function parameters but does not appear in the function signature; this construct cannot be type-checked at this time"
  }

  pub fn operator_call_49(&self, e: &PackWhereClauseNeeded) -> String {
    let tp = format!("{:?}", e.tp);
    String::from("Type pack function instance ")
      + &tp
      + " depends on generic function parameters but does not appear in the function signature; this construct cannot be type-checked at this time"
  }

  pub fn operator_call_5(&self, e: &CheckedFunctionCallError) -> String {
    let mut result = String::from("the function '");
    result.push_str(e.checked_function_name());
    result.push_str("' expects to get a ");
    result.push_str(&to_string_type_id(e.expected()));
    result.push_str(" as its ");
    result.push_str(&to_human_readable_index(e.argument_index()));
    result.push_str(" argument, but is being given a ");
    result.push_str(&to_string_type_id(e.passed()));
    result
  }

  pub fn operator_call_47(&self, e: &NonStrictFunctionDefinitionError) -> String {
    let mut result = String::new();
    if !e.function_name().is_empty() {
      result.push_str("in the function '");
      result.push_str(e.function_name());
      result.push_str("', '");
    }
    result.push_str("the argument '");
    result.push_str(e.argument());
    result.push_str("' is used in a way that will error at runtime");
    result
  }

  pub fn operator_call_50(&self, e: &PropertyAccessViolation) -> String {
    let mut chars = e.key().chars();
    let is_identifier_key = chars
      .next()
      .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
      && chars.all(|c| c.is_ascii_alphanumeric() || c == '_');

    let string_key = if is_identifier_key {
      e.key().to_string()
    } else {
      format!("\"{}\"", e.key())
    };

    if fflag::LuauTweakAccessViolationReporting.get() {
      let kind = if get_table_type(e.table()).is_some() {
        "table"
      } else {
        "type"
      };
      format!(
        "Property {string_key} of {kind} '{}' is {}-only",
        to_string_type_id(e.table()),
        match e.context() {
          property_access_violation::Context::CannotRead => "write",
          property_access_violation::Context::CannotWrite => "read",
        }
      )
    } else {
      let kind = match e.context() {
        property_access_violation::Context::CannotRead => "write",
        property_access_violation::Context::CannotWrite => "read",
      };
      format!(
        "Property {string_key} of table '{}' is {kind}-only",
        to_string_type_id(e.table())
      )
    }
  }

  pub fn operator_call_6(&self, e: &CheckedFunctionIncorrectArgs) -> String {
    format!(
      "the function '{}' will error at runtime if it is not called with {} arguments, \
       but we are calling it here with {} arguments",
      e.function_name(),
      e.expected(),
      e.actual()
    )
  }

  pub fn operator_call_58(&self, e: &UnexpectedTypeInSubtyping) -> String {
    let ty = to_string_type_id(e.ty);
    String::from("Encountered an unexpected type in subtyping: ") + &ty
  }

  pub fn operator_call_59(&self, e: &UnexpectedTypePackInSubtyping) -> String {
    let tp_str = to_string_type_pack_id(e.tp);
    String::from("Encountered an unexpected type pack in subtyping: ") + &tp_str
  }

  pub fn operator_call_62(&self, e: &UserDefinedTypeFunctionError) -> String {
    String::from(e.message())
  }

  pub fn operator_call_2(&self, e: &BuiltInTypeFunctionError) -> String {
    to_string_type_function_error(&e.error)
  }

  pub fn operator_call_52(&self, e: &ReservedIdentifier) -> String {
    let mut result = String::from(e.name());
    result.push_str(" cannot be used as an identifier for a type function or alias");
    result
  }

  pub fn operator_call_3(&self, e: &CannotAssignToNever) -> String {
    let opts = ToStringOptions::default();
    let rhs_type_str = to_string_type_id_to_string_options_mut(e.rhs_type(), opts);
    let mut result =
      String::from("Cannot assign a value of type ") + &rhs_type_str + " to a field of type never";

    if e.reason() == Reason::PropertyNarrowed && !e.cause().is_empty() {
      result.push_str("\ncaused by the property being given the following incompatible types:\n");
      for ty in e.cause() {
        let opts = ToStringOptions::default();
        let ty_str = to_string_type_id_to_string_options_mut(*ty, opts);
        result.push_str("    ");
        result.push_str(&ty_str);
        result.push('\n');
      }
      result.push_str("There are no values that could safely satisfy all of these types at once.");
    }

    result
  }

  pub fn operator_call_45(&self, e: &UnknownSymbol) -> String {
    match e.context() {
      UnknownSymbolContext::Binding => {
        let mut result = String::from("Unknown global '");
        result.push_str(e.name());
        result.push_str("'; consider assigning to it first");
        result
      }
      UnknownSymbolContext::Type => {
        let mut result = String::from("Unknown type '");
        result.push_str(e.name());
        result.push('\'');
        result
      }
    }

    // Keep same structure as the C++ version: this branch is unreachable.
    // If a new context variant is added, compilation will fail due to non-exhaustive match.
  }

  pub fn operator_call_57(&self, _error: &UnexpectedArrayLikeTableItem) -> String {
    String::from(
      "Unexpected array-like table item: the indexer key type of this table is not `number`.",
    )
  }

  pub fn operator_call_4(&self, _e: &CannotCheckDynamicStringFormatCalls) -> String {
    String::from(
      "We cannot statically check the type of `string.format` when called with a format string that is not statically known.\nIf you'd like to use an unchecked `string.format` call, you can cast the format string to `any` using `:: any`.",
    )
  }

  pub fn operator_call_10(&self, e: &GenericTypeCountMismatch) -> String {
    format!(
      "Different number of generic type parameters: subtype had {}, supertype had {}.",
      e.sub_ty_generic_count, e.super_ty_generic_count
    )
  }

  pub fn operator_call_11(&self, e: &GenericTypePackCountMismatch) -> String {
    format!(
      "Different number of generic type pack parameters: subtype had {}, supertype had {}.",
      e.sub_ty_generic_pack_count, e.super_ty_generic_pack_count
    )
  }

  pub fn operator_call_46(&self, e: &MultipleNonviableOverloads) -> String {
    format!(
      "None of the overloads for function that accept {} arguments are compatible.",
      e.attempted_arg_count
    )
  }

  pub fn operator_call_51(&self, _e: &RecursiveRestraintViolation) -> String {
    String::from("Recursive type being used with different parameters.")
  }

  pub fn operator_call_9(&self, e: &GenericBoundsMismatch) -> String {
    let mut lower_bounds = String::new();
    {
      let mut writer = SepWriter::new(&mut lower_bounds, " | ");
      for bound in &e.lower_bounds {
        writer.push(&to_string_type_id(*bound).to_string());
      }
    }
    let mut upper_bounds = String::new();
    {
      let mut writer = SepWriter::new(&mut upper_bounds, " & ");
      for bound in &e.upper_bounds {
        writer.push(&to_string_type_id(*bound).to_string());
      }
    }

    format!(
      "No valid instantiation could be inferred for generic type parameter {}. It was expected to be at least:\n\t{}\nand at most:\n\t{}\nbut these types are not compatible with one another.",
      e.generic_name, lower_bounds, upper_bounds
    )
  }

  pub fn operator_call_12(&self, e: &InstantiateGenericsOnNonFunction) -> String {
    match e.interesting_edge_case {
      InstantiateGenericsOnNonFunction::NONE => {
        String::from("Cannot instantiate type parameters on something without type parameters.")
      }
      InstantiateGenericsOnNonFunction::METATABLE_CALL => {
        // `__call` is complicated because `f<<T>>()` is interpreted as `f<<T>>` as its own expression that is then called.
        // This is so that you can write code like `local f2 = f<<number>>`, and then call `f2()`.
        // With metatables, it's not so obvious what this would result in.
        String::from(
          "Luau does not currently support explicitly instantiating a table with a `__call` metamethod. You may be able to work around this by creating a function that calls the table, and using that instead.",
        )
      }
      InstantiateGenericsOnNonFunction::INTERSECTION => String::from(
        "Luau does not currently support explicitly instantiating an overloaded function type.",
      ),
    }
  }

  pub fn operator_call_53(&self, e: &TypeInstantiationCountMismatch) -> String {
    LUAU_ASSERT!(
      e.provided_types() > e.maximum_types() || e.provided_type_packs() > e.maximum_type_packs()
    );

    // 复数后缀：数量为 1 时无 s，与 C++ 输出逐字节一致
    let plural = |n: usize| if n == 1 { "" } else { "s" };

    let mut result = String::from("Too many type parameters passed to ");

    if let Some(function_name) = e.function_name() {
      let _ = write!(result, "'{function_name}', which is typed as ");
    } else {
      result.push_str("function typed as ");
    }

    let _ = write!(
      result,
      "{}. Expected ",
      to_string_type_id(e.function_type())
    );

    if e.provided_types() > e.maximum_types() {
      let _ = write!(
        result,
        "at most {} type parameter{}, but {} provided",
        e.maximum_types(),
        plural(e.maximum_types()),
        e.provided_types()
      );

      if e.provided_type_packs() > e.maximum_type_packs() {
        result.push_str(". Also expected ");
      }
    }

    if e.provided_type_packs() > e.maximum_type_packs() {
      let _ = write!(
        result,
        "at most {} type pack{}, but {} provided",
        e.maximum_type_packs(),
        plural(e.maximum_type_packs()),
        e.provided_type_packs()
      );
    }

    result.push('.');
    result
  }

  pub fn operator_call_56(&self, _e: &UnappliedTypeFunction) -> String {
    String::from("Type functions always require `<>` when referenced.")
  }

  pub fn operator_call(&self, afc: &AmbiguousFunctionCall) -> String {
    let function_str = to_string_type_id(afc.function);
    let arguments_str = to_string_type_pack_id(afc.arguments);
    format!(
      "Calling function {} with argument pack {} is ambiguous.",
      function_str, arguments_str
    )
  }

  /// C++ `ErrorConverter::operator()(const UninitializedFieldAccess&)`（`Error.cpp:1025-1033`）。
  pub fn operator_call_64(&self, e: &UninitializedFieldAccess) -> String {
    match &e.field_name {
      Some(field) => {
        String::from("Access to field '") + field + "' of self before it has been initialized"
      }
      None => String::from("Access to 'self' before all of its fields have been initialized"),
    }
  }

  /// C++ `ErrorConverter::operator()(const TypeAnnotationRequired&)`（`Error.cpp:1035-1044`）。
  pub fn operator_call_65(&self, e: &TypeAnnotationRequired) -> String {
    let mut opts = ToStringOptions {
      function_type_arguments: true,
      ignore_synthetic_name: true,
      ..ToStringOptions::default()
    };
    let tos = to_string_detailed(e.inferred_ty, &mut opts);
    if !tos.invalid && !tos.truncated && !tos.error {
      String::from("Type annotation required here.  Consider ") + &tos.name
    } else {
      String::from("Type annotation required here.  Unable to infer the type of this function.")
    }
  }

  pub fn operator_call_43(&self, e: &UnknownProperty) -> String {
    let t = follow_type::follow(e.table);
    if get_type::get::<TableType>(t).is_none() {
      if get_type::get::<ExternType>(t).is_none() {
        format!(
          "Type '{}' does not have key '{}'",
          to_string_type_id(e.table),
          e.key
        )
      } else {
        format!(
          "Key '{}' not found in external type '{}'",
          e.key,
          to_string_type_id(t)
        )
      }
    } else {
      format!(
        "Key '{}' not found in table '{}'",
        e.key,
        to_string_type_id(t)
      )
    }
  }

  pub fn operator_call_34(&self, e: &NotATable) -> String {
    let ty = to_string_type_id(e.ty);
    String::from("Expected type table, got '") + &ty + "' instead"
  }

  pub fn operator_call_15(&self, e: &CannotExtendTable) -> String {
    let table = to_string_type_id(e.table_type());
    match e.context() {
      CannotExtendTableContext::Property => {
        format!("Cannot add property '{}' to table '{}'", e.prop(), table)
      }
      CannotExtendTableContext::Metatable => format!("Cannot add metatable to table '{table}'"),
      CannotExtendTableContext::Indexer => format!("Cannot add indexer to table '{table}'"),
    }
  }

  pub fn operator_call_14(&self, e: &CannotCompareUnrelatedTypes) -> String {
    let left_str = to_string_type_id(e.left);
    let right_str = to_string_type_id(e.right);
    let op_str = to_str(e.op);
    format!(
      "Cannot compare unrelated types '{}' and '{}' with '{}'",
      left_str, right_str, op_str
    )
  }

  pub fn operator_call_36(&self, e: &OnlyTablesCanHaveMethods) -> String {
    format!("Cannot add method to non-table type '{:?}'", e.table_type)
  }

  pub fn operator_call_22(&self, e: &DuplicateTypeDefinition) -> String {
    match e.previous_location() {
      Some(previous_location) => format!(
        "Redefinition of type '{}', previously defined at line {}",
        e.name(),
        previous_location.begin.line + 1
      ),
      None => format!("Redefinition of type '{}'", e.name()),
    }
  }

  pub fn operator_call_19(&self, e: &CountMismatch) -> String {
    let expected_s = if e.expected == 1 { "" } else { "s" };
    let actual_verb = if e.actual == 1 { "is" } else { "are" };

    match e.context {
      CountMismatchContext::Return => {
        alloc::format!(
          "Expected to return {} value{}, but {} {} returned here",
          e.expected,
          expected_s,
          e.actual,
          actual_verb
        )
      }
      CountMismatchContext::FunctionResult => {
        // It is alright if right hand side produces more values than the
        // left hand side accepts. In this context consider only the opposite case.
        alloc::format!(
          "Function only returns {} value{}, but {} {} required here",
          e.expected,
          expected_s,
          e.actual,
          actual_verb
        )
      }
      CountMismatchContext::ExprListResult => {
        alloc::format!(
          "Expression list has {} value{}, but {} {} required here",
          e.expected,
          expected_s,
          e.actual,
          actual_verb
        )
      }
      CountMismatchContext::Arg => {
        // DELIBERATE DEVIATION：`.setgenerics` 省略函数名分支来自更新上游
        // （本地 oracle Error.cpp:261-269 仅判 `!e.function.empty()`，无
        // setgenerics 特例）——对名为 `x.setgenerics` 的函数，Rust 省略
        // 引号函数名而 cpp 输出 `'x.setgenerics'`，仅错误文案差异。
        let omit_function_name = e.function.ends_with(".setgenerics");

        if !e.function.is_empty() && !omit_function_name {
          alloc::format!(
            "Argument count mismatch. Function '{}' {}",
            e.function,
            wrong_number_of_args_string(e.expected, e.maximum, e.actual, None, e.is_variadic)
          )
        } else {
          alloc::format!(
            "Argument count mismatch. Function {}",
            wrong_number_of_args_string(e.expected, e.maximum, e.actual, None, e.is_variadic)
          )
        }
      }
    }
  }

  pub fn operator_call_24(&self, _: &FunctionDoesNotTakeSelf) -> String {
    String::from("This function does not take self. Did you mean to use a dot instead of a colon?")
  }

  pub fn operator_call_26(&self, _e: &FunctionRequiresSelf) -> String {
    String::from(
      "This function must be called with self. Did you mean to use a colon instead of a dot?",
    )
  }

  pub fn operator_call_35(&self, _: &OccursCheckFailed) -> String {
    String::from("Type contains a self-recursive construct that cannot be resolved")
  }

  pub fn operator_call_44(&self, e: &UnknownRequire) -> String {
    if e.module_path().is_empty() {
      String::from("Unknown require: unsupported path")
    } else {
      let mut result = String::from("Unknown require: ");
      result.push_str(e.module_path());
      result
    }
  }

  pub fn operator_call_29(&self, e: &IncorrectGenericParameterCount) -> String {
    let mut name = e.name.clone();
    let opts = ToStringOptions::default();

    if !e.type_fun.type_params.is_empty() || !e.type_fun.type_pack_params.is_empty() {
      name.push('<');
      // 两组参数共用同一份首元素旗标（cpp 同一 `first` 变量跨两个循环），
      // 由单个写入器跨循环持有以保持分隔语义。
      {
        let mut writer = SepWriter::new(&mut name, ", ");
        for param in &e.type_fun.type_params {
          writer.push(&to_string_type_id_to_string_options_mut(
            param.ty,
            opts.clone(),
          ));
        }
        for param in &e.type_fun.type_pack_params {
          writer.push(&to_string_type_pack_id_to_string_options_mut(
            param.tp,
            opts.clone(),
          ));
        }
      }
      name.push('>');
    }

    if e.type_fun.type_params.len() != e.actual_parameters {
      let is_variadic = !e.type_fun.type_pack_params.is_empty();
      return format!(
        "Generic type '{}' {}",
        name,
        wrong_number_of_args_string(
          e.type_fun.type_params.len(),
          None,
          e.actual_parameters,
          Some("type"),
          is_variadic
        )
      );
    }

    format!(
      "Generic type '{}' {}",
      name,
      wrong_number_of_args_string(
        e.type_fun.type_pack_params.len(),
        None,
        e.actual_pack_parameters,
        Some("type pack"),
        false,
      )
    )
  }

  pub fn operator_call_39(&self, e: &SyntaxError) -> String {
    String::from(e.message())
  }

  pub fn operator_call_17(&self, _error: &CodeTooComplex) -> String {
    String::from("Code is too complex to typecheck! Consider simplifying the code around this area")
  }

  pub fn operator_call_41(&self, _: &UnificationTooComplex) -> String {
    String::from(
      "Internal error: Code is too complex to typecheck! Consider adding type annotations around this area",
    )
  }

  pub fn operator_call_42(&self, e: &UnknownPropButFoundLikeProp) -> String {
    let mut candidates_suggestion = String::from("Did you mean ");
    if e.candidates().len() != 1 {
      candidates_suggestion.push_str("one of ");
    }

    {
      let mut writer = SepWriter::new(&mut candidates_suggestion, ", ");
      for name in e.candidates() {
        writer.push(&format!("'{}'", name));
      }
    }

    let mut s = String::from("Key '");
    s.push_str(e.key());
    s.push_str("' not found in ");

    let t = follow_type::follow(e.table());
    if get_type::get::<ExternType>(t).is_none() {
      s.push_str("table");
    } else {
      s.push_str("external type");
    }

    s.push_str(" '");
    s.push_str(&to_string_type_id(e.table()));
    s.push_str("'.  ");
    s.push_str(&candidates_suggestion);
    s.push('?');

    s
  }

  pub fn operator_call_27(&self, e: &GenericError) -> String {
    String::from(e.message())
  }

  pub fn operator_call_30(&self, e: &InternalError) -> String {
    String::from(e.message())
  }

  pub fn operator_call_18(&self, _e: &ConstraintSolvingIncompleteError) -> String {
    String::from(
      "Type inference failed to complete, you may see some confusing types and type errors.",
    )
  }

  pub fn operator_call_13(&self, e: &CannotCallNonFunction) -> String {
    let t = follow_type::follow(e.ty);

    if let Some(union_ty) = get_type::get::<UnionType>(t) {
      let mut err = String::from("Cannot call a value of the union type:");

      // C++ `for (auto option : unionTy)`——UnionTypeIterator 展平嵌套 union
      // 并 follow,裸遍历 options 会漏掉嵌套成员。
      for option in begin_union_type(union_ty) {
        let option = follow_type::follow(option);

        if get_type::get::<FunctionType>(option).is_some()
          || self.find_call_metamethod(option).is_some()
        {
          err.push_str("\n  | ");
          err.push_str(&to_string_type_id(option));
          continue;
        }

        return format!(
          "Cannot call a value of type {} in union:\n  {}",
          to_string_type_id(option),
          to_string_type_id(e.ty)
        );
      }

      err.push_str("\nWe are unable to determine the appropriate result type for such a call.");
      return err;
    }

    if let Some(primitive_ty) = get_type::get::<PrimitiveType>(t)
      && primitive_ty.r#type == PrimitiveType::FUNCTION
    {
      return format!(
        "The type {} is not precise enough for us to determine the appropriate result type of this call.",
        to_string_type_id(e.ty)
      );
    }

    format!("Cannot call a value of type {}", to_string_type_id(e.ty))
  }

  pub fn operator_call_23(&self, e: &ExtraInformation) -> String {
    String::from(e.message())
  }

  pub fn operator_call_20(&self, e: &DeprecatedApiUsed) -> String {
    format!(
      "The property .{} is deprecated.  Use .{} instead.",
      e.symbol, e.use_instead
    )
  }

  pub fn operator_call_33(&self, e: &ModuleHasCyclicDependency) -> String {
    if e.cycle().is_empty() {
      return String::from("Cyclic module dependency detected");
    }

    let mut s = String::from("Cyclic module dependency: ");

    {
      let mut writer = SepWriter::new(&mut s, " -> ");
      for name in e.cycle() {
        let readable = self
          .file_resolver_ref()
          .map(|r| r.get_human_readable_module_name(name));
        writer.push(readable.as_deref().unwrap_or(name));
      }
    }

    s
  }

  pub fn operator_call_25(&self, e: &FunctionExitsWithoutReturning) -> String {
    let expected_type_str = to_string_type_pack_id(e.expected_return_type);
    format!(
      "Not all codepaths in this function return '{}'.",
      expected_type_str
    )
  }
}

fn find_binary_op(name: &str) -> Option<&'static str> {
  match name {
    "add" => Some("+"),
    "sub" => Some("-"),
    "mul" => Some("*"),
    "div" => Some("/"),
    "idiv" => Some("//"),
    "pow" => Some("^"),
    "mod" => Some("%"),
    "concat" => Some(".."),
    "lt" => Some("< or >="),
    "le" => Some("<= or >"),
    "eq" => Some("== or ~="),
    _ => None,
  }
}

fn find_unary_op(name: &str) -> Option<&'static str> {
  match name {
    "unm" => Some("-"),
    "len" => Some("#"),
    "not" => Some("not"),
    _ => None,
  }
}

fn is_unreachable_type_function(name: &str) -> bool {
  matches!(
    name,
    "refine" | "singleton" | "union" | "intersect" | "and" | "or"
  )
}
