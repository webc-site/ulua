use alloc::string::ToString;
use core::{cmp::min, ptr::null_mut, str::from_utf8};

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_index_name::AstExprIndexName,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::FFlag;

use crate::{
  enums::value::Value,
  functions::{
    begin_type_pack::begin, end_type_pack::end, flatten_type_pack::flatten_type_pack_id,
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    parse_format_string::parse_format_string_bytes,
    should_suppress_errors_type_utils::should_suppress_errors, unwrap_group::unwrap_group,
  },
  records::{
    cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls,
    count_mismatch::{CountMismatch, CountMismatchContext},
    error_suppression::ErrorSuppression,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    type_mismatch::TypeMismatch,
  },
  type_aliases::type_error_data::TypeErrorData,
};
pub fn magic_format_type_check(context: &MagicFunctionTypeCheckContext) -> bool {
  let typechecker = unsafe { &mut *context.typechecker.as_ptr() };
  let call_site = unsafe { &*context.call_site };

  let iter = { begin(context.arguments) };
  let end_iter = { end(context.arguments) };

  if iter.operator_eq(&end_iter) {
    typechecker.report_error_type_error_data_location(
      TypeErrorData::CountMismatch(CountMismatch {
        expected: 1,
        maximum: None,
        actual: 0,
        context: CountMismatchContext::Arg,
        is_variadic: true,
        function: "string.format".to_string(),
      }),
      &call_site.base.base.location,
    );
    return true;
  }

  // we'll suppress any errors for `string.format` if the format string is error suppressing.
  if unsafe {
    should_suppress_errors(
      &mut typechecker.normalizer as *mut _,
      follow_type_id(*iter.operator_deref()),
    )
  } == ErrorSuppression::from_value(Value::Suppress)
  {
    return true;
  }

  let mut fmt: *mut AstExprConstantString = null_mut();
  if !call_site.func.is_null() {
    let index_expr = unsafe { ast_node_as::<AstExprIndexName>(call_site.func as *mut AstNode) };
    if !index_expr.is_null() && call_site.self_ {
      let unwrapped = unwrap_group(unsafe { &mut *index_expr }.expr);
      fmt = unsafe { ast_node_as::<AstExprConstantString>(unwrapped as *mut AstNode) };
    }
  }

  if !call_site.self_ && call_site.args.size > 0 {
    fmt = unsafe { ast_node_as::<AstExprConstantString>(*call_site.args.data as *mut AstNode) };
  }

  let mut format_string: Option<&str> = None;
  if !fmt.is_null() {
    let fmt_ref = unsafe { &*fmt };
    format_string = from_utf8(fmt_ref.value.as_bytes()).ok();
  } else {
    let first_arg = { *iter.operator_deref() };
    let followed = follow_type_id(first_arg);
    if let Some(singleton) = get_type_id::<SingletonType>(followed)
      && let Some(string_singleton) = singleton.variant.get_if::<StringSingleton>()
    {
      format_string = Some(&string_singleton.value);
    }
  }

  if FFlag::LuauSilenceDynamicFormatStringErrors.get() {
    if format_string.is_none() {
      return true;
    }
  } else if format_string.is_none() {
    typechecker.report_error_type_error_data_location(
      TypeErrorData::CannotCheckDynamicStringFormatCalls(
        CannotCheckDynamicStringFormatCalls::default(),
      ),
      &call_site.base.base.location,
    );
    return true;
  }

  // CLI-150726: The block below effectively constructs a type pack and then type checks it by going parameter-by-parameter.
  let format_str = format_string.unwrap();
  let expected = parse_format_string_bytes(context.builtin_types, format_str.as_bytes());

  let (params, _tail) = flatten_type_pack_id(context.arguments);

  let param_offset = 1;
  // Compare the expressions passed with the types the function expects to determine whether this function was called with : or .
  let called_with_self = expected.len() == call_site.args.size;
  // unify the prefix one argument at a time
  for (i, &expected_ty) in expected.iter().enumerate() {
    let Some(&actual_ty) = params.get(i + param_offset) else {
      break;
    };
    // No argument expressions ⇒ nothing to attach a location to, and
    // `args.size - 1` would underflow.
    if call_site.args.size == 0 {
      break;
    }
    let arg_index = min(
      call_site.args.size - 1,
      i + if called_with_self { 0 } else { param_offset },
    );
    let location = unsafe { (*(*call_site.args.data.add(arg_index))).base.location };
    // use subtyping instead here
    let scope_ptr = context.check_scope.as_ptr();
    let result = unsafe {
      (*typechecker.subtyping).is_subtype_type_id_type_id_not_null_scope(
        actual_ty,
        expected_ty,
        scope_ptr,
      )
    };

    if !result.is_subtype {
      match unsafe { should_suppress_errors(&mut typechecker.normalizer as *mut _, actual_ty) }
        .value
      {
        Value::Suppress => {}
        Value::NormalizationFailed => {}
        Value::DoNotSuppress => {
          let reasonings = typechecker
            .explain_reasonings_type_id_type_id_location_subtyping_result(
              actual_ty,
              expected_ty,
              location,
              &result,
            );

          if !reasonings.suppressed {
            let reason = reasonings.to_string();
            typechecker.report_error_type_error_data_location(
              TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given_reason(
                expected_ty,
                actual_ty,
                reason,
              )),
              &location,
            );
          }
        }
      }
    }
  }

  true
}
