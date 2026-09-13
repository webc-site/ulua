use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

#[derive(Debug)]
pub struct FragmentAutocompleteFixture {
  pub base: FragmentAutocompleteFixtureImpl,
}

impl Default for FragmentAutocompleteFixture {
  fn default() -> Self {
    Self::new()
  }
}
