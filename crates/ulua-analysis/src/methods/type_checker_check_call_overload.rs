use alloc::{sync::Arc, vec::Vec};
use core::ptr::null;

use ulua_ast::records::{ast_expr_call::AstExprCall, ast_node::AstNode, location::Location};

use crate::{
  functions::{
    get_error::get_type_error, get_mutable_type_pack::get_mutable_type_pack_id,
    get_type_alt_j::get_type_id,
  },
  records::{
    any_type::AnyType,
    cannot_call_non_function::CannotCallNonFunction,
    count_mismatch::{CountMismatch, CountMismatchContext},
    extern_type::ExternType,
    free_type::FreeType,
    function_type::FunctionType,
    metatable_type::MetatableType,
    module::Module,
    never_type::NeverType,
    overload_error_entry::OverloadErrorEntry,
    type_checker::TypeChecker,
    type_pack::TypePack,
    unifier_options::UnifierOptions,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    error_type::ErrorType, name_type::Name, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
  },
};
/// `check_call_overload` 的参数包，对应 C++ 十一参数签名
/// （TypeInfer.cpp:4650）。按语义分组：调用上下文 / 重载目标 / 实参 /
/// 重载消解输出发射端。
pub struct CheckCallOverloadArgs<'a> {
  // —— 调用上下文 ——
  pub scope: &'a ScopePtr,
  pub expr: &'a AstExprCall,
  // —— 重载目标与返回/实参 pack ——
  pub fn_ty: TypeId,
  pub ret_pack: TypePackId,
  pub arg_pack: TypePackId,
  // —— 实参明细 ——
  pub args: &'a mut TypePack,
  pub arg_locations: &'a [Location],
  pub arg_list_result: &'a WithPredicate<TypePackId>,
  // —— 重载消解输出发射端 ——
  pub overloads_that_match_arg_count: &'a mut Vec<TypeId>,
  pub overloads_that_dont: &'a mut Vec<TypeId>,
  pub errors: &'a mut Vec<OverloadErrorEntry>,
}
impl TypeChecker {
  pub fn check_call_overload(
    &mut self,
    args: CheckCallOverloadArgs<'_>,
  ) -> Option<Box<WithPredicate<TypePackId>>> {
    // 按原参数顺序解包，保持与 C++ 一一对应。
    let CheckCallOverloadArgs {
      scope,
      expr,
      fn_ty,
      ret_pack,
      arg_pack,
      args,
      arg_locations,
      arg_list_result,
      overloads_that_match_arg_count,
      overloads_that_dont,
      errors,
    } = args;
    // SAFETY: expr.func 指向 AST arena 节点（parser 保证非空）。
    let func_loc = unsafe { (*expr.func).base.location };
    let mut fn_ty = self.strip_from_nil_and_report(fn_ty, &func_loc);

    if get_type_id::<AnyType>(fn_ty).is_some() {
      self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
        self.any_type_pack,
        arg_pack,
        scope,
        &expr.base.base.location,
        CountMismatchContext::Arg,
      );
      return Some(Box::new(WithPredicate::with_predicate_t(
        self.any_type_pack,
      )));
    }

    if get_type_id::<ErrorType>(fn_ty).is_some() {
      return Some(Box::new(WithPredicate::with_predicate_t(
        self.error_recovery_type_pack_scope_ptr(scope.clone()),
      )));
    }

    if get_type_id::<NeverType>(fn_ty).is_some() {
      return Some(Box::new(WithPredicate::with_predicate_t(
        self.uninhabitable_type_pack,
      )));
    }

    if get_type_id::<FreeType>(fn_ty).is_some() {
      // fn is one of the overloads of actualFunctionType, which
      // has been instantiated, so is a monotype. We can therefore
      // unify it with a monomorphic function.
      let mut function = FunctionType::function_type_new(arg_pack, ret_pack, None, false);
      function.level = scope.level;
      let r = self.add_type(&function);

      let options = UnifierOptions {
        is_function_call: true,
      };
      self.unify_type_id_type_id_scope_ptr_location_unifier_options(
        r,
        fn_ty,
        scope,
        &expr.base.base.location,
        &options,
      );

      return Some(Box::new(WithPredicate::with_predicate_t(ret_pack)));
    }

