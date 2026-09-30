use ulua_analysis::records::type_arena::TypeArena;

use crate::{functions::register_hidden_types::register_hidden_types, records::fixture::Fixture};

#[derive(Debug)]
pub struct NegationFixture {
  pub base: Fixture,
  pub arena: TypeArena,
}

impl Default for NegationFixture {
  fn default() -> Self {
    let mut fixture = Self {
      base: Fixture::fixture_bool(false),
      arena: TypeArena::default(),
    };
    register_hidden_types(fixture.base.get_frontend());
    fixture
  }
}
