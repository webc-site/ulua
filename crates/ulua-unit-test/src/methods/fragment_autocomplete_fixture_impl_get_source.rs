use alloc::boxed::Box;

use ulua_analysis::records::source_module::SourceModule;

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
impl FragmentAutocompleteFixtureImpl {
  pub fn get_source(&mut self) -> &mut SourceModule {
    self.base.base.source_module = Some(Box::new(SourceModule::new()));
    self
      .base
      .base
      .source_module
      .as_deref_mut()
      .expect("fragment source module was just initialized")
  }
}
