use alloc::sync::Arc;

use ulua_analysis::{
  records::{scope::Scope, subtyping_result::SubtypingResult},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn is_subtype_type_pack_id_type_pack_id(
    &mut self,
    sub_ty: TypePackId,
    super_ty: TypePackId,
  ) -> SubtypingResult {
    let scope = Arc::as_ptr(&self.root_scope) as *mut Scope;
    let bindable_generics: Vec<TypeId> = Vec::new();
    self
      .subtyping
      .is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
        sub_ty,
        super_ty,
        scope,
        &bindable_generics,
      )
  }
}
