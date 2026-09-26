use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{fast_is_subtype::fast_is_subtype, follow_type, get_type},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, free_type::FreeType,
    singleton_type::SingletonType, table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn extract_matching_table_type_deprecated(
  tables: &mut [TypeId],
  expr_type: TypeId,
  builtin_types: Handle<BuiltinTypes>,
) -> Option<TypeId> {
  LUAU_ASSERT!(!fflag::LuauBidirectionalInferenceBetterUnionHandling.get());
  if tables.is_empty() {
    return None;
  }

  let expr_table = get_type::get::<TableType>(follow_type::follow(expr_type))?;

  let mut table_count: usize = 0;
  let mut first_table: Option<TypeId> = None;

  for &ty in tables.iter() {
    let ty = follow_type::follow(ty);
    let tt = get_type::get::<TableType>(ty);
    if let Some(tt) = tt {
      // If the expected table has a key whose type is a string or boolean
      // singleton and the corresponding exprType property does not match,
      // then skip this table.

      first_table.get_or_insert(ty);
      table_count += 1;

      for (name, expected_prop) in &tt.props {
        let expected_read_ty = match expected_prop.read_ty {
          Some(t) => t,
          None => continue,
        };

        let expected_type = follow_type::follow(expected_read_ty);

        let st = get_type::get::<SingletonType>(expected_type);
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

        let prop_type = follow_type::follow(expr_read_ty);

        let ft = get_type::get::<FreeType>(prop_type);

        if let Some(ft) = ft
          && get_type::get::<SingletonType>(ft.lower_bound).is_some()
        {
          // SAFETY: builtin_types 由调用方契约保证有效（C++ 同款裸解引用）
          let (boolean_type, string_type) = (
            builtin_types.get().boolean_type,
            builtin_types.get().string_type,
          );
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
