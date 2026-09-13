use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id, subsumes_scope::subsumes,
  },
  records::{scope::Scope, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn seal_table(scope: *mut Scope, ty: TypeId) {
  let follow_ty = follow_type_id(ty);
  let table_ty = get_mutable_type_id::<TableType>(follow_ty);

  let Some(table_ty) = table_ty else {
    return;
  };

  if !subsumes(scope, table_ty.scope) {
    return;
  }

  if table_ty.state == TableState::Unsealed || table_ty.state == TableState::Free {
    table_ty.state = TableState::Sealed;
  }
}
