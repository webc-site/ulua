use std::sync::Arc;

use ulua_analysis::{
  functions::generalize::generalize, records::scope::Scope, type_aliases::type_id::TypeId,
};

use crate::records::generalization_fixture::GeneralizationFixture;

impl GeneralizationFixture {
  pub fn generalize(&mut self, ty: TypeId) -> Option<TypeId> {
    let scope = Arc::as_ptr(&self.scope) as *mut Scope;
    generalize(
      &mut *self.arena,
      &mut *self.builtin_types,
      scope,
      &mut self.generalized_types,
      ty,
      None,
    )
  }
}
