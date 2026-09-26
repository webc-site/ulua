use ulua_analysis::{records::subtyping_result::SubtypingResult, type_aliases::type_id::TypeId};

use crate::{functions::raw_handle::raw_handle, records::subtype_fixture::SubtypeFixture};

impl SubtypeFixture {
  pub fn is_subtype_type_id_type_id(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
  ) -> SubtypingResult {
    let scope = raw_handle(&self.root_scope);
    self
      .subtyping
      .is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope)
  }
}
