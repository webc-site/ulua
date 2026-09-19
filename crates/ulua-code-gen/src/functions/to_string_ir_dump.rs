use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{
    append::append, get_cmd_name::get_cmd_name, has_result::has_result,
    to_string_ir_dump_alt_d::to_string as to_string_op,
  },
  records::{ir_inst::IrInst, ir_to_string_context::IrToStringContext},
};

pub fn to_string(ctx: &mut IrToStringContext, inst: &IrInst, index: u32) {
  ctx.result.push_str("  ");

  // 有结果的指令显示目标虚拟寄存器
  if has_result(inst.cmd) {
    append(ctx.result, format_args!("%{} = ", index));
  }

  ctx.result.push_str(get_cmd_name(inst.cmd));

  for (i, &op) in inst.ops.as_slice().iter().enumerate() {
    if op.kind() == IrOpKind::None {
      continue;
    }

    ctx.result.push_str(if i == 0 { " " } else { ", " });
    to_string_op(ctx, op);
  }
}
