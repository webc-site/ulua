#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PropIndexType {
  Point,
  Colon,
  Key,
}
