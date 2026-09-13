#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum Variant {
  #[default]
  Pack,
  Union,
  Intersection,
}
