#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Relation {
  Disjoint,
  Coincident,
  Intersects,
  Subset,
  Superset,
}
