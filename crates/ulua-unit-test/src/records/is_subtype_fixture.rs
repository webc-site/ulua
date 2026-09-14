//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Fixture.h:229:is_subtype_fixture`
//! Source: `tests/Fixture.h`
//! Graph edges:
//! - declared_by: source_file tests/Fixture.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinTypeFunctions.h
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Ast/include/Luau/Location.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file tests/IostreamOptional.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Fixture.h
//!   - type_ref <- method IsSubtypeFixture::isSubtype (tests/Fixture.cpp)
//! - outgoing:
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item IsSubtypeFixture

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
