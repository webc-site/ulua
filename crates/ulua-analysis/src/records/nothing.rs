use core::convert::Infallible;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Nothing {
  pub(crate) _unused: Option<Infallible>,
}
