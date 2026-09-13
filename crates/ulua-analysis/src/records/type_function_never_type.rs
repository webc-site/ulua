use core::convert::Infallible;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TypeFunctionNeverType {
  pub(crate) _unused: Option<Infallible>,
}
