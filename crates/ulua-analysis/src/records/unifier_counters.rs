#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct UnifierCounters {
  pub(crate) recursion_count: i32,
  pub(crate) recursion_limit: i32,
  pub(crate) iteration_count: i32,
  pub(crate) iteration_limit: i32,
}
