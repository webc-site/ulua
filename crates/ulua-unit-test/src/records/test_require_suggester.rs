//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Fixture.h:69:test_require_suggester`
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
//!   - type_ref <- method TestFileResolver::TestFileResolver (tests/Fixture.h)
//!   - type_ref <- method TestRequireSuggester::getNode (tests/Fixture.cpp)
//!   - type_ref <- method TestRequireSuggester::TestRequireSuggester (tests/Fixture.h)
//! - outgoing:
//!   - type_ref -> method TestRequireSuggester::TestRequireSuggester (tests/Fixture.h)
//!   - type_ref -> record RequireSuggester (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record RequireNode (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record TestFileResolver (tests/Fixture.h)
//!   - translates_to -> rust_item TestRequireSuggester

#[derive(Debug, Clone)]
pub struct TestRequireSuggester {
  _private: (),
}
