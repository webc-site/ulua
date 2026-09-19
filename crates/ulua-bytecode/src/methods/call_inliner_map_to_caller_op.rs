use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::bc_op_kind::BcOpKind,
  methods::call_inliner_map_vm_const_op::call_inliner_map_vm_const_op,
  records::{bc_op::BcOp, call_inliner::CallInliner},
  type_aliases::reg::Reg,
};

impl<'a> CallInliner<'a> {
  pub fn map_to_caller_op(&mut self, target_op: BcOp) -> BcOp {
    match target_op.kind {
      BcOpKind::Inst => self.map_inst_op(target_op),
      BcOpKind::Block => self.map_block_op(target_op),
      BcOpKind::Imm => {
        let imm = *self.target.imm_op(target_op);
        self.caller.immediates.push(imm);
        BcOp::bc_op_bc_op_kind_u32(
          BcOpKind::Imm,
          (self.caller.immediates.len() as u32).wrapping_sub(1),
        )
      }
      BcOpKind::Phi => {
        // cpp `mappedPhis`：递归前先备忘录查表——loop-carried phi 会（传递地）
        // 引用自身，不查表则无限递归；同时保证同一目标 phi 全程映射到同一
        // 调用者 phi，维持 op 身份一致
        if let Some(&mapped) = self.mapped_phis.find(&target_op) {
          return mapped;
        }

        let phi_op = self.caller.add_phi();
        self.mapped_phis.insert(target_op, phi_op);
        // 快照目标 phi 操作数，避免映射过程中反复重借用
        let target_phi_ops: Vec<BcOp> = self
          .target
          .phi(target_op)
          .operator_deref()
          .ops
          .as_slice()
          .to_vec();

        for target_phi_op in target_phi_ops {
          let mapped = self.map_to_caller_op(target_phi_op);
          let mut phi = self.caller.phi(phi_op);
          phi.operator_deref_mut().ops.push_back(mapped);
        }
        phi_op
      }
      BcOpKind::Proj => {
        let proj_ref = self.target.proj(target_op);
        let proj = *proj_ref.operator_deref();
        if self.target.is_vararg {
          let inst_ref = self.target.inst(proj.op);
          let inst = inst_ref.operator_deref();
          if inst.op == LuauOpcode::LOP_GETVARARGS {
            // BcGetVarArgs is a helper struct wrapping a BcInst reference.
            // Since we don't have the full BcGetVarArgs definition here,
            // we use the underlying inst and proj.index directly.
            LUAU_ASSERT!(inst.ops.len() > 1);
            return self.get_var_arg_param(proj.op, proj.index);
          }
        }
        let mapped_op = self.map_to_caller_op(proj.op);
        self.caller.add_proj(mapped_op, proj.index)
      }
      BcOpKind::VmReg => {
        if target_op.index < self.target.numparams as u32 {
          LUAU_ASSERT!(target_op.index < self.call_params.len() as u32);
          self.call_params[target_op.index as usize]
        } else {
          BcOp::bc_op_bc_op_kind_u32(
            BcOpKind::VmReg,
            self.map_to_caller_reg(target_op.index as Reg) as u32,
          )
        }
      }
      BcOpKind::VmConst => {
        call_inliner_map_vm_const_op(self.caller_vm_const_size_before_inline, target_op)
      }
      BcOpKind::VmProto => self.map_proto_op(target_op),
      BcOpKind::VmUpvalue => self.map_up_value_op(target_op),
      _ => target_op,
    }
  }
}
