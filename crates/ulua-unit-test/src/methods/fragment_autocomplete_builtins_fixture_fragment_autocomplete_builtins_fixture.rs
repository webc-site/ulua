use crate::records::{
  fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
  fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};

impl FragmentAutocompleteBuiltinsFixture {
  pub fn new() -> Self {
    let mut fixture = Self {
      base: FragmentAutocompleteFixtureImpl::new(),
    };
    // C++ reaches the frontend through the virtual `getFrontend()` override, which
    // lazily loads the `FakeVec` class definition and the `game` global into both the
    // globals and the for-autocomplete globals. The Rust port has no virtual dispatch,
    // so prime the override once at construction to make those definitions available
    // to the (non-virtual) `self.base.get_frontend()` calls the fixture methods use.
    fixture.get_frontend();
    fixture
  }
}
