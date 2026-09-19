use crate::{
  enums::{ir_const_kind::IrConstKind, ir_value_kind::IrValueKind},
  records::ir_const::IrConst,
};

pub fn get_const_value_kind(constant: &IrConst) -> IrValueKind {
  match constant.kind {
    IrConstKind::Int => IrValueKind::Int,
    IrConstKind::Int64 => IrValueKind::Int64,
    IrConstKind::Uint => IrValueKind::Int,
    IrConstKind::Double => IrValueKind::Double,
    IrConstKind::Tag => IrValueKind::Tag,
    IrConstKind::Import => {
      debug_assert!(false, "Import constants cannot be used as IR values");
      IrValueKind::Unknown
    }
  }
}
