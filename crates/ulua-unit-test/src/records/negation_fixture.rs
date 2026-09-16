use ulua_analysis::records::type_arena::TypeArena;

use crate::records::fixture::Fixture;

#[derive(Debug)]
pub struct NegationFixture {
  pub base: Fixture,
  pub arena: TypeArena,
}

impl NegationFixture {
  pub fn negation_fixture(&mut self) {
    self.base.register_test_types();
  }
}

impl Default for NegationFixture {
  fn default() -> Self {
    let mut fixture = Self {
      base: Fixture::fixture_bool(false),
      arena: TypeArena::default(),
    };
    fixture.negation_fixture_negation_fixture();
    fixture
  }
}
