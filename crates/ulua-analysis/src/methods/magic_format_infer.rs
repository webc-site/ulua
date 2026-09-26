use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_index_name::AstExprIndexName,
  },
  rtti::ast_node_try_as_ptr,
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
    count_mismatch::CountMismatch, error_suppression::ErrorSuppression,
    magic_function_call_context::MagicFunctionCallContext, type_error::TypeError,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_variant::TypePackVariant},
};
pub fn magic_format_infer(context: &MagicFunctionCallContext) -> bool {
  // Safety: `context.solver` 是派发本约束时以 NotNull 语义注入的求解器地址，
  // 指向调用栈内存活、覆盖本次 infer 全程的 ConstraintSolver。
  let solver = unsafe { context.solver.as_ref() };
  // Safety: `solver.arena` 构造期指向 solver 内嵌 TypeArena，本次 infer 的
  // 结果类型包分配是它的唯一可变借用路径。
  let arena = { &mut solver.arena.get_mut() };

  let iter = { begin(context.arguments) };
  let end_iter = { end(context.arguments) };

  // we'll suppress any errors for `string.format` if the format string is error suppressing.
  // Safety: `||` 短路保证仅当 iter != end 时才走到这里，故 `*iter.current()`
  // 落在有效非尾位置；`solver.normalizer`
  // 为构造期 NotNull 注入的存活 Normalizer，满足 should_suppress_errors 契约。
  if iter == end_iter
    || unsafe {
      should_suppress_errors(
        solver.normalizer.as_ptr(),
        follow_type::follow(*iter.current()),
      )
    } == ErrorSuppression::from_value(Value::Suppress)
  {
    // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，比本次调用
    // 长寿且此处只读取 string_type 的 TypeId 值。
    let result_pack =
      arena.add_type_pack_initializer_list_type_id(&[solver.builtin_types.get().string_type]);
    let result_mut = as_mutable_type_pack(context.result);
    // Safety: `context.result` 是待定版的 arena TypePackVar，TypePackId→*mut
    // 对应 cpp `asMutable` 的 const_cast，本次 infer 独占写其 `ty` 为 Bound。
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(result_pack);
    }
    return true;
  }

  let mut fmt: Option<&AstExprConstantString> = None;

  {
    // Safety: `context.call_site` 指向 parser 拥有、求解期间稳定的当前
    // AstExprCall 节点，本块只读其 func/self_/args。
    let call_site = unsafe { context.call_site.as_ref() };
    if call_site.func.is_null() {
      return false;
    }

    // Safety: `call_site.func` 是 parser 写入的非空子表达式指针，指向存活
    // AstExpr；try_as_ptr 先判空再按 class_index 甄别，命中即 repr(C) 基址重合的
    // 存活 AstExprIndexName 只读借用，读取其 `expr` 字段为共享读。
    let index_expr = unsafe { ast_node_try_as_ptr::<AstExprIndexName>(call_site.func) };
    if let Some(index_expr) = index_expr.filter(|_| call_site.self_) {
      // expr 已句柄化恒非空；unwrap_group 行走链为既有裸指针 API，经 as_ptr 桥接。
      let unwrapped = unwrap_group(index_expr.expr.as_ptr());
      // Safety: `unwrapped` 经 unwrap_group 剥组后仍指向 arena 的存活表达式
      // 节点（或 null）；try_as_ptr 判空+甄别，命中即存活 AstExprConstantString
      // 只读借用，模块存活、地址不移动。
      fmt = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(unwrapped) };
    }

    if !call_site.self_ && call_site.args.size > 0 {
      // size > 0 保证 [0] 在界内，安全切片取首元素表达式指针再下转。
      // Safety: 该元素是 arena 写入 args 的存活表达式节点；try_as_ptr 判空+甄别，
      // 命中即存活 AstExprConstantString 只读借用。
      fmt = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(call_site.args.as_slice()[0]) };
    }
  }

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
    // Safety: `as_ptr` 取回仍独占存活的 solver 地址；constraint_solver_unify 需
    // `&mut self`，循环内本调用同步持有 solver 独占访问，无并发别名。
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
      // Safety: `context.call_site` 指向求解期间稳定存活的当前 AstExprCall，
      // 仅读取其 location 元数据。
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
    // Safety: 对独占存活 solver 地址的可变借用，`report_error_type_error` 向
    // 其错误向量追加，本调用内无并发别名。
    unsafe {
      (*context.solver.as_ptr()).report_error_type_error(error);
    }
  }

  // This is invoked at solve time, so we just need to provide a type for the result of :/.format
  let result_pack = arena.add_type_pack_initializer_list_type_id(&[
    // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，比本次调用
    // 长寿且此处只读 string_type。
    solver.builtin_types.get().string_type,
  ]);
  let result_mut = as_mutable_type_pack(context.result);
  // Safety: `context.result` 为待定版 arena TypePackVar，const_cast 后由本次
  // infer 独占写 Bound(string)。
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(result_pack);
  }

  true
}
