use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::vm_reg_op::vm_reg_op,
  macros::{
    codegen_assert::CODEGEN_ASSERT, op_a::op_a, op_b::op_b, op_c::op_c, op_d::op_d, op_e::op_e,
    op_f::op_f, op_g::op_g,
  },
  records::{
    block_vm_reg_live_in_computation::BlockVmRegLiveInComputation, ir_function::IrFunction,
    ir_inst::IrInst,
  },
};

pub fn visit_vm_reg_defs_uses_t_ir_function_ir_inst(
  visitor: &mut BlockVmRegLiveInComputation<'_>,
  function: &mut IrFunction,
  inst: &mut IrInst,
) {
  match inst.cmd {
    IrCmd::LoadTag
    | IrCmd::LoadPointer
    | IrCmd::LoadDouble
    | IrCmd::LoadInt
    | IrCmd::LoadInt64
    | IrCmd::LoadFloat
    | IrCmd::LoadTvalue => {
      visitor.maybe_use(op_a(inst));
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
      visitor.maybe_def(op_a(inst));
    }

    IrCmd::CmpAny => {
      visitor.r#use(op_a(inst), 0);
      visitor.r#use(op_b(inst.clone()), 0);
    }

    IrCmd::CmpTag => {
      visitor.maybe_use(op_a(inst));
    }

    IrCmd::JumpIfTruthy | IrCmd::JumpIfFalsy => {
      visitor.r#use(op_a(inst), 0);
    }

    IrCmd::JumpEqTag => {
      visitor.maybe_use(op_a(inst));
    }

    IrCmd::DoArith => {
      visitor.maybe_use(op_b(inst.clone()));
      visitor.maybe_use(op_c(inst.clone()));
      visitor.def(op_a(inst), 0);
    }

    IrCmd::GetTable => {
      visitor.r#use(op_b(inst.clone()), 0);
      visitor.maybe_use(op_c(inst.clone()));
      visitor.def(op_a(inst), 0);
    }

    IrCmd::SetTable => {
      visitor.r#use(op_a(inst), 0);
      visitor.r#use(op_b(inst.clone()), 0);
      visitor.maybe_use(op_c(inst.clone()));
    }

    IrCmd::DoLen => {
      visitor.r#use(op_b(inst.clone()), 0);
      visitor.def(op_a(inst), 0);
    }

    IrCmd::GetCachedImport => {
      visitor.def(op_a(inst), 0);
    }

    IrCmd::CONCAT => {
      let start = vm_reg_op(op_a(inst));
      let count = function.uint_op(op_b(inst.clone())) as i32;
      visitor.use_range(start, count);
      visitor.def_range(start, count);
    }

    IrCmd::GetUpvalue => {
      // break;
    }

    IrCmd::SetUpvalue => {
      // break;
    }

    IrCmd::INTERRUPT => {
      // break;
    }

    IrCmd::BarrierObj | IrCmd::BarrierTableForward => {
      visitor.maybe_use(op_b(inst.clone()));
    }

    IrCmd::CloseUpvals => {
      // break;
    }

    IrCmd::CAPTURE => {
      visitor.maybe_use(op_a(inst));
      if function.uint_op(op_b(inst.clone())) == 1 {
        visitor.capture(vm_reg_op(op_a(inst)));
      }
    }

    IrCmd::SETLIST => {
      visitor.r#use(op_b(inst.clone()), 0);
      visitor.use_range(
        vm_reg_op(op_c(inst.clone())),
        function.int_op(op_d(inst.clone())),
      );
    }

    IrCmd::CALL => {
      let ra = vm_reg_op(op_a(inst));
      visitor.r#use(op_a(inst), 0);
      visitor.use_range(ra + 1, function.int_op(op_b(inst.clone())));
      visitor.def_range(ra, function.int_op(op_c(inst.clone())));
    }

    IrCmd::RETURN => {
      visitor.use_range(vm_reg_op(op_a(inst)), function.int_op(op_b(inst.clone())));
    }

    IrCmd::FASTCALL => {
      visitor.r#use(op_c(inst.clone()), 0);
      visitor.def_range(
        vm_reg_op(op_b(inst.clone())),
        function.int_op(op_d(inst.clone())),
      );
    }

    IrCmd::InvokeFastcall => {
      let count = function.int_op(op_f(inst.clone()));
      if count != -1 {
        // Only LOP_FASTCALL3 lowering is allowed to have third optional argument
        if count >= 3 && op_e(inst.clone()).kind() == IrOpKind::Undef {
          CODEGEN_ASSERT!(
            op_d(inst.clone()).kind() == IrOpKind::VmReg
              && vm_reg_op(op_d(inst.clone())) == vm_reg_op(op_c(inst.clone())) + 1
          );
          visitor.use_range(vm_reg_op(op_c(inst.clone())), count);
        } else {
          if count >= 1 {
            visitor.r#use(op_c(inst.clone()), 0);
          }
          if count >= 2 {
            visitor.maybe_use(op_d(inst.clone()));
          }
          if count >= 3 {
            visitor.maybe_use(op_e(inst.clone()));
          }
        }
      } else {
        visitor.use_varargs(vm_reg_op(op_c(inst.clone())) as u8);
      }

      visitor.def_range(
        vm_reg_op(op_b(inst.clone())),
        function.int_op(op_g(inst.clone())),
      );
    }

    IrCmd::FORGLOOP => {
      visitor.r#use(op_a(inst), 1);
      visitor.r#use(op_a(inst), 2);
      visitor.def(op_a(inst), 2);
      visitor.def_range(
        vm_reg_op(op_a(inst)) + 3,
        function.int_op(op_b(inst.clone())),
      );
    }

    IrCmd::ForgloopFallback => {
      visitor.use_range(vm_reg_op(op_a(inst)), 3);
      visitor.def(op_a(inst), 2);
      visitor.def_range(
        vm_reg_op(op_a(inst)) + 3,
        (function.int_op(op_b(inst.clone())) as u8) as i32,
      );
    }

    IrCmd::ForgprepXnextFallback => {
      visitor.r#use(op_b(inst.clone()), 0);
    }

    IrCmd::FallbackGetglobal => {
      visitor.def(op_b(inst.clone()), 0);
    }

    IrCmd::FallbackSetglobal => {
      visitor.r#use(op_b(inst.clone()), 0);
    }

    IrCmd::FallbackGettableks => {
      visitor.r#use(op_c(inst.clone()), 0);
      visitor.def(op_b(inst.clone()), 0);
    }

    IrCmd::FallbackSettableks => {
      visitor.r#use(op_b(inst.clone()), 0);
      visitor.r#use(op_c(inst.clone()), 0);
    }

    IrCmd::FallbackNamecall => {
      visitor.r#use(op_c(inst.clone()), 0);
      visitor.def_range(vm_reg_op(op_b(inst.clone())), 2);
    }

    IrCmd::FallbackPrepvarargs => {
      // break;
    }

    IrCmd::FallbackGetvarargs => {
      visitor.def_range(
        vm_reg_op(op_b(inst.clone())),
        function.int_op(op_c(inst.clone())),
      );
    }

    IrCmd::FallbackDupclosure => {
      visitor.def(op_b(inst.clone()), 0);
    }

    IrCmd::FallbackForgprep => {
      let start = vm_reg_op(op_b(inst.clone()));
      visitor.use_range(start, 3);
      visitor.def_range(start, 3);
    }

    IrCmd::AdjustStackToReg => {
      visitor.def_range(vm_reg_op(op_a(inst)), -1);
    }

    IrCmd::AdjustStackToTop => {
      // break;
    }

    IrCmd::GetTypeof => {
      visitor.r#use(op_a(inst), 0);
    }

    IrCmd::FINDUPVAL => {
      visitor.r#use(op_a(inst), 0);
    }

    IrCmd::MarkUsed => {
      visitor.use_range(vm_reg_op(op_a(inst)), function.int_op(op_b(inst.clone())));
    }

    IrCmd::MarkDead => {
      // break;
    }

    _ => {
      // All instructions which reference registers have to be handled explicitly
      // for (auto& op : inst.ops) CODEGEN_ASSERT(op.kind != IrOpKind::VmReg);
      for op in inst.ops.as_slice() {
        CODEGEN_ASSERT!(op.kind() != IrOpKind::VmReg);
      }
    }
  }
}
