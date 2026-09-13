use alloc::sync::Arc;

use ulua_analysis::{
  records::{scope::Scope, subtyping_result::SubtypingResult},
  type_aliases::type_id::TypeId,
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn is_subtype_type_id_type_id(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
  ) -> SubtypingResult {
    let scope = Arc::as_ptr(&self.root_scope) as *mut Scope;
    self
      .subtyping
      .is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope)
  }
}
