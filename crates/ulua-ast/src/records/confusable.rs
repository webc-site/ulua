#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Confusable {
  pub(crate) codepoint: u32,
  pub(crate) text: [u8; 5],
}
