#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Checkpoint {
  pub(crate) offset: usize,
}
