//! Source: `tests/Normalize.test.cpp`

use alloc::{boxed::Box, sync::Arc};

use ulua_analysis::records::{
  arena_handle::Handle, internal_error_reporter::InternalErrorReporter, normalizer::Normalizer,
  scope::Scope, type_arena::TypeArena, unifier_shared_state::UnifierSharedState,
};

use crate::records::fixture::Fixture;

#[derive(Debug)]
pub struct NormalizeFixture {
  pub global_scope: Option<Arc<Scope>>,
  pub normalizer: Option<Normalizer>,
  pub unifier_state: UnifierSharedState,
  pub ice_handler: Box<InternalErrorReporter>,
  pub arena: TypeArena,
  pub base: Fixture,
}

impl Default for NormalizeFixture {
  fn default() -> Self {
    let mut ice_handler = Box::new(InternalErrorReporter::default());
    let unifier_state = UnifierSharedState::new(Handle::from_mut(&mut *ice_handler));

    Self {
      global_scope: None,
      normalizer: None,
      unifier_state,
      ice_handler,
      arena: TypeArena::default(),
      base: Fixture::fixture_bool(false),
    }
  }
}
