use ulua_code_gen::enums::host_metamethod::HostMetamethod;

use crate::common::{
  enums::userdata_kind::UserdataKind, functions::userdata_index_to_type::userdata_index_to_type,
  records::vec_2_conformance_ir_hooks::Vec2,
};

const LBC_TYPE_ANY: u8 = 15;

pub fn userdata_metamethod_bytecode_type(lhs_ty: u8, rhs_ty: u8, method: HostMetamethod) -> u8 {
  match method {
    HostMetamethod::Add | HostMetamethod::Sub | HostMetamethod::Mul | HostMetamethod::Div => {
      if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2)
        || UserdataKind::from_type(rhs_ty) == Some(UserdataKind::Vec2)
      {
        userdata_index_to_type(Vec2::USERDATA_INDEX)
      } else {
        LBC_TYPE_ANY
      }
    }
    HostMetamethod::Minus if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2) => {
      userdata_index_to_type(Vec2::USERDATA_INDEX)
    }
    _ => LBC_TYPE_ANY,
  }
}
