use std::collections::BTreeMap;

use crate::records::{
  builtins_fixture::BuiltinsFixture,
  fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};
impl FragmentAutocompleteFixtureImpl {
  pub fn new() -> Self {
    let mut base = BuiltinsFixture::default();
    base.builtins_fixture_builtins_fixture(true);
    FragmentAutocompleteFixtureImpl {
      base,
      marker_position: BTreeMap::new(),
    }
  }
}

impl Default for FragmentAutocompleteFixtureImpl {
  fn default() -> Self {
    Self::new()
  }
}
