use alloc::sync::Arc;

use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  functions::{first::first, flatten_type_pack::flatten_type_pack_id, get_type_alt_j::get_type_id},
  records::{
    module::Module, never_type::NeverType, type_checker::TypeChecker, type_pack::TypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
pub fn magic_assert_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  _expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;
  let predicates = with_predicate.predicates;
  let module = typechecker.current_module.as_ref()?;
  let arena = unsafe { &mut (*(Arc::as_ptr(module) as *mut Module)).internal_types };

  let (mut head, tail) = flatten_type_pack_id(param_pack);
  if head.is_empty()
    && let Some(t) = tail
  {
    let fst = first(t, false);
    if let Some(fst) = fst {
      head.push(fst);
    } else {
      return Some(WithPredicate::with_predicate_t_predicate_vec(
        param_pack, predicates,
      ));
    }
  }

  typechecker.resolve_predicate_vec_scope_ptr_bool(&predicates, scope, true);

  if !head.is_empty() {
    let (ty, _ok) = typechecker.pick_types_from_sense(head[0], true, unsafe {
      (*typechecker.builtin_types).nil_type
    });

    if let Some(ty) = ty {
      if !get_type_id::<NeverType>(ty).is_none() {
        head = vec![ty];
      } else {
        head[0] = ty;
      }
    }
  }

  let new_tp_id = arena.add_type_pack_t(TypePack { head, tail });
  Some(WithPredicate::with_predicate_t_predicate_vec(
    new_tp_id, predicates,
  ))
}
