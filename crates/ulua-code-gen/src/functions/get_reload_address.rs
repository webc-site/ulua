use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::{ir_op_kind::IrOpKind, kind_a_64::KindA64},
  functions::{
    get_reload_offset::get_reload_offset, vm_const_op::vm_const_op, vm_reg_op::vm_reg_op,
  },
  records::{
    address_a_64::AddressA64,
    register_a_64::{RegisterA64, reg},
    value_restore_location::ValueRestoreLocation,
  },
  type_aliases::mem::mem,
};

const R_BASE: RegisterA64 = reg(KindA64::X, 25);
const R_CONSTANTS: RegisterA64 = reg(KindA64::X, 22);
const XZR: RegisterA64 = reg(KindA64::X, 31);

pub fn get_reload_address(location: ValueRestoreLocation) -> AddressA64 {
  let op = location.op;

  if op.kind() == IrOpKind::VmReg {
    let offset = vm_reg_op(op) * size_of::<TValue>() as i32 + get_reload_offset(location.kind);
    return mem(R_BASE, offset);
  }

  // load 宽度为 4/8/16 字节；保守起见按 4 字节索引限制偏移量
  if op.kind() == IrOpKind::VmConst
    && (vm_const_op(op) as usize * size_of::<TValue>() <= AddressA64::K_MAX_OFFSET * 4)
  {
    let offset = vm_const_op(op) * size_of::<TValue>() as i32 + get_reload_offset(location.kind);
    return mem(R_CONSTANTS, offset);
  }

  AddressA64::address_a_64_register_a_64_register_a_64(XZR, XZR)
}
