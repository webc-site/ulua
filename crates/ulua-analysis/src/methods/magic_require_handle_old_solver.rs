use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  functions::{arc_as_mut::arc_as_mut, check_require_path::check_require_path},
  records::{
    generic_error::GenericError, type_checker::TypeChecker, type_pack::TypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_pack_id::TypePackId,
  },
};
pub fn magic_require_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  _with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let module = typechecker.current_module.as_ref()?.clone();
  // Safety: arc_as_mut 返回 Arc 堆数据的稳定地址（与 Arc::as_ptr 同址，非空）；
  // 此处重建 &mut 只覆盖 internal_types 字段的字节范围。单线程串行：借用存续
  // 期间（至 add_type_pack_t 返回止）对同一字节的读写字段仅有 arena 自身，
  // 穿插的 check_require_path 只操作实参 AST 与 typechecker 的 resolver 状态、
  // resolve_module_info 仅读 module.name（另一字段，地址区间不相交），均未
  // 触碰 internal_types 字节，故无别名冲突；Module 由 Arc 保活，不悬垂。
  let arena = unsafe { &mut (*(arc_as_mut(&module))).internal_types };

  if expr.args.size != 1 {
    typechecker.report_error_location_type_error_data(
      &expr.base.base.location,
      TypeErrorData::GenericError(GenericError::new("require takes 1 argument".to_string())),
    );
    return None;
  }

  let arg = expr.args[0];
  if !check_require_path(typechecker, arg) {
    return None;
  }

  // cpp BuiltinDefinitions.cpp:1899：handleOldSolver 以 `expr`（AstExprCall 的
  // 基类 AstExpr 视图）为 require-trace 查找 key，而非实参节点；
  // 与 magic_require_infer.rs 的 call_site_expr 处理一致。
  // arg 仅用于上方 check_require_path。
  let module_info = typechecker
    .resolver_ref()
    .resolve_module_info(&module.name, &expr.base);

  if let Some(module_info) = module_info {
    let require_type = typechecker.check_require(scope, &module_info, &expr.base.base.location);
    return Some(WithPredicate::with_predicate_t(
      arena.add_type_pack_t(TypePack::single(require_type)),
    ));
  }

  None
}
