use crate::records::fenv_visitor::FenvVisitor;

impl<'a> FenvVisitor<'a> {
  pub fn new(getfenv_used: &'a mut bool, setfenv_used: &'a mut bool) -> Self {
    FenvVisitor {
      getfenv_used,
      setfenv_used,
    }
  }
}
