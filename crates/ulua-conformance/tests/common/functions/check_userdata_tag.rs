//! IR 侧 userdata tag 检查（cpp `ConformanceIrHooks.h` 各 userdata 类型的
//! 同款 `check_tag` 辅助）：发射 `CheckUserdataTag(udata, tag, vmExit(pcpos))`。
//! `Vec2` / `Vertex` 的 `check_tag` 之前各抄一份，收口于此、tag 由调用方传入。
use ulua_code_gen::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn check_userdata_tag(build: &mut IrBuilder, udata: IrOp, pcpos: i32, tag: i32) {
  let tag_op = build.const_int(tag);
  let exit = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckUserdataTag, udata, tag_op, exit);
}
