use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// tag 直发守卫收口（binary/minus/gettableks 六站同构，userdata 直发臂同型）：
/// `tb_op` 不匹配 `tag` 即 vm_exit，随后 `CheckTag` 校验。
#[inline]
pub(crate) fn check_tag_exit(build: &mut IrBuilder, tb_op: IrOp, tag: u8, pcpos: i32) {
  let tag_op = build.const_tag(tag);
  let exit = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb_op, tag_op, exit);
}
