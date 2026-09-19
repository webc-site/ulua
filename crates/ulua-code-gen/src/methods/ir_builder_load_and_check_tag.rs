use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn load_and_check_tag(&mut self, loc: IrOp, tag: u8, fallback: IrOp) {
    let tag_op = self.inst_ir_cmd_ir_op(IrCmd::LoadTag, loc);
    let const_tag_op = self.const_tag(tag);
    self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_op, const_tag_op, fallback);
  }
}
