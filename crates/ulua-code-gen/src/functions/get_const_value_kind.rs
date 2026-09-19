use crate::{
  enums::ir_value_kind::IrValueKind,
  records::ir_const::IrConst,
};

pub fn get_const_value_kind(constant: &IrConst) -> IrValueKind {
  match constant {
    IrConst::Int(_) | IrConst::Uint(_) => IrValueKind::Int,
    IrConst::Int64(_) => IrValueKind::Int64,
    IrConst::Double(_) => IrValueKind::Double,
    IrConst::Tag(_) => IrValueKind::Tag,
    IrConst::Import(_) => {
      debug_assert!(false, "Import constants cannot be used as IR values");
      IrValueKind::Unknown
    }
  }
}
