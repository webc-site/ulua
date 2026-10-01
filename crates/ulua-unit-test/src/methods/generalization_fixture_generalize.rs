use ulua_analysis::{
  functions::generalize::generalize, records::arena_handle::Handle, type_aliases::type_id::TypeId,
};

use crate::{
  functions::raw_handle::raw_handle, records::generalization_fixture::GeneralizationFixture,
};

impl GeneralizationFixture {
  pub fn generalize(&mut self, ty: TypeId) -> Option<TypeId> {
    generalize(
      Handle::from_mut(&mut *self.arena),
      Handle::from_mut(&mut *self.builtin_types),
      unsafe { &*raw_handle(&self.scope) },
      &mut self.generalized_types,
      ty,
      None,
    )
  }
}
