use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::{
    address_kind_a_64::AddressKindA64, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind,
    kind_a_64::KindA64,
  },
  functions::{
    emit_add_offset::emit_add_offset, get_cmd_value_kind::get_cmd_value_kind,
    vm_const_op::vm_const_op, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    address_a_64::AddressA64, ir_lowering_a_64::IrLoweringA64, ir_op::IrOp,
    register_a_64::RegisterA64,
  },
};

const K_MAX_IMMEDIATE: usize = 4095;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << 3),
  }
}

const R_BASE: RegisterA64 = reg(KindA64::X, 25);
const R_CONSTANTS: RegisterA64 = reg(KindA64::X, 22);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
  AddressA64 {
    kind: AddressKindA64::Imm,
    base,
    offset: RegisterA64::NOREG,
    data,
  }
}

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_temp_addr(
    &mut self,
    op: IrOp,
    offset: i32,
    temp_storage: RegisterA64,
  ) -> AddressA64 {
    CODEGEN_ASSERT!(offset % 4 == 0);
    CODEGEN_ASSERT!(offset >= 0 && (offset as usize / 4) <= K_MAX_IMMEDIATE);

    if op.kind() == IrOpKind::VmReg {
      return mem(R_BASE, vm_reg_op(op) * size_of::<TValue>() as i32 + offset);
    } else if op.kind() == IrOpKind::VmConst {
      let constant_offset = vm_const_op(op) as usize * size_of::<TValue>() + offset as usize;

      if constant_offset / 4 <= AddressA64::K_MAX_OFFSET {
        return mem(R_CONSTANTS, constant_offset as i32);
      }

      let temp = if temp_storage == RegisterA64::NOREG {
        self.regs.alloc_temp(KindA64::X)
      } else {
        temp_storage
      };
      CODEGEN_ASSERT!(
        temp.kind() == KindA64::X,
        "temp storage, when provided, must be an 'x' register"
      );

      unsafe {
        emit_add_offset(&mut *self.build, temp, R_CONSTANTS, constant_offset);
      }
      return mem(temp, 0);
    } else if op.kind() == IrOpKind::Inst {
      unsafe {
        CODEGEN_ASSERT!(
          get_cmd_value_kind((*self.function).inst_op(op).cmd) == IrValueKind::Pointer
        );
      }
      return mem(self.ir_lowering_a_64_reg_op(op), offset);
    }

    CODEGEN_ASSERT!(false, "Unsupported instruction form");
    mem(RegisterA64::NOREG, 0)
  }
}
