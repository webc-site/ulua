use ulua_vm::type_aliases::proto::Proto;

use crate::{
  enums::{ir_condition::IrCondition, ir_op_kind::IrOpKind},
  functions::{
    append::append, append_vm_constant::append_vm_constant,
    get_block_kind_name::get_block_kind_name,
    to_string_ir_dump_alt_e::to_string as to_string_const, vm_const_op::vm_const_op,
    vm_exit_op::vm_exit_op, vm_reg_op::vm_reg_op, vm_upvalue_op::vm_upvalue_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_op::IrOp, ir_to_string_context::IrToStringContext},
};

static TEXT_FOR_CONDITION: [&str; IrCondition::Count as usize] = [
  "eq", "not_eq", "lt", "not_lt", "le", "not_le", "gt", "not_gt", "ge", "not_ge", "u_lt", "u_le",
  "u_gt", "u_ge",
];

const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;

pub fn to_string(ctx: &mut IrToStringContext, op: IrOp) {
  match op.kind() {
    IrOpKind::None => {}
    IrOpKind::Undef => append(ctx.result, format_args!("undef")),
    IrOpKind::Constant => {
      let proto = ctx.proto as *mut Proto;
      let constant = ctx.constants[op.index() as usize];
      unsafe { to_string_const(ctx.result, proto, constant) };
    }
    IrOpKind::Condition => {
      CODEGEN_ASSERT!(op.index() < IrCondition::Count as u32);
      ctx.result.push_str(TEXT_FOR_CONDITION[op.index() as usize]);
    }
    IrOpKind::Inst => append(ctx.result, format_args!("%{}", op.index())),
    IrOpKind::Block => {
      let name = get_block_kind_name(ctx.blocks[op.index() as usize].kind);
      append(ctx.result, format_args!("{}_{}", name, op.index()));
    }
    IrOpKind::VmReg => append(ctx.result, format_args!("R{}", vm_reg_op(op))),
    IrOpKind::VmConst => {
      append(ctx.result, format_args!("K{}", vm_const_op(op)));

      if !ctx.proto.is_null() {
        let proto = ctx.proto as *mut Proto;
        append(ctx.result, format_args!(" ("));
        unsafe { append_vm_constant(ctx.result, proto, vm_const_op(op)) };
        append(ctx.result, format_args!(")"));
      }
    }
    IrOpKind::VmUpvalue => append(ctx.result, format_args!("U{}", vm_upvalue_op(op))),
    IrOpKind::VmExit => {
      if vm_exit_op(op) == K_VM_EXIT_ENTRY_GUARD_PC {
        append(ctx.result, format_args!("exit(entry)"));
      } else {
        append(ctx.result, format_args!("exit({})", vm_exit_op(op)));
      }
    }
  }
}
