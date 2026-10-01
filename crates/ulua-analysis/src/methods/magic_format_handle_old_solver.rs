use alloc::vec::Vec;
use core::ptr::NonNull;
use std::cmp::min;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
  ast_expr_group::AstExprGroup, ast_expr_index_name::AstExprIndexName, node_handle::OptNode,
};

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, parse_format_string::parse_format_string_bytes,
    shared_mut::shared_mut,
  },
  records::{
    count_mismatch::{CountMismatch, CountMismatchContext},
    type_checker::TypeChecker,
    type_error::TypeError,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
pub fn magic_format_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let (param_pack, _predicates) = (with_predicate.r#type, with_predicate.predicates);

  let module = typechecker.current_module.as_ref()?;
  // Safety: `shared_mut(module)` 返回 `Arc<Module>` 内嵌 Module 的裸地址（Arc 由
  // `current_module` 持有、`module` 借用覆盖全函数，指针非空对齐且存活）。取
  // `&mut internal_types` 是 C++ `asMutable(module)->internalTypes` 的等价惯用法：
  // 全函数单线程串行，此 arena 无第二处可变句柄并存。
  let arena = { &mut (shared_mut(module)).internal_types };

  // `func`/`args` 槽仍是 records 引用化波次前的裸指针字段：链上判型全部走
  // 生命周期正确的 `try_as`/`ast_node_try_as`（局部 `OptNode` 句柄折叠判空），
  // 候选格式串表达式仅以身份指针传递（Copy，无借用逃逸），最终在 fn 作用域
  // 句柄上兑现借用，不再锻造 'static。
  let mut fmt_slot: OptNode<AstExpr> = OptNode::default();

  if expr.self_ {
    let func = OptNode::from_ptr(expr.func);
    if let Some(index) = func.try_as::<AstExprIndexName>() {
      fmt_slot = match index.expr.try_as::<AstExprGroup>() {
        Some(group) => OptNode::from_ptr(group.expr.as_ptr()),
        None => OptNode::from_ptr(index.expr.as_ptr()),
      };
    }
  }

  if !expr.self_ && expr.args.size > 0 {
    // size > 0 保证第 0 实参在界内，改用安全切片取元素指针。
    fmt_slot = OptNode::from_ptr(expr.args.as_slice()[0]);
  }

  let fmt = fmt_slot.try_as::<AstExprConstantString>()?;

  // `typechecker.builtin_types` 为 Handle（NonNull 编码非空）持有的会话级内置
  // 类型表，比本次调用长寿，as_ptr 还原裸地址后 new_unchecked 不变量由句柄
  // 成立；`fmt` 经生命周期正确的 `try_as` 命中，指向存活 AstExprConstantString。格式扫描
  // 走安全门面 `parse_format_string_bytes` + `as_bytes`（与旧
  // `parse_format_string` 的 c_slice 折算逐位等价），不再触碰 data/size 裸字段。
  let expected: Vec<TypeId> = parse_format_string_bytes(
    // Safety: Handle 非空不变量由类型保证（注释同上）。
    unsafe { NonNull::new_unchecked(typechecker.builtin_types.as_ptr()) },
    fmt.value.as_bytes(),
  );

  let (params, tail) = flatten_type_pack_id(param_pack);

  let param_offset: usize = 1;
  let data_offset: usize = if expr.self_ { 0 } else { 1 };

  for (i, &expected_ty) in expected.iter().enumerate() {
    let Some(param) = params.get(i + param_offset) else {
      break;
    };
    // No argument expressions ⇒ nothing to attach a location to, and
    // `args.size - 1` would underflow (the self-call path lacks the
    // `args.size > 0` guard the non-self path has).
    if expr.args.size == 0 {
      break;
    }

    let arg_index = min(i + data_offset, expr.args.size - 1);
    // arg_index 经 min 截断且 size>0 已由上方早退保证，恒在 args 界内；元素
    // 是 parser 写入 arena 的存活表达式节点，仅只读其 base.location 交给
    // unify 记录错误位置。元素解引用收口在 `iter_nodes` 门面。
    let location = &expr
      .args
      .iter_nodes()
      .nth(arg_index as usize)
      .expect("arg_index 经 min 截断恒在 args 界内")
      .base
      .location;

    typechecker.unify_type_id_type_id_scope_ptr_location(*param, expected_ty, scope, location);
  }

  let num_actual_params = params.len();
  let num_expected_params = expected.len() + 1;

  if num_expected_params != num_actual_params
    && (!tail.is_some() || num_expected_params < num_actual_params)
  {
    let error = TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::CountMismatch(CountMismatch {
        expected: num_expected_params,
        maximum: None,
        actual: num_actual_params,
        context: CountMismatchContext::Arg,
        is_variadic: false,
        function: String::new(),
      }),
    );
    typechecker.report_error_type_error(&error);
  }

  Some(WithPredicate::with_predicate_t(
    arena.add_type_pack_initializer_list_type_id(&[typechecker.string_type]),
  ))
}
