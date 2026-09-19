use crate::{
  enums::ir_cmd::IrCmd, functions::kill_ir_utils::kill_ir_function_ir_inst_at,
  macros::codegen_assert::CODEGEN_ASSERT, records::ir_function::IrFunction,
};

pub fn kill_ir_function_u32_u32(function: &mut IrFunction, start: u32, end: u32) {
  // 逆序 kill，避免误杀仍被标记为使用的指令
  for idx in (start..=end).rev() {
    CODEGEN_ASSERT!((idx as usize) < function.instructions.len());

    let inst = &function.instructions[idx as usize];
    // 只读判定：cmd 为 NOP 或仍在使用则跳过；借用在此结束，下面的 kill 才能取可变借用
    let killable = inst.cmd != IrCmd::NOP && inst.use_count == 0;

    // 不强制销毁仍在使用的指令：操作数释放时它会随之自动销毁
    if killable {
      kill_ir_function_ir_inst_at(function, idx);
    }
  }
}
