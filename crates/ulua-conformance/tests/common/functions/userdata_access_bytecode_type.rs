use crate::common::{
  enums::userdata_kind::UserdataKind,
  functions::userdata_index_to_type::userdata_index_to_type,
  records::{
    bytecode_types::{LBC_TYPE_ANY, LBC_TYPE_NUMBER, LBC_TYPE_VECTOR},
    vec_2_conformance_ir_hooks::Vec2,
  },
};

pub fn userdata_access_bytecode_type(r#type: u8, member: &str) -> u8 {
  match UserdataKind::from_type(r#type) {
    Some(UserdataKind::Color) => match member {
      "R" | "G" | "B" => LBC_TYPE_NUMBER,
      _ => LBC_TYPE_ANY,
    },
    Some(UserdataKind::Vec2) => match member {
      "X" | "Y" | "Magnitude" => LBC_TYPE_NUMBER,
      "Unit" => userdata_index_to_type(Vec2::USERDATA_INDEX),
      _ => LBC_TYPE_ANY,
    },
    Some(UserdataKind::Mat3) => match member {
      "Row1" | "Row2" | "Row3" => LBC_TYPE_VECTOR,
      _ => LBC_TYPE_ANY,
    },
    Some(UserdataKind::Vertex) => match member {
      "pos" | "normal" => LBC_TYPE_VECTOR,
      "uv" => userdata_index_to_type(Vec2::USERDATA_INDEX),
      _ => LBC_TYPE_ANY,
    },
    _ => LBC_TYPE_ANY,
  }
}
