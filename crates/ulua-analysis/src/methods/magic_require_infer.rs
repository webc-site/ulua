use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, check_require_path_dcr::check_require_path_dcr,
  },
  records::{
    generic_error::GenericError, magic_function_call_context::MagicFunctionCallContext,
    type_pack::TypePack,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_variant::TypePackVariant},
};
pub fn magic_require_infer(context: &MagicFunctionCallContext) -> bool {
  // Safety: call_site 是 NonNull<AstExprCall>，NonNull 不变量保证非空且对齐；它指向
  // 本次 magic 推断的调用点 AST 节点（arena 持有、同步调用期内存活），取只读共享引用。
  let call_site = unsafe { context.call_site.as_ref() };
  // Safety: solver 是 NonNull<ConstraintSolver>，非空对齐；该 solver 为本次同步 magic
  // 推断的独占活跃对象（单线程），重建 &mut 期间无并发借用且比本函数长寿。
  let solver = unsafe { &mut *context.solver.as_ptr() };

  if call_site.args.size != 1 {
    solver.report_error_type_error_data_location(
      TypeErrorData::GenericError(GenericError::new("require takes 1 argument".to_string())),
      &call_site.base.base.location,
    );
    return false;
  }

  // 等价替换：args.size == 1 已在上方校验，as_slice()（safe，arena 按 data/size 成对
  // 写入保证区域合法）取首元素即原 `*data.add(0)` 的只读访问，故此处无需 unsafe。
  let arg = call_site.args.as_slice()[0];
  if !check_require_path_dcr(context.solver, arg) {
    return false;
  }

  let module = match solver.module.as_ref() {
    Some(module) => module.clone(),
    None => return false,
  };

  // cpp: `resolveModuleInfo(resolveFrom, *context.callSite)` —— AstExprCall 的
  // 基类 `AstExpr` 视图（同 cpp 的上行转换）。
  // Safety: context.call_site 指向存活 AstExprCall，AstExpr 是其 repr(C) 首字段、
  // 基址重合，故可安全取 &AstExpr 只读上行视图供 resolve_module_info 读取。
  let call_site_expr = unsafe { &*(context.call_site.as_ptr() as *const AstExpr) };
  let module_info = solver
    .module_resolver_ref()
    .resolve_module_info(&module.name, call_site_expr);

  if let Some(module_info) = module_info {
    let module_type = solver.resolve_module(&module_info, &call_site.base.base.location);
    // Safety: solver.arena 是构造时接线的 *mut TypeArena，非空、对齐且比 solver 长寿；
    // add_type_pack_t 向 arena 追加类型包，单线程同步推断下对该 arena 的独占可变
    // 借用此刻无其他活跃别名。
    let module_result = {
      solver
        .arena
        .get_mut()
        .add_type_pack_t(TypePack::single(module_type))
    };
    let result_tp = as_mutable_type_pack(context.result);
    // Safety: context.result 为分配于 arena 的结果 TypePackId（非空、存活），
    // as_mutable_type_pack 据其基址给出写句柄；本次 magic 是该类型包唯一一次绑定，
    // 同步流程此刻其结果位无其他活跃借用，&mut 重建不与他者别名冲突。
    unsafe {
      (*result_tp).ty = TypePackVariant::Bound(module_result);
    }

    return true;
  }

  false
}