    // Might be a callable table or class
    let mut call_ty: Option<TypeId> = None;
    if let Some(mttv) = get_type_id::<MetatableType>(fn_ty) {
      call_ty = self.get_index_type_from_type(
        scope.clone(),
        mttv.metatable,
        &Name::from("__call"),
        &func_loc,
        false,
      );
    } else if let Some(etv) = get_type_id::<ExternType>(fn_ty)
      && let Some(metatable) = etv.metatable
    {
      call_ty = self.get_index_type_from_type(
        scope.clone(),
        metatable,
        &Name::from("__call"),
        &func_loc,
        false,
      );
    }

    let mut cur_arg_pack = arg_pack;
    let mut cur_args: &mut TypePack = args;
    let mut cur_arg_locations: Vec<Location> = arg_locations.to_vec();

    if let Some(cty) = call_ty {
      // Construct arguments with 'self' added in front
      let meta_call_arg_pack = self.add_type_pack_type_pack(TypePack {
        head: args.head.clone(),
        tail: args.tail,
      });

      // meta_call_arg_pack 刚以 TypePack 变体分配，下转必然成功。
      let meta_call_args = get_mutable_type_pack_id::<TypePack>(meta_call_arg_pack).unwrap();
      meta_call_args.head.insert(0, fn_ty);

      let mut meta_arg_locations = arg_locations.to_vec();
      meta_arg_locations.insert(0, func_loc);

      fn_ty = self.instantiate(scope, cty, func_loc, null());

      cur_arg_pack = meta_call_arg_pack;
      cur_args = meta_call_args;
      cur_arg_locations = meta_arg_locations.clone();
    }

    let Some(ftv) = get_type_id::<FunctionType>(fn_ty) else {
      self.report_error_location_type_error_data(
        &func_loc,
        TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
      );
      let recovery = self.error_recovery_type_pack_scope_ptr(scope.clone());
      self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
        recovery,
        ret_pack,
        scope,
        &func_loc,
        CountMismatchContext::FunctionResult,
      );
      return Some(Box::new(WithPredicate::with_predicate_t(
        self.error_recovery_type_pack_scope_ptr(scope.clone()),
      )));
    };

    // When this function type has magic functions and did return something, we select that overload instead.
    if let Some(magic) = ftv.magic.clone() {
      // TODO: We're passing in the wrong TypePackId. Should be argPack, but a unit test fails otherwise. CLI-40458
      if let Some(ret) = (magic.handle_old_solver)(self, scope, expr, arg_list_result.clone()) {
        return Some(Box::new(WithPredicate::with_predicate_t_predicate_vec(
          ret.r#type,
          ret.predicates,
        )));
      }
    }

    let mut state = self.mk_unifier(scope, &expr.base.base.location);

    // Unify return type_arguments
    let ret_types = ftv.ret_types;
    self.check_argument_list(
      scope,
      // SAFETY: expr.func 指向 AST arena 节点。
      unsafe { &*expr.func },
      &mut state,
      ret_pack,
      ret_types,
      &Vec::new(),
    );
    if !state.errors.is_empty() {
      return None;
    }

    let arg_types = ftv.arg_types;
    self.check_argument_list(
      scope,
      // SAFETY: 同上。
      unsafe { &*expr.func },
      &mut state,
      cur_arg_pack,
      arg_types,
      &cur_arg_locations,
    );

    if !state.errors.is_empty() {
      let mut arg_mismatch = false;
      for err in &state.errors {
        if let Some(cm) = get_type_error::<CountMismatch>(err)
          && cm.context == CountMismatchContext::Arg
        {
          arg_mismatch = true;
          break;
        }
      }

      if !arg_mismatch {
        overloads_that_match_arg_count.push(fn_ty);
      } else {
        overloads_that_dont.push(fn_ty);
      }

      errors.push(OverloadErrorEntry {
        log: state.log.clone(),
        errors: state.errors.clone(),
        arguments: cur_args.head.clone(),
        fn_ty: ftv as *const FunctionType,
      });
    } else {
      state.log.commit();

      // SAFETY: current_module 在类型检查期间独占使用（C++ 直接改
      // module->astOverloadResolvedTypes 同义）。
      unsafe {
        let module_ptr = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
        *(*module_ptr)
          .ast_overload_resolved_types
          .get_or_insert(expr as *const AstExprCall as *const AstNode) = fn_ty;
      }

      // We select this overload
      return Some(Box::new(WithPredicate::with_predicate_t(ret_pack)));
    }

    None
  }
}
