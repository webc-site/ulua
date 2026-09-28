#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct Jump {
  pub(crate) source: u32,
  pub(crate) target: u32,
}
