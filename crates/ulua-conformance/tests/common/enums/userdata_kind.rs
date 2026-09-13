use crate::common::functions::{
  type_to_userdata_index::type_to_userdata_index, userdata_index_to_type::userdata_index_to_type,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UserdataKind {
  Extra = 0,
  Color = 1,
  Vec2 = 2,
  Mat3 = 3,
  Vertex = 4,
}

impl UserdataKind {
  pub const EXTRA: u8 = Self::Extra as u8;
  pub const COLOR: u8 = Self::Color as u8;
  pub const VEC2: u8 = Self::Vec2 as u8;
  pub const MAT3: u8 = Self::Mat3 as u8;
  pub const VERTEX: u8 = Self::Vertex as u8;

  #[inline]
  pub fn from_type(ty: u8) -> Option<Self> {
    match type_to_userdata_index(ty) {
      0 => Some(Self::Extra),
      1 => Some(Self::Color),
      2 => Some(Self::Vec2),
      3 => Some(Self::Mat3),
      4 => Some(Self::Vertex),
      _ => None,
    }
  }

  #[inline]
  pub fn to_type(self) -> u8 {
    userdata_index_to_type(self as u8)
  }
}
