//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:55:fragment_autocomplete_fixture_impl`
//! Source: `tests/FragmentAutocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
//!   - type_ref <- record FragmentAutocompleteFixture (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- record FragmentAutocompleteBuiltinsFixture (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::FragmentAutocompleteFixtureImpl (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::cleanMarkers (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::parseHelper_ (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::parseHelper (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::checkOldSolver (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::checkFragment (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::autocompleteFragment (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::autocompleteFragmentInNewSolver (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::autocompleteFragmentInOldSolver (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::autocompleteFragmentInBothSolvers (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::typecheckFragmentForModule (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::autocompleteFragmentForModule (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::getSource (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref <- method FragmentAutocompleteFixtureImpl::getPosition (tests/FragmentAutocomplete.test.cpp)
//! - outgoing:
//!   - type_ref -> method FragmentAutocompleteFixtureImpl::FragmentAutocompleteFixtureImpl (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
//!   - translates_to -> rust_item FragmentAutocompleteFixtureImpl

use std::collections::BTreeMap;

use ulua_ast::records::position::Position;

use crate::records::builtins_fixture::BuiltinsFixture;
#[derive(Debug)]
pub struct FragmentAutocompleteFixtureImpl {
  pub base: BuiltinsFixture,
  /// C++ `std::map<char, Position> markerPosition` — maps a marker character to
  /// the position it occupied in the source (computed by `cleanMarkers`).
  pub marker_position: BTreeMap<char, Position>,
}
