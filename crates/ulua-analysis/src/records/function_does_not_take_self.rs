use core::convert::Infallible;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FunctionDoesNotTakeSelf {
  pub(crate) _unused: Option<Infallible>,
}
