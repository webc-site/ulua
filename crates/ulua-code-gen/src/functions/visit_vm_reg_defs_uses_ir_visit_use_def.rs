use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::vm_reg_op::vm_reg_op,
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    ir_operand::{op_a_ref, op_b_ref, op_c_ref, op_d_ref, op_e_ref, op_f_ref, op_g_ref},
  },
  records::{
    block_vm_reg_live_in_computation::BlockVmRegLiveInComputation, ir_block::IrBlock,
    ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp,
  },
};

pub trait VmRegDefsUsesVisitor {
  fn maybe_use(&mut self, op: IrOp);
  fn maybe_def(&mut self, op: IrOp);
  fn use_(&mut self, op: IrOp, offset: i32);
  fn def(&mut self, op: IrOp, offset: i32);
  fn use_range(&mut self, start: i32, count: i32);
  fn def_range(&mut self, start: i32, count: i32);
  fn capture(&mut self, reg: i32);
  fn use_varargs(&mut self, vararg_start: u8);
}

impl VmRegDefsUsesVisitor for BlockVmRegLiveInComputation<'_> {
  #[inline]
  fn maybe_use(&mut self, op: IrOp) {
    self.maybe_use(op);
  }
  #[inline]
  fn maybe_def(&mut self, op: IrOp) {
    self.maybe_def(op);
  }
  #[inline]
  fn use_(&mut self, op: IrOp, offset: i32) {
    self.r#use(op, offset);
  }
  #[inline]
  fn def(&mut self, op: IrOp, offset: i32) {
    self.def(op, offset);
  }
  #[inline]
  fn use_range(&mut self, start: i32, count: i32) {
    self.use_range(start, count);
  }
  #[inline]
  fn def_range(&mut self, start: i32, count: i32) {
    self.def_range(start, count);
  }
  #[inline]
  fn capture(&mut self, reg: i32) {
    self.capture(reg);
  }
  #[inline]
  fn use_varargs(&mut self, vararg_start: u8) {
    self.use_varargs(vararg_start);
  }
}

