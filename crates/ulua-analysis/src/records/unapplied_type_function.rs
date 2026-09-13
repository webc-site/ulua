use core::convert::Infallible;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct UnappliedTypeFunction {
  pub(crate) _unused: Option<Infallible>,
}
