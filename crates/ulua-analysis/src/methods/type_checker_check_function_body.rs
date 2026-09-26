use ulua_ast::records::{ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_stat::AstStat};

use crate::{
  functions::{
    allows_no_return_values::allows_no_return_values, arc_as_mut::arc_as_mut,
    as_mutable_type_pack::as_mutable_type_pack, follow_type_pack,
    get_end_location::get_end_location, get_fallthrough::get_fallthrough, get_mutable_type,
    get_type_pack,
  },
  records::{
    free_type_pack::FreeTypePack, function_exits_without_returning::FunctionExitsWithoutReturning,
    function_type::FunctionType, type_checker::TypeChecker, type_pack::TypePack,
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

    let Some(fun_ty) = get_mutable_type::get_mutable::<FunctionType>(ty) else {
      self.ice_string("Checking non functional type");
      return;
    };

    self.check_stat_block(scope, &function.body);

    // We explicitly don't follow here to check if we have a 'true' free type instead of bound one
    let ret_pack_is_bound = get_type_pack::get::<BoundTypePack>(fun_ty.ret_types).is_some();
    if !ret_pack_is_bound && get_type_pack::get::<FreeTypePack>(fun_ty.ret_types).is_some() {
      // SAFETY: as_mutable_type_pack 去除 const（C++ asMutable 同义），ret_types 有效。
      let ret_pack = as_mutable_type_pack(fun_ty.ret_types);
      unsafe {
        (*ret_pack).ty = TypePackVariant::TypePack(TypePack::empty());
      }
    }

    let reaches_implicit_return =
      !get_fallthrough(function.body.cast::<AstStat>().as_ptr()).is_null();

    if reaches_implicit_return
      && !allows_no_return_values(follow_type_pack::follow(fun_ty.ret_types))
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
    let module = unsafe {
      &mut *(arc_as_mut(self.expect_current_module()))
    };
    if module.ast_types.find(&key).is_none() {
      *module.ast_types.get_or_insert(key) = ty;
    }
  }
}
