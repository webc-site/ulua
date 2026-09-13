use crate::common::{
  enums::userdata_kind::UserdataKind, functions::userdata_index_to_type::userdata_index_to_type,
  records::vec_2_conformance_ir_hooks::Vec2,
};

const LBC_TYPE_ANY: u8 = 0;
const LBC_TYPE_NUMBER: u8 = 2;

pub fn userdata_namecall_bytecode_type(type_: u8, member: &str) -> u8 {
  match UserdataKind::from_type(type_) {
    Some(UserdataKind::Vec2) => match member {
      "Dot" => LBC_TYPE_NUMBER,
      "Min" => userdata_index_to_type(Vec2::USERDATA_INDEX),
      _ => LBC_TYPE_ANY,
    },
    _ => LBC_TYPE_ANY,
  }
}
