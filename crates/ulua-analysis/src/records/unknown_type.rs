use core::convert::Infallible;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct UnknownType {
  pub(crate) _unused: Option<Infallible>,
}
