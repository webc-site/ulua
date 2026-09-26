//! Source: `tests/FragmentAutocomplete.test.cpp`

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
