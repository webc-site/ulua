use core::mem::size_of;

use ulua_vm::{
  enums::lua_type::LuaType,
  records::{luau_buffer::LuauBuffer, udata::Udata},
};

use crate::{
  enums::{address_kind_a_64::AddressKindA64, ir_op_kind::IrOpKind, kind_a_64::KindA64},
  functions::{
    emit_add_offset::emit_add_offset,
    produces_dirty_high_register_bits::produces_dirty_high_register_bits,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    address_a_64::AddressA64, ir_lowering_a_64::IrLoweringA64, ir_op::IrOp,
    register_a_64::RegisterA64,
  },
};

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
  AddressA64 {
    kind: AddressKindA64::Imm,
    base,
    offset: RegisterA64::NOREG,
    data,
  }
}

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_temp_addr_buffer(
    &mut self,
    buffer_op: IrOp,
    index_op: IrOp,
    tag: u8,
  ) -> AddressA64 {
    CODEGEN_ASSERT!(tag == LuaType::UserData as u8 || tag == LuaType::Buffer as u8);

    let data_offset = if tag == LuaType::Buffer as u8 {
      (size_of::<LuauBuffer>() - 1) as i32
    } else {
      core::mem::offset_of!(Udata, data) as i32
    };

    if index_op.kind() == IrOpKind::Inst {
      unsafe {
        CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
          (*self.function).inst_op(index_op).cmd
        ));
      }

      let temp = self.regs.alloc_temp(KindA64::X);
      let buffer = self.ir_lowering_a_64_reg_op(buffer_op);
      let index = self.ir_lowering_a_64_reg_op(index_op);
      unsafe {
        (*self.build).add_register_a_64_register_a_64_register_a_64_i32(temp, buffer, index, 0);
      }
      return mem(temp, data_offset);
    } else if index_op.kind() == IrOpKind::Constant {
      let buffer = self.ir_lowering_a_64_reg_op(buffer_op);
      let index = unsafe { (*self.function).int_op(index_op) };

      if (index as u32).wrapping_add(data_offset as u32) <= 255 {
        return mem(buffer, index + data_offset);
      }

      if index < 0 {
        return mem(buffer, data_offset);
      }

      let temp = self.regs.alloc_temp(KindA64::X);
      unsafe {
        emit_add_offset(&mut *self.build, temp, buffer, index as usize);
      }
      return mem(temp, data_offset);
    }

    CODEGEN_ASSERT!(false, "Unsupported instruction form");
    mem(RegisterA64::NOREG, 0)
  }
}
