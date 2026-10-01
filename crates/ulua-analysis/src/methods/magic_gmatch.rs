//! `magic_gmatch` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::{NonNull, null_mut};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    node_handle::OptNode,
  },
  rtti::ast_node_try_as,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes, shared_mut::shared_mut,
  },
  records::{
    arena_handle::{alias, alias_ref},
    function_type::FunctionType,
    magic_function_call_context::MagicFunctionCallContext,
    magic_gmatch::MagicGmatch,
    type_checker::TypeChecker,
    type_pack::TypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};

pub fn magic_gmatch_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;
  let (params, _tail) = flatten_type_pack_id(param_pack);

  if params.len() != 2 {
    return None;
  }

  let module = typechecker.current_module.as_ref()?;
  // Safety: `shared_mut(module)` 返回的裸指针来自 `typechecker.current_module` 内
  // 存活的 `Arc<Module>`，本函数单线程执行且 `module` 借用贯穿整个函数；`internal_types`
  // 是 arena 字段、地址不随新增类型移动，重建其可变借用无并发/别名冲突（沿用仓库
  // `shared_mut` 的独占写约定）。
  let arena = { &mut (shared_mut(module)).internal_types };

  let index = if expr.self_ { 0 } else { 1 };
  // `args` 槽位仍是裸指针：size 守卫在界内后取元素指针，经句柄门面
  // `OptNode::from_ptr` 把可空性与缺位一并折叠为 `Option`，判型下转走生命
  // 周期正确的 [`ast_node_try_as`]，借用半径由本函数局部句柄供给。
  let pattern_node = if expr.args.size > index {
    OptNode::from_ptr(expr.args.as_slice()[index])
  } else {
    OptNode::from_ptr(null_mut())
  };
  let pattern = pattern_node
    .get()
    .and_then(|a| ast_node_try_as::<AstExprConstantString>(a))?;

  // Safety: `pattern` 由 `try_as` class-index 命中保证其确为存活的
  // `AstExprConstantString` arena 节点，只读借用读取 `value` 字节安全；
  // `builtin_types` 为 Handle（NonNull 编码非空）单例，as_ptr 还原的裸地址
  // 非空故 `NonNull::new(..).unwrap()` 不会 panic。
  let return_types: Vec<_> = parse_pattern_string_bytes(
    NonNull::new(typechecker.builtin_types.as_ptr())
      .expect("Handle 以 NonNull 编码非空，as_ptr 还原恒非空"),
    pattern.value.as_bytes(),
  );

  if return_types.is_empty() {
    return None;
  }

  // 走到此处必有 `expr.args.size > index >= 0`，故 `size >= 1`，下标 0 在界内，
  // `as_slice()[0]` 安全取得首个实参节点指针。
  let first_arg = expr.args.as_slice()[0];
  let first_location = &alias_ref(first_arg).base.location;
  typechecker.unify_type_id_type_id_scope_ptr_location(
    params[0],
    typechecker.string_type,
    scope,
    first_location,
  );

  let empty_pack = arena.add_type_pack_t(TypePack::empty());
  let return_list = arena.add_type_pack_t(TypePack::from_vec(return_types));
  let iterator_type = arena.add_type(FunctionType::function_type_new(
    empty_pack,
    return_list,
    None,
    false,
  ));
  Some(WithPredicate::with_predicate_t(
    arena.add_type_pack_t(TypePack::single(iterator_type)),
  ))
}

impl MagicGmatch {
  pub fn infer(&self, context: &MagicFunctionCallContext) -> bool {
    let (params, _tail) = flatten_type_pack_id(context.arguments);

    if params.len() != 2 {
      return false;
    }

    let solver = alias_ref(context.solver.as_ptr());
    let call_site = alias_ref(context.call_site.as_ptr());

    let index = if call_site.self_ { 0 } else { 1 };
    // 同上：`args` 槽位经 `OptNode::from_ptr` 折叠可空/缺位，下转走生命周期
    // 正确的 [`ast_node_try_as`]，借用半径由局部句柄供给。
    let pattern_node = if call_site.args.size > index {
      OptNode::from_ptr(call_site.args.as_slice()[index])
    } else {
      OptNode::from_ptr(null_mut())
    };
    let Some(pattern) = pattern_node
      .get()
      .and_then(|a| ast_node_try_as::<AstExprConstantString>(a))
    else {
      return false;
    };

    let return_types =
      parse_pattern_string_bytes(solver.builtin_types.as_nonnull(), pattern.value.as_bytes());

    if return_types.is_empty() {
      return false;
    }

    // constraint 仅作身份指针透传。
    let builtin_types = &solver.builtin_types.get_mut();
    alias(context.solver.as_ptr()).constraint_solver_unify(
      context.constraint.as_ptr(),
      params[0],
      builtin_types.string_type,
    );

    let arena = alias(context.solver.as_ptr()).arena.get_mut();

    let empty_pack = arena.add_type_pack_t(TypePack::empty());
    let return_list = arena.add_type_pack_t(TypePack::from_vec(return_types));
    let iterator_type = arena.add_type(FunctionType::function_type_new(
      empty_pack,
      return_list,
      None,
      false,
    ));
    let res_type_pack = arena.add_type_pack_t(TypePack::single(iterator_type));

    let result_mut = as_mutable_type_pack(context.result);
    alias(result_mut).ty = TypePackVariant::Bound(res_type_pack);

    true
  }
}
