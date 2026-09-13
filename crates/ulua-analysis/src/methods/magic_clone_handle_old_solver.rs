// ({+ +}) -> {+ +}
// <T: {}>(T) -> T
use alloc::{sync::Arc, vec, vec::Vec};
use core::ptr::null;

use ulua_ast::records::ast_expr_call::AstExprCall;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    extend_type_pack::extend_type_pack, follow_type::follow_type_id, get_type_alt_j::get_type_id,
    shallow_clone_clone_alt_b::shallow_clone,
  },
  records::{
    clone_state::CloneState, count_mismatch::CountMismatch, intersection_type::IntersectionType,
    module::Module, scope::Scope, table_type::TableType, type_checker::TypeChecker,
    type_pack::TypePack, with_predicate::WithPredicate,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_id::TypePackId},
};
pub fn magic_clone_handle_old_solver(
  typechecker: &mut TypeChecker,
  _scope: &Arc<Scope>,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;

  let builtin_types = typechecker.builtin_types;
  let module = typechecker.current_module.as_ref()?;
  let arena = unsafe { &mut (*(Arc::as_ptr(module) as *mut Module)).internal_types };

  // in the old solver, nonstrict in particular is really bad about inferring `...any` for things that are definitely present
  // and the only real way for us to deal with this is to just be more permissive here
  let extended = unsafe { extend_type_pack(arena, builtin_types, param_pack, 1, Vec::new()) };
  let param_types = extended.head;
  if param_types.is_empty() || expr.args.size == 0 {
    typechecker.report_error_location_type_error_data(
      &expr.arg_location,
      TypeErrorData::CountMismatch(CountMismatch {
        expected: 1,
        actual: 0,
        ..Default::default()
      }),
    );
    return None;
  }

  let input_type = follow_type_id(param_types[0]);

  let table_ty = get_type_id::<TableType>(input_type);
  let intersection_ty = get_type_id::<IntersectionType>(input_type);
  if table_ty.is_none() && intersection_ty.is_none() {
    return None;
  }

  if let Some(intersection_ty) = intersection_ty {
    for &ty in intersection_ty.parts.iter() {
      get_type_id::<TableType>(ty)?;
    }
  }

  let mut clone_state = CloneState {
    builtin_types,
    seen_types: DenseHashMap::new(null()),
    seen_type_packs: DenseHashMap::new(null()),
  };
  let result_type = unsafe {
    shallow_clone(
      input_type,
      arena,
      &mut clone_state,
      /* clonePersistentTypes */ false,
    )
  };

  let cloned_type_pack = arena.add_type_pack_t(TypePack {
    head: vec![result_type],
    tail: None,
  });
  Some(WithPredicate::with_predicate_t(cloned_type_pack))
}
