use ulua_common::FFlag;

use crate::{
  records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
};

#[derive(Debug)]
pub struct TypeStateFixture {
  pub dcr: ScopedFastFlag,
  pub base: BuiltinsFixture,
}

impl Default for TypeStateFixture {
  fn default() -> Self {
    let dcr = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut base = BuiltinsFixture::default();
    base.get_frontend();

    Self { dcr, base }
  }
}
