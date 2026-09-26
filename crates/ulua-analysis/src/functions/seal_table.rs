use crate::{
  enums::table_state::TableState,
  functions::{follow_type, get_mutable_type, subsumes_scope::subsumes},
  records::{scope::Scope, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn seal_table(scope: *mut Scope, ty: TypeId) {
  let follow_ty = follow_type::follow(ty);
  let table_ty = get_mutable_type::get_mutable::<TableType>(follow_ty);

  let Some(table_ty) = table_ty else {
    return;
  };

  if !subsumes(scope, table_ty.scope) {
    return;
  }

  if matches!(table_ty.state, TableState::Unsealed | TableState::Free) {
    table_ty.state = TableState::Sealed;
  }
}
