use alloc::vec;

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::{
    as_mutable_type_pack_alt_d::as_mutable_type_pack,
    check_require_path_dcr::check_require_path_dcr,
  },
  records::{
    generic_error::GenericError, magic_function_call_context::MagicFunctionCallContext,
    type_pack::TypePack,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_variant::TypePackVariant},
};
pub fn magic_require_infer(context: &MagicFunctionCallContext) -> bool {
  let call_site = unsafe { context.call_site.as_ref() };
  let solver = unsafe { &mut *context.solver.as_ptr() };

  if call_site.args.size != 1 {
    solver.report_error_type_error_data_location(
      TypeErrorData::GenericError(GenericError::new("require takes 1 argument".to_string())),
      &call_site.base.base.location,
    );
    return false;
  }

  let arg = unsafe { *call_site.args.data.add(0) };
  if !check_require_path_dcr(context.solver, arg) {
    return false;
  }

  let module = match solver.module.as_ref() {
    Some(module) => module.clone(),
    None => return false,
  };

  let module_info = unsafe {
    ((*solver.module_resolver).vtable.resolve_module_info)(
      solver.module_resolver,
      &module.name,
      context.call_site.as_ptr() as *const AstExpr,
    )
  };

  if let Some(module_info) = module_info {
    let module_type = solver.resolve_module(&module_info, &call_site.base.base.location);
    let module_result = unsafe {
      (*solver.arena).add_type_pack_t(TypePack {
        head: vec![module_type],
        tail: None,
      })
    };
    let result_tp = as_mutable_type_pack(context.result);
    unsafe {
      (*result_tp).ty = TypePackVariant::Bound(module_result);
    }

    return true;
  }

  false
}
