#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum PathType {
  RelativeToCurrent,
  RelativeToParent,
  Aliased,
  Unsupported,
}
