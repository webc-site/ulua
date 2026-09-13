use crate::records::ac_fixture_impl::AcFixtureImpl;

#[derive(Debug)]
pub struct AcExternTypeFixture {
  pub base: AcFixtureImpl,
}

impl Default for AcExternTypeFixture {
  fn default() -> Self {
    let base = AcFixtureImpl {
      register_builtins: true,
      ..Default::default()
    };
    Self { base }
  }
}
