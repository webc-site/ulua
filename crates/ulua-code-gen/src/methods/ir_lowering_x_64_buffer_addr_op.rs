use core::mem::size_of;

use ulua_vm::{
  enums::lua_type::LuaType,
  records::{luau_buffer::LuauBuffer, udata::Udata},
};

use crate::{
  enums::{ir_op_kind::IrOpKind, size_x_64::SizeX64},
  functions::{
    produces_dirty_high_register_bits::produces_dirty_high_register_bits, qword_reg::qword_reg,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

impl IrLoweringX64 {
  pub fn buffer_addr_op(&mut self, buffer_op: IrOp, index_op: IrOp, tag: u8) -> OperandX64 {
    CODEGEN_ASSERT!(tag == LuaType::UserData as u8 || tag == LuaType::Buffer as u8);
    let data_offset = if tag == LuaType::Buffer as u8 {
      size_of::<LuauBuffer>() - 1
    } else {
      core::mem::offset_of!(Udata, data)
    };

    if index_op.kind() == IrOpKind::Inst {
      let inst_op = unsafe { (*self.function).inst_op(index_op) };
      CODEGEN_ASSERT!(!produces_dirty_high_register_bits(inst_op.cmd));

      let buffer_reg = self.reg_op(buffer_op);
      let index_reg = self.reg_op(index_op);
      let scaled_index = qword_reg(index_reg);
      return OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::Qword,
        scaled_index,
        1,
        buffer_reg,
        data_offset as i32,
      );
    } else if index_op.kind() == IrOpKind::Constant {
      let buffer_reg = self.reg_op(buffer_op);
      let index_val = self.int_op(index_op);
      return OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::Qword,
        RegisterX64::NOREG,
        1,
        buffer_reg,
        index_val + data_offset as i32,
      );
    }

    CODEGEN_ASSERT!(false, "Unsupported instruction form");
    OperandX64::operand_x_64_register_x_64(RegisterX64::NOREG)
  }
}
