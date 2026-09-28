#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InterestingEdgeCase {
  #[default]
  None,
  MetatableCall,
  Intersection,
}
