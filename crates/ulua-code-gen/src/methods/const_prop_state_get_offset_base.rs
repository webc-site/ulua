use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::{op_a::op_a, op_b::op_b, opt_op_a::opt_op_a, opt_op_b::opt_op_b},
  records::{buffer_access_base::BufferAccessBase, const_prop_state::ConstPropState, ir_op::IrOp},
};

impl ConstPropState {
  pub fn get_offset_base(&mut self, value: IrOp) -> BufferAccessBase {
    let mut base = BufferAccessBase {
      op: value,
      scale: 1,
      offset: 0,
    };

    loop {
      if base.op.kind() != IrOpKind::Inst {
        break;
      }

      let inst = unsafe { (*self.function).inst_op(base.op).clone() };
      let lhs_num = unsafe { (*self.function).as_double_op(opt_op_a(inst.clone())) };
      let rhs_num = unsafe { (*self.function).as_double_op(opt_op_b(inst.clone())) };
      let lhs_int = unsafe { (*self.function).as_int_op(opt_op_a(inst.clone())) };
      let rhs_int = unsafe { (*self.function).as_int_op(opt_op_b(inst.clone())) };

      if inst.cmd == IrCmd::AddNum && lhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.offset += (lhs_num.unwrap() as i32) * base.scale;
        base.op = op_b(inst);
      } else if inst.cmd == IrCmd::AddNum
        && rhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.offset += (rhs_num.unwrap() as i32) * base.scale;
        base.op = op_a(&mut inst.clone());
      } else if inst.cmd == IrCmd::SubNum
        && rhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.offset -= (rhs_num.unwrap() as i32) * base.scale;
        base.op = op_a(&mut inst.clone());
      } else if inst.cmd == IrCmd::MulNum
        && lhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.scale *= lhs_num.unwrap() as i32;
        base.op = op_b(inst);
      } else if inst.cmd == IrCmd::MulNum
        && rhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.scale *= rhs_num.unwrap() as i32;
        base.op = op_a(&mut inst.clone());
      } else if inst.cmd == IrCmd::AddInt
        && lhs_int.is_some_and(|n| self.is_valid_integer_for_immediate(n))
      {
        base.offset += lhs_int.unwrap() * base.scale;
        base.op = op_b(inst);
      } else if inst.cmd == IrCmd::AddInt
        && rhs_int.is_some_and(|n| self.is_valid_integer_for_immediate(n))
      {
        base.offset += rhs_int.unwrap() * base.scale;
        base.op = op_a(&mut inst.clone());
      } else if inst.cmd == IrCmd::SubInt
        && rhs_int.is_some_and(|n| self.is_valid_integer_for_immediate(n))
      {
        base.offset -= rhs_int.unwrap() * base.scale;
        base.op = op_a(&mut inst.clone());
      } else if inst.cmd == IrCmd::TruncateUint {
        base.op = op_a(&mut inst.clone());
      } else {
        break;
      }

      if !self.is_valid_integer_for_immediate(base.offset)
        || !self.is_valid_integer_for_immediate(base.scale)
      {
        break;
      }
    }

    base
  }
}
