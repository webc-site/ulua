use ulua_ast::records::{ast_name::AstName, ast_name_table::AstNameTable};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::enums::global::Global;

/// 把 `_G` 与宿主登记的 `mutable_globals` 名单标为 `Global::Mutable`。
#[inline]
pub(crate) fn assign_mutable<'a>(
  globals: &mut DenseHashMap<AstName, Global>,
  names: &AstNameTable,
  mutable_globals: impl IntoIterator<Item = &'a [u8]>,
) {
  let name = names.get_str("_G");
  if !name.is_null() {
    *globals.get_or_insert(name) = Global::Mutable;
  }

  for name_bytes in mutable_globals {
    let name = names.get_slice(name_bytes);
    if !name.is_null() {
      *globals.get_or_insert(name) = Global::Mutable;
    }
  }
}
