#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LintOptions {
  pub warning_mask: u64,
}
