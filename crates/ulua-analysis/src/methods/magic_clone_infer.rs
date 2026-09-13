use alloc::vec;
use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    as_mutable_type_pack_alt_d::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, shallow_clone_clone_alt_b::shallow_clone,
    track_interior_free_type::track_interior_free_type,
  },
  records::{
    clone_state::CloneState, count_mismatch::CountMismatch,
    magic_function_call_context::MagicFunctionCallContext, table_type::TableType,
    type_pack::TypePack,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_variant::TypePackVariant},
};
pub fn magic_clone_infer(context: &MagicFunctionCallContext) -> bool {
  let solver = unsafe { context.solver.as_ref() };
  let arena = unsafe { &mut *solver.arena };
  let call_site = unsafe { context.call_site.as_ref() };

  let (param_types, _param_tail) = flatten_type_pack_id(context.arguments);
  if param_types.is_empty() || call_site.args.size == 0 {
    unsafe {
      (*context.solver.as_ptr()).report_error_type_error_data_location(
        TypeErrorData::CountMismatch(CountMismatch {
          expected: 1,
          actual: 0,
          ..Default::default()
        }),
        &call_site.arg_location,
      );
    }
    return false;
  }

  let input_type = follow_type_id(param_types[0]);

  if get_type_id::<TableType>(input_type).is_none() {
    return false;
  }

  let mut clone_state = CloneState {
    builtin_types: solver.builtin_types,
    seen_types: DenseHashMap::new(null()),
    seen_type_packs: DenseHashMap::new(null()),
  };
  let result_type = unsafe {
    shallow_clone(
      input_type,
      arena,
      &mut clone_state,
      /* ignorePersistent */ true,
    )
  };

  let constraint_scope = unsafe { (*context.constraint.as_ptr()).scope };

  if let Some(table_type) = get_mutable_type_id::<TableType>(result_type) {
    table_type.scope = constraint_scope;
  }

  track_interior_free_type(constraint_scope, result_type);

  let cloned_type_pack = arena.add_type_pack_t(TypePack {
    head: vec![result_type],
    tail: None,
  });
  let result_mut = as_mutable_type_pack(context.result);
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(cloned_type_pack);
  }

  true
}
