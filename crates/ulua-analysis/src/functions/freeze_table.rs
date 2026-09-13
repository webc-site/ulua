use core::ptr::null;

use ulua_common::{FFlag, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, shallow_clone_clone_alt_b::shallow_clone,
  },
  records::{
    clone_state::CloneState, magic_function_call_context::MagicFunctionCallContext,
    metatable_type::MetatableType, table_type::TableType, type_mismatch::TypeMismatch,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
pub fn freeze_table(input_type: TypeId, context: &MagicFunctionCallContext) -> Option<TypeId> {
  let solver = unsafe { context.solver.as_ref() };
  let arena = unsafe { &mut *solver.arena };
  let input_type = follow_type_id(input_type);

  if let Some(mt) = get_type_id::<MetatableType>(input_type) {
    let mt_table = mt.table;
    let frozen_table = freeze_table(mt_table, context)?;

    let result_type = unsafe { &mut *solver.arena }.add_type(MetatableType {
      table: frozen_table,
      metatable: mt.metatable,
      synthetic_name: mt.synthetic_name.clone(),
    });

    return Some(result_type);
  }

  if get_type_id::<TableType>(input_type).is_some() {
    // Clone the input type, this will become our final result type after we mutate it.
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
    let table_ty = get_mutable_type_id::<TableType>(result_type);
    // `clone` should not break this.
    ulua_common::macros::luau_assert::LUAU_ASSERT!(table_ty.is_some());
    let table_ty = table_ty.unwrap();
    table_ty.state = TableState::Sealed;

    // We'll mutate the table to make every property type read-only.
    table_ty.props.retain(|_name, prop| !prop.is_write_only());
    for prop in table_ty.props.values_mut() {
      prop.write_ty = None;
    }

    return Some(result_type);
  }

  if !FFlag::LuauTableFreezeCheckIsSubtype.get() {
    let call_site = unsafe { context.call_site.as_ref() };
    let table_type = unsafe { &*solver.builtin_types }.table_type;
    unsafe {
      (*context.solver.as_ptr()).report_error_type_error_data_location(
        TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given(table_type, input_type)),
        &call_site.arg_location,
      );
    }
  }
  None
}
