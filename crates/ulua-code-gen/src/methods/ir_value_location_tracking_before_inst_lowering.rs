use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::vm_reg_op::vm_reg_op,
  macros::{
    codegen_assert::CODEGEN_ASSERT, op_a::op_a, op_b_ref::op_b_ref, op_c_ref::op_c_ref,
    op_d_ref::op_d_ref, op_g_ref::op_g_ref,
  },
  records::{ir_inst::IrInst, ir_value_location_tracking::IrValueLocationTracking},
};

impl IrValueLocationTracking {
  pub fn before_inst_lowering(&mut self, inst: &mut IrInst) {
    match inst.cmd {
      IrCmd::StoreTag => self.invalidate_restore_op(op_a(inst), true),
      IrCmd::StoreExtra
      | IrCmd::StorePointer
      | IrCmd::StoreDouble
      | IrCmd::StoreInt
      | IrCmd::StoreInt64
      | IrCmd::StoreVector
      | IrCmd::StoreTvalue
      | IrCmd::StoreSplitTvalue => self.invalidate_restore_op(op_a(inst), false),

      IrCmd::AdjustStackToReg => self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)), -1),
      IrCmd::FASTCALL => {
        let function = unsafe { &*self.function };
        self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), function.int_op(op_d_ref(inst)));
      }
      IrCmd::InvokeFastcall => {
        let function = unsafe { &*self.function };
        self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), function.int_op(op_g_ref(inst)));
      }

      IrCmd::DoArith | IrCmd::DoLen | IrCmd::GetTable | IrCmd::GetCachedImport => {
        self.invalidate_restore_op(op_a(inst), false)
      }
      IrCmd::CONCAT => {
        let function = unsafe { &*self.function };
        self.invalidate_restore_vm_regs(
          vm_reg_op(op_a(inst)),
          function.uint_op(op_b_ref(inst)) as i32,
        );
      }
      IrCmd::GetUpvalue => {}
      IrCmd::CALL => self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)), -1),
      IrCmd::FORGLOOP | IrCmd::ForgloopFallback => {
        self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)) + 2, -1)
      }
      IrCmd::FallbackGetglobal | IrCmd::FallbackGettableks => {
        self.invalidate_restore_op(op_b_ref(inst), false)
      }
      IrCmd::FallbackNamecall => self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), 2),
      IrCmd::FallbackGetvarargs => {
        let function = unsafe { &*self.function };
        self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), function.int_op(op_c_ref(inst)));
      }
      IrCmd::FallbackDupclosure => self.invalidate_restore_op(op_b_ref(inst), false),
      IrCmd::FallbackForgprep => self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), 3),

      IrCmd::LoadTag
      | IrCmd::LoadPointer
      | IrCmd::LoadDouble
      | IrCmd::LoadInt64
      | IrCmd::LoadInt
      | IrCmd::LoadFloat
      | IrCmd::LoadTvalue
      | IrCmd::CmpAny
      | IrCmd::CmpTag
      | IrCmd::JumpIfTruthy
      | IrCmd::JumpIfFalsy
      | IrCmd::JumpEqTag
      | IrCmd::SelectInt64
      | IrCmd::SetTable
      | IrCmd::SetUpvalue
      | IrCmd::INTERRUPT
      | IrCmd::BarrierObj
      | IrCmd::BarrierTableForward
      | IrCmd::CloseUpvals
      | IrCmd::CAPTURE
      | IrCmd::SETLIST
      | IrCmd::RETURN
      | IrCmd::ForgprepXnextFallback
      | IrCmd::FallbackSetglobal
      | IrCmd::FallbackSettableks
      | IrCmd::FallbackPrepvarargs
      | IrCmd::AdjustStackToTop
      | IrCmd::GetTypeof
      | IrCmd::NEWCLOSURE
      | IrCmd::FINDUPVAL
      | IrCmd::CheckTag
      | IrCmd::CheckTruthy
      | IrCmd::AddNum
      | IrCmd::SubNum
      | IrCmd::MulNum
      | IrCmd::DivNum
      | IrCmd::IdivNum
      | IrCmd::ModNum
      | IrCmd::MinNum
      | IrCmd::MaxNum
      | IrCmd::JumpCmpNum
      | IrCmd::FloorNum
      | IrCmd::CeilNum
      | IrCmd::RoundNum
      | IrCmd::SqrtNum
      | IrCmd::AbsNum => {}

      _ => {
        for op in inst.ops.as_slice() {
          CODEGEN_ASSERT!(op.kind() != IrOpKind::VmReg);
        }
      }
    }
  }
}
