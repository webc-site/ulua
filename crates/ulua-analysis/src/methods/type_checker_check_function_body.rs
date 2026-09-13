use alloc::{sync::Arc, vec::Vec};

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_stat::AstStat};

use crate::{
  functions::{
    allows_no_return_values::allows_no_return_values,
    as_mutable_type_pack::as_mutable_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_end_location::get_end_location, get_fallthrough::get_fallthrough,
    get_mutable_type::get_mutable_type_id, get_type_pack::get_type_pack_id,
  },
  records::{
    free_type_pack::FreeTypePack, function_exits_without_returning::FunctionExitsWithoutReturning,
    function_type::FunctionType, module::Module, type_checker::TypeChecker, type_pack::TypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_variant::TypePackVariant,
  },
};
impl TypeChecker {
  pub fn check_function_body(&mut self, scope: &ScopePtr, ty: TypeId, function: &AstExprFunction) {
    // LUAU_TIMETRACE_SCOPE("TypeChecker::checkFunctionBody", "TypeChecker");
    // (TimeTrace argument bookkeeping is a no-op in the Rust port.)

    let Some(fun_ty) = get_mutable_type_id::<FunctionType>(ty) else {
      self.ice_string("Checking non functional type");
      return;
    };

    // SAFETY: function.body 指向 AST arena 节点（parser 保证非空）。
    self.check_scope_ptr_ast_stat_block(scope, unsafe { &*function.body });

    // We explicitly don't follow here to check if we have a 'true' free type instead of bound one
    let ret_pack_is_bound = get_type_pack_id::<BoundTypePack>(fun_ty.ret_types).is_some();
    if !ret_pack_is_bound && get_type_pack_id::<FreeTypePack>(fun_ty.ret_types).is_some() {
      // SAFETY: as_mutable_type_pack_id 去除 const（C++ asMutable 同义），ret_types 有效。
      let ret_pack = as_mutable_type_pack_id(fun_ty.ret_types);
      unsafe {
        (*ret_pack).ty = TypePackVariant::TypePack(TypePack {
          head: Vec::new(),
          tail: None,
        });
      }
    }

    let reaches_implicit_return = !get_fallthrough(function.body as *const AstStat).is_null();

    if reaches_implicit_return
      && !allows_no_return_values(unsafe { follow_type_pack_id(fun_ty.ret_types) })
    {
      // If we're in nonstrict mode we want to only report this missing return
      // statement if there are type annotations on the function. In strict mode
      // we report it regardless.
      if !self.is_nonstrict_mode() || !function.return_annotation.is_null() {
        self.report_error_location_type_error_data(
          &get_end_location(function),
          TypeErrorData::FunctionExitsWithoutReturning(FunctionExitsWithoutReturning {
            expected_return_type: fun_ty.ret_types,
          }),
        );
      }
    }

    let key = function as *const AstExprFunction as *const AstExpr;
    // SAFETY: current_module 在类型检查期间独占使用（C++ 直接改 module->astTypes 同义）；
    // Arc::as_ptr 转 *mut 仅供此处插入，单线程无别名。
    let module =
      unsafe { &mut *(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module) };
    if module.ast_types.find(&key).is_none() {
      *module.ast_types.get_or_insert(key) = ty;
    }
  }
}
