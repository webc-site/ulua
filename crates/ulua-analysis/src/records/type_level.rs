#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TypeLevel {
  pub(crate) level: i32,
  pub(crate) sub_level: i32,
}

impl TypeLevel {
  pub(crate) const fn new(level: i32, sub_level: i32) -> Self {
    Self { level, sub_level }
  }
}

pub const TYPE_LEVEL_DEFAULT: TypeLevel = TypeLevel {
  level: 0,
  sub_level: 0,
};
