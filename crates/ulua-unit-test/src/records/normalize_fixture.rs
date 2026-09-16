//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Normalize.test.cpp:429:normalize_fixture`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//!   - type_ref <- method NormalizeFixture::NormalizeFixture (tests/Normalize.test.cpp)
//!   - type_ref <- method NormalizeFixture::toNormalizedType (tests/Normalize.test.cpp)
//!   - type_ref <- method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - type_ref <- method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - type_ref <- method NormalizeFixture::isInhabited (tests/Normalize.test.cpp)
//!   - type_ref <- method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - type_ref <- method NormalizeFixture::getFrontend (tests/Normalize.test.cpp)
//!   - type_ref <- method NormalizeFixture::getGlobalScope (tests/Normalize.test.cpp)
//! - outgoing:
//!   - type_ref -> method NormalizeFixture::NormalizeFixture (tests/Normalize.test.cpp)
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
//!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
//!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
//!   - translates_to -> rust_item NormalizeFixture

use alloc::{boxed::Box, sync::Arc};

use ulua_analysis::records::{
  internal_error_reporter::InternalErrorReporter, normalizer::Normalizer, scope::Scope,
  type_arena::TypeArena, unifier_shared_state::UnifierSharedState,
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
    let unifier_state = UnifierSharedState::new(&mut *ice_handler as *mut _);

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
