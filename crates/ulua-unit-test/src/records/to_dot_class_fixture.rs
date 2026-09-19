//! Source: `tests/ToDot.test.cpp`

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
