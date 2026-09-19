use ulua_analysis::records::frontend::Frontend;

use crate::records::ac_fixture::AcFixture;

impl AcFixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    self.base.get_frontend()
  }
}
