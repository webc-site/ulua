#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Token {
  pub(crate) name: &'static str,
  pub(crate) category: &'static str,
}
