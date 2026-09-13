use core::convert::Infallible;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NotBindable {
  pub(crate) _unused: Option<Infallible>,
}
