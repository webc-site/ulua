use crate::{
  enums::{ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  functions::{
    get_cmd_value_kind::get_cmd_value_kind, luau_constant::luau_constant,
    luau_constant_tag::luau_constant_tag, luau_constant_value::luau_constant_value,
    luau_reg::luau_reg, luau_reg_tag::luau_reg_tag, luau_reg_value::luau_reg_value,
    luau_reg_value_int::luau_reg_value_int, luau_reg_value_int_64::luau_reg_value_int_64,
    vm_const_op::vm_const_op, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_inst::IrInst, ir_reg_alloc_x_64::IrRegAllocX64, operand_x_64::OperandX64,
    value_restore_location::ValueRestoreLocation,
  },
};

impl IrRegAllocX64 {
  pub fn get_restore_address(
    &self,
    inst: &IrInst,
    restore_location: ValueRestoreLocation,
  ) -> OperandX64 {
    let op = restore_location.op;
    CODEGEN_ASSERT!(op.kind() != IrOpKind::None);

    let _inst_kind = get_cmd_value_kind(inst.cmd);

    match restore_location.kind {
      IrValueKind::Unknown
      | IrValueKind::None
      | IrValueKind::Float
      | IrValueKind::Count
      | IrValueKind::Int64 => {
        if op.kind() == IrOpKind::VmReg {
          luau_reg_value_int_64(vm_reg_op(op))
        } else {
          luau_constant_value(vm_const_op(op))
        }
      }
      IrValueKind::Tag => {
        if op.kind() == IrOpKind::VmReg {
          luau_reg_tag(vm_reg_op(op))
        } else {
          luau_constant_tag(vm_const_op(op))
        }
      }
      IrValueKind::Int => {
        CODEGEN_ASSERT!(op.kind() == IrOpKind::VmReg);
        luau_reg_value_int(vm_reg_op(op))
      }
      IrValueKind::Pointer | IrValueKind::Double => {
        if op.kind() == IrOpKind::VmReg {
          luau_reg_value(vm_reg_op(op))
        } else {
          luau_constant_value(vm_const_op(op))
        }
      }
      IrValueKind::Tvalue => {
        if op.kind() == IrOpKind::VmReg {
          luau_reg(vm_reg_op(op))
        } else {
          luau_constant(vm_const_op(op))
        }
      }
    }
  }
}
