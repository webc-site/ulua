use core::{
  ptr::{NonNull, null_mut},
  str::from_utf8,
};

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_index_name::AstExprIndexName,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{
  enums::value::Value,
  functions::{
    as_mutable_type_pack_alt_d::as_mutable_type_pack, begin_type_pack::begin, end_type_pack::end,
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    get_type_alt_j::get_type_id, parse_format_string::parse_format_string_bytes,
    should_suppress_errors_type_utils::should_suppress_errors, unwrap_group::unwrap_group,
  },
  records::{
    count_mismatch::CountMismatch, error_suppression::ErrorSuppression,
    magic_function_call_context::MagicFunctionCallContext, singleton_type::SingletonType,
    string_singleton::StringSingleton, type_error::TypeError,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_variant::TypePackVariant},
};
pub fn magic_format_infer(context: &MagicFunctionCallContext) -> bool {
  let solver = unsafe { context.solver.as_ref() };
  let arena = unsafe { &mut *solver.arena };

  let iter = { begin(context.arguments) };
  let end_iter = { end(context.arguments) };

  // we'll suppress any errors for `string.format` if the format string is error suppressing.
  if iter.operator_eq(&end_iter)
    || unsafe { should_suppress_errors(solver.normalizer, follow_type_id(*iter.operator_deref())) }
      == ErrorSuppression::from_value(Value::Suppress)
  {
    let result_pack = arena
      .add_type_pack_initializer_list_type_id(&[unsafe { &*solver.builtin_types }.string_type]);
    let result_mut = as_mutable_type_pack(context.result);
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(result_pack);
    }
    return true;
  }

  let mut fmt: *mut AstExprConstantString = null_mut();

  {
    let call_site = unsafe { context.call_site.as_ref() };
    if call_site.func.is_null() {
      return false;
    }

    let index_expr = unsafe { ast_node_as::<AstExprIndexName>(call_site.func as *mut AstNode) };
    if !index_expr.is_null() && call_site.self_ {
      let unwrapped = unwrap_group(unsafe { &mut *index_expr }.expr);
      fmt = unsafe { ast_node_as::<AstExprConstantString>(unwrapped as *mut AstNode) };
    }

    if !call_site.self_ && call_site.args.size > 0 {
      fmt = unsafe { ast_node_as::<AstExprConstantString>(*call_site.args.data as *mut AstNode) };
    }
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

  if format_string.is_none() {
    return false;
  }

  let format_str = format_string.unwrap();
  let expected = parse_format_string_bytes(
    NonNull::new(unsafe { &mut *solver.builtin_types }).unwrap(),
    format_str.as_bytes(),
  );

  let (params, tail) = flatten_type_pack_id(context.arguments);

  let param_offset = 1;

  // unify the prefix one argument at a time - needed if any of the involved types are free
  // params[1..] 与 expected 一一对应，zip 取较短的长度
  for (&param, &exp) in params.iter().skip(param_offset).zip(expected.iter()) {
    unsafe {
      (*context.solver.as_ptr()).constraint_solver_unify(context.constraint.as_ptr(), param, exp);
    }
  }

  // if we know the argument count or if we have too many arguments for sure, we can issue an error
  let num_actual_params = params.len();
  let num_expected_params = expected.len() + 1; // + 1 for the format string

  if num_expected_params != num_actual_params
    && (tail.is_none() || num_expected_params < num_actual_params)
  {
    let error = TypeError::type_error_location_type_error_data(
      unsafe { &*context.call_site.as_ptr() }.base.base.location,
      TypeErrorData::CountMismatch(CountMismatch {
        expected: num_expected_params,
        maximum: None,
        actual: num_actual_params,
        context: CountMismatch::ARG,
        is_variadic: tail.is_some(),
        function: String::new(),
      }),
    );
    unsafe {
      (*context.solver.as_ptr()).report_error_type_error(error);
    }
  }

  // This is invoked at solve time, so we just need to provide a type for the result of :/.format
  let result_pack =
    arena.add_type_pack_initializer_list_type_id(&[unsafe { &*solver.builtin_types }.string_type]);
  let result_mut = as_mutable_type_pack(context.result);
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(result_pack);
  }

  true
}