pub fn visit_vm_reg_defs_uses<V: VmRegDefsUsesVisitor>(
  visitor: &mut V,
  function: &IrFunction,
  inst: &IrInst,
) {
  match inst.cmd {
    IrCmd::LoadTag
    | IrCmd::LoadPointer
    | IrCmd::LoadDouble
    | IrCmd::LoadInt
    | IrCmd::LoadInt64
    | IrCmd::LoadFloat
    | IrCmd::LoadTvalue => {
      visitor.maybe_use(op_a_ref(inst));
    }

    IrCmd::StoreTag
    | IrCmd::StoreExtra
    | IrCmd::StorePointer
    | IrCmd::StoreDouble
    | IrCmd::StoreInt
    | IrCmd::StoreInt64
    | IrCmd::StoreVector
    | IrCmd::StoreTvalue
    | IrCmd::StoreSplitTvalue => {
      visitor.maybe_def(op_a_ref(inst));
    }

    IrCmd::CmpAny => {
      visitor.use_(op_a_ref(inst), 0);
      visitor.use_(op_b_ref(inst), 0);
    }

    IrCmd::CmpTag => {
      visitor.maybe_use(op_a_ref(inst));
    }

    IrCmd::JumpIfTruthy | IrCmd::JumpIfFalsy => {
      visitor.use_(op_a_ref(inst), 0);
    }

    IrCmd::JumpEqTag => {
      visitor.maybe_use(op_a_ref(inst));
    }

    IrCmd::DoArith => {
      visitor.maybe_use(op_b_ref(inst));
      visitor.maybe_use(op_c_ref(inst));
      visitor.def(op_a_ref(inst), 0);
    }

    IrCmd::GetTable => {
      visitor.use_(op_b_ref(inst), 0);
      visitor.maybe_use(op_c_ref(inst));
      visitor.def(op_a_ref(inst), 0);
    }

    IrCmd::SetTable => {
      visitor.use_(op_a_ref(inst), 0);
      visitor.use_(op_b_ref(inst), 0);
      visitor.maybe_use(op_c_ref(inst));
    }

    IrCmd::DoLen => {
      visitor.use_(op_b_ref(inst), 0);
      visitor.def(op_a_ref(inst), 0);
    }

    IrCmd::GetCachedImport => {
      visitor.def(op_a_ref(inst), 0);
    }

    IrCmd::CONCAT => {
      let start = vm_reg_op(op_a_ref(inst));
      let count = function.uint_op(op_b_ref(inst)) as i32;
      visitor.use_range(start, count);
      visitor.def_range(start, count);
    }

    IrCmd::GetUpvalue | IrCmd::SetUpvalue | IrCmd::INTERRUPT => {}

    IrCmd::BarrierObj | IrCmd::BarrierTableForward => {
      visitor.maybe_use(op_b_ref(inst));
    }

    IrCmd::CloseUpvals => {}

    IrCmd::CAPTURE => {
      visitor.maybe_use(op_a_ref(inst));
      if function.uint_op(op_b_ref(inst)) == 1 {
        visitor.capture(vm_reg_op(op_a_ref(inst)));
      }
    }

    IrCmd::SETLIST => {
      visitor.use_(op_b_ref(inst), 0);
      visitor.use_range(vm_reg_op(op_c_ref(inst)), function.int_op(op_d_ref(inst)));
    }

    IrCmd::CALL => {
      let ra = vm_reg_op(op_a_ref(inst));
      visitor.use_(op_a_ref(inst), 0);
      visitor.use_range(ra + 1, function.int_op(op_b_ref(inst)));
      visitor.def_range(ra, function.int_op(op_c_ref(inst)));
    }

    IrCmd::RETURN => {
      visitor.use_range(vm_reg_op(op_a_ref(inst)), function.int_op(op_b_ref(inst)));
    }

    IrCmd::FASTCALL => {
      visitor.use_(op_c_ref(inst), 0);
      visitor.def_range(vm_reg_op(op_b_ref(inst)), function.int_op(op_d_ref(inst)));
    }

    IrCmd::InvokeFastcall => {
      let count = function.int_op(op_f_ref(inst));
      if count != -1 {
        // 只有 LOP_FASTCALL3 的 lowering 允许第三个可选参数
        if count >= 3 && op_e_ref(inst).kind() == IrOpKind::Undef {
          CODEGEN_ASSERT!(
            op_d_ref(inst).kind() == IrOpKind::VmReg
              && vm_reg_op(op_d_ref(inst)) == vm_reg_op(op_c_ref(inst)) + 1
          );
          visitor.use_range(vm_reg_op(op_c_ref(inst)), count);
        } else {
          if count >= 1 {
            visitor.use_(op_c_ref(inst), 0);
          }
          if count >= 2 {
            visitor.maybe_use(op_d_ref(inst));
          }
          if count >= 3 {
            visitor.maybe_use(op_e_ref(inst));
          }
        }
      } else {
        visitor.use_varargs(vm_reg_op(op_c_ref(inst)) as u8);
      }

      visitor.def_range(vm_reg_op(op_b_ref(inst)), function.int_op(op_g_ref(inst)));
    }

    IrCmd::FORGLOOP => {
      visitor.use_(op_a_ref(inst), 1);
      visitor.use_(op_a_ref(inst), 2);
      visitor.def(op_a_ref(inst), 2);
      visitor.def_range(
        vm_reg_op(op_a_ref(inst)) + 3,
        function.int_op(op_b_ref(inst)),
      );
    }

    IrCmd::ForgloopFallback => {
      visitor.use_range(vm_reg_op(op_a_ref(inst)), 3);
      visitor.def(op_a_ref(inst), 2);
      visitor.def_range(
        vm_reg_op(op_a_ref(inst)) + 3,
        (function.int_op(op_b_ref(inst)) as u8) as i32,
      );
    }

    IrCmd::ForgprepXnextFallback => {
      visitor.use_(op_b_ref(inst), 0);
    }

    IrCmd::FallbackGetglobal => {
      visitor.def(op_b_ref(inst), 0);
    }

    IrCmd::FallbackSetglobal => {
      visitor.use_(op_b_ref(inst), 0);
    }

    IrCmd::FallbackGettableks => {
      visitor.use_(op_c_ref(inst), 0);
      visitor.def(op_b_ref(inst), 0);
    }

    IrCmd::FallbackSettableks => {
      visitor.use_(op_b_ref(inst), 0);
      visitor.use_(op_c_ref(inst), 0);
    }

    IrCmd::FallbackNamecall => {
      visitor.use_(op_c_ref(inst), 0);
      visitor.def_range(vm_reg_op(op_b_ref(inst)), 2);
    }

    IrCmd::FallbackPrepvarargs => {}

    IrCmd::FallbackGetvarargs => {
      visitor.def_range(vm_reg_op(op_b_ref(inst)), function.int_op(op_c_ref(inst)));
    }

    IrCmd::FallbackDupclosure => {
      visitor.def(op_b_ref(inst), 0);
    }

    IrCmd::FallbackForgprep => {
      let start = vm_reg_op(op_b_ref(inst));
      visitor.use_range(start, 3);
      visitor.def_range(start, 3);
    }

    IrCmd::AdjustStackToReg => {
      visitor.def_range(vm_reg_op(op_a_ref(inst)), -1);
    }

    IrCmd::AdjustStackToTop => {}

    IrCmd::GetTypeof | IrCmd::FINDUPVAL => {
      visitor.use_(op_a_ref(inst), 0);
    }

    IrCmd::MarkUsed => {
      visitor.use_range(vm_reg_op(op_a_ref(inst)), function.int_op(op_b_ref(inst)));
    }

    IrCmd::MarkDead => {}

    _ => {
      for op in inst.ops.as_slice() {
        CODEGEN_ASSERT!(op.kind() != IrOpKind::VmReg);
      }
    }
  }
}

/// 被访问者对 function/instructions 均为只读（int_op/uint_op + 指令字段快照），
/// 借用改共享后同调用并传，消除裸指针回转。
pub fn visit_vm_reg_defs_uses_t_ir_function_ir_block(
  visitor: &mut BlockVmRegLiveInComputation<'_>,
  function: &IrFunction,
  block: &IrBlock,
) {
  let start = block.start;
  let finish = block.finish;

  for inst_idx in start..=finish {
    visit_vm_reg_defs_uses(visitor, function, &function.instructions[inst_idx as usize]);
  }
}
