use crate::{
  enums::ir_cmd::IrCmd,
  functions::{has_result::has_result, is_pseudo::is_pseudo},
};

pub fn has_side_effects(cmd: IrCmd) -> bool {
  if cmd == IrCmd::InvokeFastcall {
    return true;
  }

  if is_pseudo(cmd) {
    return false;
  }

  // 不产出结果的指令几乎必然带有别的副作用才有意义
  // 目前做完整 switch 只会镜像 'hasResult' 函数，故用此简单条件
  !has_result(cmd)
}
