use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{require_variadic_sequence::require_variadic_sequence, vm_reg_op::vm_reg_op},
  records::{ir_op::IrOp, register_set::RegisterSet},
};

#[derive(Debug)]
pub struct BlockVmRegLiveInComputation<'a> {
  pub(crate) def_rs: &'a mut RegisterSet,
  pub(crate) captured_regs: &'a mut [u64; 4],
  pub(crate) in_rs: RegisterSet,
}

impl<'a> BlockVmRegLiveInComputation<'a> {
  pub fn new(def_rs: &'a mut RegisterSet, captured_regs: &'a mut [u64; 4]) -> Self {
    Self {
      def_rs,
      captured_regs,
      in_rs: RegisterSet {
        regs: [0; 4],
        vararg_seq: false,
        vararg_start: 0,
      },
    }
  }

  pub fn def(&mut self, op: IrOp, offset: i32) {
    let reg = (vm_reg_op(op) + offset) as usize;
    self.def_rs.regs[reg / 64] |= 1 << (reg % 64);
  }

  pub fn r#use(&mut self, op: IrOp, offset: i32) {
    let reg = (vm_reg_op(op) + offset) as usize;
    if (self.def_rs.regs[reg / 64] & (1 << (reg % 64))) == 0 {
      self.in_rs.regs[reg / 64] |= 1 << (reg % 64);
    }
  }

  pub fn maybe_def(&mut self, op: IrOp) {
    if op.kind() == IrOpKind::VmReg {
      let reg = vm_reg_op(op) as usize;
      self.def_rs.regs[reg / 64] |= 1 << (reg % 64);
    }
  }

  pub fn maybe_use(&mut self, op: IrOp) {
    if op.kind() == IrOpKind::VmReg {
      let reg = vm_reg_op(op) as usize;
      if (self.def_rs.regs[reg / 64] & (1 << (reg % 64))) == 0 {
        self.in_rs.regs[reg / 64] |= 1 << (reg % 64);
      }
    }
  }

  pub fn def_varargs(&mut self, vararg_start: u8) {
    self.def_rs.vararg_seq = true;
    self.def_rs.vararg_start = vararg_start;
  }

  pub fn use_varargs(&mut self, vararg_start: u8) {
    require_variadic_sequence(&mut self.in_rs, &*self.def_rs, vararg_start);

    // Variadic sequence has been consumed
    self.def_rs.vararg_seq = false;
    self.def_rs.vararg_start = 0;
  }

  pub fn def_range(&mut self, start: i32, count: i32) {
    if count == -1 {
      self.def_varargs(start as u8);
    } else {
      for i in start..(start + count) {
        let reg = i as usize;
        self.def_rs.regs[reg / 64] |= 1 << (reg % 64);
      }
    }
  }

  pub fn use_range(&mut self, start: i32, count: i32) {
    if count == -1 {
      self.use_varargs(start as u8);
    } else {
      for i in start..(start + count) {
        let reg = i as usize;
        if (self.def_rs.regs[reg / 64] & (1 << (reg % 64))) == 0 {
          self.in_rs.regs[reg / 64] |= 1 << (reg % 64);
        }
      }
    }
  }

  pub fn capture(&mut self, reg: i32) {
    let r = reg as usize;
    self.captured_regs[r / 64] |= 1 << (r % 64);
  }
}
