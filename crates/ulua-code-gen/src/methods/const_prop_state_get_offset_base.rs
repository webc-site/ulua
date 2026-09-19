use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
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

      // 只快照 cmd 与两个操作数（均 Copy），避免整条 IrInst clone；
      // 快照结束即释放对 instructions 的可变借用，后续再借函数读常量。
      let (cmd, lhs_op, rhs_op) = unsafe {
        let inst = (*self.function).inst_op(base.op);
        let size = inst.ops.size();
        let lhs = if size > 0 {
          inst.ops[0]
        } else {
          IrOp::default()
        };
        let rhs = if size > 1 {
          inst.ops[1]
        } else {
          IrOp::default()
        };
        (inst.cmd, lhs, rhs)
      };

      let lhs_num = unsafe { (*self.function).as_double_op(lhs_op) };
      let rhs_num = unsafe { (*self.function).as_double_op(rhs_op) };
      let lhs_int = unsafe { (*self.function).as_int_op(lhs_op) };
      let rhs_int = unsafe { (*self.function).as_int_op(rhs_op) };

      if cmd == IrCmd::AddNum && lhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n)) {
        base.offset += (lhs_num.unwrap() as i32) * base.scale;
        base.op = rhs_op;
      } else if cmd == IrCmd::AddNum
        && rhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.offset += (rhs_num.unwrap() as i32) * base.scale;
        base.op = lhs_op;
      } else if cmd == IrCmd::SubNum
        && rhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.offset -= (rhs_num.unwrap() as i32) * base.scale;
        base.op = lhs_op;
      } else if cmd == IrCmd::MulNum
        && lhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.scale *= lhs_num.unwrap() as i32;
        base.op = rhs_op;
      } else if cmd == IrCmd::MulNum
        && rhs_num.is_some_and(|n| self.is_valid_double_for_immediate(n))
      {
        base.scale *= rhs_num.unwrap() as i32;
        base.op = lhs_op;
      } else if cmd == IrCmd::AddInt
        && lhs_int.is_some_and(|n| self.is_valid_integer_for_immediate(n))
      {
        base.offset += lhs_int.unwrap() * base.scale;
        base.op = rhs_op;
      } else if cmd == IrCmd::AddInt
        && rhs_int.is_some_and(|n| self.is_valid_integer_for_immediate(n))
      {
        base.offset += rhs_int.unwrap() * base.scale;
        base.op = lhs_op;
      } else if cmd == IrCmd::SubInt
        && rhs_int.is_some_and(|n| self.is_valid_integer_for_immediate(n))
      {
        base.offset -= rhs_int.unwrap() * base.scale;
        base.op = lhs_op;
      } else if cmd == IrCmd::TruncateUint {
        base.op = lhs_op;
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
