#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RefinementsOpKind {
  Intersect,
  Refine,
  #[default]
  None,
}
