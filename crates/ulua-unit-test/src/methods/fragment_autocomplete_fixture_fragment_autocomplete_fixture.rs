use crate::records::{
  fragment_autocomplete_fixture::FragmentAutocompleteFixture,
  fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};

impl FragmentAutocompleteFixture {
  pub fn new() -> Self {
    Self {
      base: FragmentAutocompleteFixtureImpl::new(),
    }
  }
}
