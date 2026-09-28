use ulua_analysis::{
  records::{block::Block, instr_registry::resolve_instruction},
  type_aliases::instruction::InstructionMember,
};

/// 按索引取 `block` 内第 `idx` 条指令并下转型为 `&T`（类型不符则断言失败）。
/// `Block.instructions` 自 #17 续起为 `InstrId` u32 句柄：句柄经
/// `instr_registry` 解析为只读 `Instruction` 视图（见该模块契约），变体下转
/// 由 `get_if` 完成，全函数 safe。
pub fn require_inst<T: InstructionMember>(block: &Block, idx: usize) -> &T {
  let instructions = block.get_instructions();
  assert!(idx < instructions.len());

  let inst = resolve_instruction(instructions[idx])
    .expect("Block.instructions 为构建期 register_instruction 发放的存活句柄");
  let typed = T::get_if(inst);
  assert!(
    typed.is_some(),
    "instruction {idx} is not of the expected type"
  );

  typed.unwrap()
}
