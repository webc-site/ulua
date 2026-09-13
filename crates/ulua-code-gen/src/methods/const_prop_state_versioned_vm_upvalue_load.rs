use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_inst::IrInst;
use crate::macros::op_a::OP_A;
use crate::functions::vm_upvalue_op::vm_upvalue_op;

impl ConstPropState {
    pub fn versioned_vm_upvalue_load(&mut self, load_inst: &mut IrInst) -> IrInst {
        let mut op = OP_A(load_inst.clone());
        let version = self.regs[vm_upvalue_op(op) as usize].version;
        op.kind_and_index = vm_upvalue_op(OP_A(load_inst.clone())) | (version << 8);
        IrInst { cmd: load_inst.cmd, ops: crate::type_aliases::ir_ops::IrOps::from([op]), last_use: 0, use_count: 0, reg_x64: Default::default(), reg_a64: Default::default(), reused_reg: false, spilled: false, needs_reload: false }
    }
}
