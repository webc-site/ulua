use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString},
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes,
  },
  records::{
    function_type::FunctionType, type_checker::TypeChecker, type_pack::TypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
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
  // Safety: `arc_as_mut(module)` 返回的裸指针来自 `typechecker.current_module` 内
  // 存活的 `Arc<Module>`，本函数单线程执行且 `module` 借用贯穿整个函数；`internal_types`
  // 是 arena 字段、地址不随新增类型移动，重建其可变借用无并发/别名冲突（沿用仓库
  // `arc_as_mut` 的独占写约定）。
  let arena = unsafe { &mut (*(arc_as_mut(module))).internal_types };

  let index = if expr.self_ { 0 } else { 1 };
  let pattern = if expr.args.size > index {
    // `index < expr.args.size` 由上面的 `if` 保证，`as_slice()` 给出与 arena 成对
    // 写入的合法区域，下标访问安全且等价于 `*data.add(index)`。
    let arg = expr.args.as_slice()[index];
    // Safety: `arg` 取自 `expr.args` 数组第 `index` 项，是 parser 写入、arena 存活的
    // 非空 `AstExpr` 节点；try_as_ptr 依 RTTI class index 分派，命中返回同址同型的
    // 存活 `AstExprConstantString` 只读借用、未命中返回 None（对应旧 `is_null` 判定）。
    unsafe { ast_node_try_as_ptr::<AstExprConstantString>(arg) }
  } else {
    None
  };

  let pattern = pattern?;

  // Safety: `pattern` 由 `try_as_ptr` class-index 命中保证其确为存活的
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
  // Safety: `first_arg` 是 `expr.args` 第 0 项、parser 保证非空且 arena 存活的
  // `AstExpr` 节点，读取其 `base.location` 为只读访问。
  let first_location = unsafe { &(*first_arg).base.location };
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
