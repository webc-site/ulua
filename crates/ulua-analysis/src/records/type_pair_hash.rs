use core::convert::Infallible;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TypePairHash {
  pub(crate) _unused: Option<Infallible>,
}

unsafe impl Send for TypePairHash {}
unsafe impl Sync for TypePairHash {}
