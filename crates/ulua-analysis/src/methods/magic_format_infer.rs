use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
  ast_expr_index_name::AstExprIndexName, node_handle::OptNode,
};

use crate::{
  enums::value::Value,
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, begin_type_pack::begin, end_type_pack::end,
    extract_format_string::extract_format_string, flatten_type_pack::flatten_type_pack_id,
    follow_type, parse_format_string::parse_format_string_bytes,
    should_suppress_errors_type_utils::should_suppress_errors, unwrap_group::unwrap_group,
  },
  records::{
    arena_handle::{alias, alias_ref},
    count_mismatch::CountMismatch,
    error_suppression::ErrorSuppression,
    magic_function_call_context::MagicFunctionCallContext,
    type_error::TypeError,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_variant::TypePackVariant},
};
pub fn magic_format_infer(context: &MagicFunctionCallContext) -> bool {
  let solver = alias_ref(context.solver.as_ptr());
  // Safety: `solver.arena` 构造期指向 solver 内嵌 TypeArena，本次 infer 的
  // 结果类型包分配是它的唯一可变借用路径。
  let arena = { &mut solver.arena.get_mut() };

  let iter = { begin(context.arguments) };
  let end_iter = { end(context.arguments) };

  // we'll suppress any errors for `string.format` if the format string is error suppressing.
  // `||` 短路保证仅当 iter != end 时才走到这里，故 `*iter.current()`
  // 落在有效非尾位置；`solver.normalizer` 为构造期 NotNull 注入的存活
  // Normalizer，经 Handle::get_mut 以 &mut 直传（should_suppress_errors 已契约前移）。
  if iter == end_iter
    || should_suppress_errors(
      solver.normalizer.get_mut(),
      follow_type::follow(*iter.current()),
    ) == ErrorSuppression::from_value(Value::Suppress)
  {
    // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，比本次调用
    // 长寿且此处只读取 string_type 的 TypeId 值。
    let result_pack =
      arena.add_type_pack_initializer_list_type_id(&[solver.builtin_types.get().string_type]);
    let result_mut = as_mutable_type_pack(context.result);
    alias(result_mut).ty = TypePackVariant::Bound(result_pack);
    return true;
  }

  // 格式串候选以 fn 作用域句柄承载身份，块尾一次 try_as 兑现借用；
  // null/类位不命中都折叠为 None，借用半径不超过 fn 作用域句柄。
  let mut fmt_node: OptNode<AstExpr> = OptNode::default();

  {
    let call_site = alias_ref(context.call_site.as_ptr());
    if call_site.func.is_null() {
      return false;
    }

    // `call_site.func` 是 parser 写入的存活表达式节点：句柄折叠后 try_as 按
    // class_index 甄别 AstExprIndexName，借用止于本 if-let。
    let func_slot = OptNode::from_ptr(call_site.func);
    if let Some(index_expr) = func_slot
      .try_as::<AstExprIndexName>()
      .filter(|_| call_site.self_)
    {
      // expr 已句柄化恒非空；unwrap_group 行走链为既有裸指针 API，经 as_ptr 桥接。
      // 剥组结果（存活节点或 null）继续以句柄承载，判型收敛到尾部 try_as。
      fmt_node = OptNode::from_ptr(unwrap_group(index_expr.expr.as_ptr()));
    }

    if !call_site.self_ && call_site.args.size > 0 {
      // size > 0 保证 [0] 在界内；元素为 arena 存活表达式节点，类位甄别延后统一。
      fmt_node = OptNode::from_ptr(call_site.args.as_slice()[0]);
    }
  }

  let fmt = fmt_node.try_as::<AstExprConstantString>();

  let Some(format_str) = extract_format_string(fmt, &iter) else {
    return false;
  };

  let expected =
    parse_format_string_bytes(solver.builtin_types.as_nonnull(), format_str.as_bytes());

  let (params, tail) = flatten_type_pack_id(context.arguments);

  let param_offset = 1;

  // unify the prefix one argument at a time - needed if any of the involved types are free
  // params[1..] 与 expected 一一对应，zip 取较短的长度
  for (&param, &exp) in params.iter().skip(param_offset).zip(expected.iter()) {
    alias(context.solver.as_ptr()).constraint_solver_unify(context.constraint.as_ptr(), param, exp);
  }

  // if we know the argument count or if we have too many arguments for sure, we can issue an error
  let num_actual_params = params.len();
  let num_expected_params = expected.len() + 1; // + 1 for the format string

  if num_expected_params != num_actual_params
    && (tail.is_none() || num_expected_params < num_actual_params)
  {
    let error = TypeError::type_error_location_type_error_data(
      alias_ref(context.call_site.as_ptr()).base.base.location,
      TypeErrorData::CountMismatch(CountMismatch {
        expected: num_expected_params,
        maximum: None,
        actual: num_actual_params,
        context: CountMismatch::ARG,
        is_variadic: tail.is_some(),
        function: String::new(),
      }),
    );
    alias(context.solver.as_ptr()).report_error_type_error(error);
  }

  // This is invoked at solve time, so we just need to provide a type for the result of :/.format
  let result_pack = arena.add_type_pack_initializer_list_type_id(&[
    // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，比本次调用
    // 长寿且此处只读 string_type。
    solver.builtin_types.get().string_type,
  ]);
  let result_mut = as_mutable_type_pack(context.result);
  alias(result_mut).ty = TypePackVariant::Bound(result_pack);

  true
}
