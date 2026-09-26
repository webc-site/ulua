use alloc::vec;

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
    type_checker::TypeChecker, type_pack::TypePack, union_type::UnionType,
    with_predicate::WithPredicate,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
pub fn magic_match_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;
  let (params, _tail) = flatten_type_pack_id(param_pack);

  if params.len() < 2 || params.len() > 3 {
    return None;
  }

  let module = typechecker.current_module.as_ref()?;
  // Safety: arc_as_mut 从 current_module 的 ModulePtr(Arc<Module>) 取堆分配写句柄，
  // 与 C++ 经 shared_ptr<Module> 直接改写 *internalTypes 同语义；分析单线程串行，
  // arena 借用存续期间无第二个 Module 内部视图存活，&mut 重建无别名冲突。
  let arena = unsafe { &mut (*(arc_as_mut(module))).internal_types };

  let pattern_index = if expr.self_ { 0 } else { 1 };
  let pattern = expr
    .args
    .get(pattern_index)
    .and_then(|&arg| unsafe { ast_node_try_as_ptr::<AstExprConstantString>(arg) })?;

  let return_types = parse_pattern_string_bytes(
    typechecker.builtin_types.as_nonnull(),
    pattern.value.as_bytes(),
  );

  if return_types.is_empty() {
    return None;
  }

  let first_location = unsafe { &(*expr.args[0]).base.location };
  typechecker.unify_type_id_type_id_scope_ptr_location(
    params[0],
    typechecker.string_type,
    scope,
    first_location,
  );

  let optional_number = arena.add_type(UnionType {
    options: vec![typechecker.nil_type, typechecker.number_type],
  });

  let init_index = if expr.self_ { 1 } else { 2 };
  if params.len() == 3
    && let Some(&init_arg) = expr.args.get(init_index)
  {
    let location = unsafe { &(*init_arg).base.location };
    typechecker.unify_type_id_type_id_scope_ptr_location(
      params[2],
      optional_number,
      scope,
      location,
    );
  }

  let return_list = arena.add_type_pack_t(TypePack::from_vec(return_types));
  Some(WithPredicate::with_predicate_t(return_list))
}
