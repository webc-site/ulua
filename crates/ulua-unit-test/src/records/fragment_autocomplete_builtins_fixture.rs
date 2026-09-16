use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

#[derive(Debug)]
pub struct FragmentAutocompleteBuiltinsFixture {
  pub base: FragmentAutocompleteFixtureImpl,
}

impl Default for FragmentAutocompleteBuiltinsFixture {
  fn default() -> Self {
    Self::new()
  }
}
