use alloc::{collections::BTreeSet, string::String, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type::follow_type_id,
    get_type_alt_j::get_type_id, is_string::is_string,
  },
  records::{
    extern_type::ExternType, metatable_type::MetatableType, primitive_type::PrimitiveType,
    table_type::TableType, type_function_context::TypeFunctionContext,
  },
  type_aliases::type_id::TypeId,
};
pub fn compute_keys_of(
  ty: TypeId,
  result: &mut BTreeSet<String>,
  seen: &mut DenseHashSet<TypeId>,
  is_raw: bool,
  ctx: &TypeFunctionContext,
) -> bool {
  let ty = follow_type_id(ty);

  if get_type_id::<PrimitiveType>(ty).is_some() {
    return false;
  }

  if seen.contains(&ty) {
    return true;
  }
  seen.insert(ty);

  if let Some(table_ty) = get_type_id::<TableType>(ty) {
    if let Some(indexer) = table_ty.indexer
      && is_string(indexer.index_type)
    {
      return false;
    }

    for key in table_ty.props.keys() {
      result.insert(key.clone());
    }
    return true;
  }

  if let Some(metatable_ty) = get_type_id::<MetatableType>(ty) {
    let mut res = true;

    if !is_raw {
      let mut dummy = Vec::new();
      if let Some(mm_type) = find_metatable_entry(
        ctx.builtins.as_ptr(),
        &mut dummy,
        ty,
        "__index",
        Location::default(),
      ) {
        res = res && compute_keys_of(mm_type, result, seen, is_raw, ctx);
      }
    }

    res = res && compute_keys_of(metatable_ty.table(), result, seen, is_raw, ctx);
    return res;
  }

  if let Some(extern_ty) = get_type_id::<ExternType>(ty) {
    for key in extern_ty.props.keys() {
      result.insert(key.clone());
    }

    let mut res = true;
    if extern_ty.metatable.is_some() && !is_raw {
      let mut dummy = Vec::new();
      if let Some(mm_type) = find_metatable_entry(
        ctx.builtins.as_ptr(),
        &mut dummy,
        ty,
        "__index",
        Location::default(),
      ) {
        res = res && compute_keys_of(mm_type, result, seen, is_raw, ctx);
      }
    }

    if let Some(parent) = extern_ty.parent {
      res = res && compute_keys_of(parent, result, seen, is_raw, ctx);
    }

    return res;
  }

  LUAU_ASSERT!(false);
  false
}
