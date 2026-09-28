//! C++ `const Position& FragmentAutocompleteFixtureImpl::getPosition(char marker) const`
//! (tests/FragmentAutocomplete.test.cpp:291-296).
use ulua_ast::records::position::Position;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixtureImpl {
  pub fn get_position(&self, marker: char) -> Position {
    let found = self.marker_position.get(&marker);
    LUAU_ASSERT!(found.is_some());
    *found.unwrap()
  }
}
