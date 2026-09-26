//! Source: `tests/Fixture.h`

use crate::records::fixture::Fixture;
#[derive(Debug)]
pub struct IsSubtypeFixture {
  pub base: Fixture,
}

impl Default for IsSubtypeFixture {
  fn default() -> Self {
    Self {
      base: Fixture::fixture_bool(false),
    }
  }
}
