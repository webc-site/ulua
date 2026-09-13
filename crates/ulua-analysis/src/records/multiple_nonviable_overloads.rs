#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MultipleNonviableOverloads {
  pub(crate) attempted_arg_count: usize,
}
