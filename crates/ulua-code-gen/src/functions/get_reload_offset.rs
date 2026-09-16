use ulua_common::macros::luau_unreachable::LUAU_UNREACHABLE;

use crate::enums::ir_value_kind::IrValueKind;

pub fn get_reload_offset(kind: IrValueKind) -> i32 {
  match kind {
    IrValueKind::Unknown | IrValueKind::None | IrValueKind::Float | IrValueKind::Count => {
      debug_assert!(false, "Invalid operand restore value kind");
    }
    IrValueKind::Tag => {
      return 8;
    }
    IrValueKind::Int | IrValueKind::Int64 | IrValueKind::Pointer | IrValueKind::Double => {
      return 0;
    }
    IrValueKind::Tvalue => {
      return 0;
    }
  }

  debug_assert!(false, "Invalid operand restore value kind");
  LUAU_UNREACHABLE!();
}
