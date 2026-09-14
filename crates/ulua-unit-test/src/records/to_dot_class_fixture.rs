//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/ToDot.test.cpp:14:to_dot_class_fixture`
//! Source: `tests/ToDot.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ToDot.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToDot.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/ToDot.test.cpp
//!   - type_ref <- method ToDotClassFixture::ToDotClassFixture (tests/ToDot.test.cpp)
//! - outgoing:
//!   - type_ref -> method ToDotClassFixture::ToDotClassFixture (tests/ToDot.test.cpp)
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - translates_to -> rust_item ToDotClassFixture

use crate::records::fixture::Fixture;

#[derive(Debug)]
pub struct ToDotClassFixture {
  pub base: Fixture,
}

impl Default for ToDotClassFixture {
  fn default() -> Self {
    let mut fixture = Self {
      base: Fixture::default(),
    };
    fixture.to_dot_class_fixture();
    fixture
  }
}
