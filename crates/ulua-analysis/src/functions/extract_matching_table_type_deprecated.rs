use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{
    fast_is_subtype::fast_is_subtype, follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    builtin_types::BuiltinTypes, free_type::FreeType, singleton_type::SingletonType,
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn extract_matching_table_type_deprecated(
  tables: &mut [TypeId],
  expr_type: TypeId,
  builtin_types: *mut BuiltinTypes,
) -> Option<TypeId> {
  LUAU_ASSERT!(!FFlag::LuauBidirectionalInferenceBetterUnionHandling.get());
  if tables.is_empty() {
    return None;
  }

  let expr_table = get_type_id::<TableType>(follow_type_id(expr_type))?;

  let mut table_count: usize = 0;
  let mut first_table: Option<TypeId> = None;

  for &ty in tables.iter() {
    let ty = follow_type_id(ty);
    let tt = get_type_id::<TableType>(ty);
    if let Some(tt) = tt {
      // If the expected table has a key whose type is a string or boolean
      // singleton and the corresponding exprType property does not match,
      // then skip this table.

      if first_table.is_none() {
        first_table = Some(ty);
      }
      table_count += 1;

      for (name, expected_prop) in &tt.props {
        let expected_read_ty = match expected_prop.read_ty {
          Some(t) => t,
          None => continue,
        };

        let expected_type = follow_type_id(expected_read_ty);

        let st = get_type_id::<SingletonType>(expected_type);
        if st.is_none() {
          continue;
        }

        let it = match expr_table.props.get(name) {
          Some(p) => p,
          None => continue,
        };

        let expr_prop = it;

        let expr_read_ty = match expr_prop.read_ty {
          Some(t) => t,
          None => continue,
        };

        let prop_type = follow_type_id(expr_read_ty);

        let ft = get_type_id::<FreeType>(prop_type);

        if let Some(ft) = ft
          && get_type_id::<SingletonType>(ft.lower_bound).is_some()
        {
          // SAFETY: builtin_types 由调用方契约保证有效（C++ 同款裸解引用）
          let (boolean_type, string_type) =
            unsafe { ((*builtin_types).boolean_type, (*builtin_types).string_type) };
          if fast_is_subtype(boolean_type, ft.upper_bound)
            && fast_is_subtype(expected_type, boolean_type)
          {
            return Some(ty);
          }

          if fast_is_subtype(string_type, ft.upper_bound)
            && fast_is_subtype(expected_type, ft.lower_bound)
          {
            return Some(ty);
          }
        }

        if fast_is_subtype(prop_type, expected_type) {
          return Some(ty);
        }
      }
    }
  }

  if table_count == 1 {
    LUAU_ASSERT!(first_table.is_some());
    return first_table;
  }

  None
}
