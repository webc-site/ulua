//! `magic_gmatch` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString},
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, as_mutable_type_pack::as_mutable_type_pack,
    flatten_type_pack::flatten_type_pack_id, parse_pattern_string::parse_pattern_string_bytes,
  },
  records::{
    function_type::FunctionType, magic_function_call_context::MagicFunctionCallContext,
    magic_gmatch::MagicGmatch, type_checker::TypeChecker, type_pack::TypePack,
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

impl MagicGmatch {
  pub fn infer(&self, context: &MagicFunctionCallContext) -> bool {
    let (params, _tail) = flatten_type_pack_id(context.arguments);

    if params.len() != 2 {
      return false;
    }

    // Safety: context.solver 是 MagicFunctionCallContext 构造期接线的
    // NonNull<ConstraintSolver>，非空且活过整个 magic 调用；此处只取共享只读借用。
    let solver = unsafe { context.solver.as_ref() };
    // Safety: context.call_site 同为构造期接线的 NonNull，指向本模块 parse arena
    // 持有的存活 AstExprCall 节点（AST 活过整个约束生成期），只读借用。
    let call_site = unsafe { context.call_site.as_ref() };

    let index = if call_site.self_ { 0 } else { 1 };
    let pattern = if call_site.args.size > index {
      // as_slice 为安全方法：args.data/size 由 parser arena 保证有效区间。
      let expr = call_site.args.as_slice()[index];
      // Safety: expr 是 parser 构造的非空实参节点（parser 子节点非空不变量）；
      // try_as_ptr 自带判空，class index 命中即 repr(C) 首字段基址重合的真实
      // AstExprConstantString 只读借用，未命中返回 None、从不解引用。
      unsafe { ast_node_try_as_ptr::<AstExprConstantString>(expr) }
    } else {
      None
    };

    let Some(pattern) = pattern else {
      return false;
    };

    let return_types =
      parse_pattern_string_bytes(solver.builtin_types.as_nonnull(), pattern.value.as_bytes());

    if return_types.is_empty() {
      return false;
    }

    // Safety: `&*solver.builtin_types` 只读借出构造期接线、此后只读不变的非空
    // BuiltinTypes 单例（借用挂在裸指针点物上，独立于 solver）；
    // `(*context.solver.as_ptr())` 把同一 NonNull 再借成 &mut ConstraintSolver，
    // 此刻 solver 的共享借用已止于上一条字段读取，unify 内部对 arena 的可变
    // 借用随其返回结束——单线程串行窗口，无并存别名；constraint 仅作身份指针透传。
    unsafe {
      let builtin_types = &solver.builtin_types.get_mut();
      (*context.solver.as_ptr()).constraint_solver_unify(
        context.constraint.as_ptr(),
        params[0],
        builtin_types.string_type,
      );
    }

    // Safety: unify 的 &mut 借用已随其返回结束；此处从构造期接线、非空且比
    // solver 长寿的 arena 裸指针字段一次性重建唯一 &mut TypeArena（借用窗口
    // 串行、无并存别名），后续 add_type/add_type_pack_t 返回 bump arena 存活
    // 节点句柄，地址稳定且活过 infer。
    let arena = unsafe { &mut (context.solver.as_ref().arena).get_mut() };

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
    // Safety: context.result 是本次 magic 调用的结果 TypePackId，指向类型 arena
    // 中存活的 TypePackVar 节点（bump arena 地址稳定）；此刻除该裸句柄外无其它
    // 活跃借用，单线程下原地写 `.ty` 等价 C++ `*asMutable(result) = ...`。
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(res_type_pack);
    }

    true
  }
}
