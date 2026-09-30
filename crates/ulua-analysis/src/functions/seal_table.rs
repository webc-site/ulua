use crate::{
  enums::table_state::TableState,
  functions::{follow_type, get_mutable_type, subsumes_scope::subsumes},
  records::{arena_handle::alias_opt, scope::Scope, table_type::TableType},
  type_aliases::type_id::TypeId,
};

/// cpp `sealTable(NotNull<Scope> scope, TypeId ty)`（Generalization.cpp:1394）。
pub fn seal_table(scope: &Scope, ty: TypeId) {
  let follow_ty = follow_type::follow(ty);
  let table_ty = get_mutable_type::get_mutable::<TableType>(follow_ty);

  let Some(table_ty) = table_ty else {
    return;
  };

  // `table_ty.scope` 是可空 `Scope*` 字段（nullptr 视为最外层作用域），经 `alias_opt` 映射。
  if !subsumes(Some(scope), alias_opt(table_ty.scope)) {
    return;
  }

  if matches!(table_ty.state, TableState::Unsealed | TableState::Free) {
    table_ty.state = TableState::Sealed;
  }
}
