#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ParenthesesRecommendation {
  #[default]
  None,
  CursorAfter,
  CursorInside,
}
