use ulua_analysis::{records::block::Block, type_aliases::instruction::InstructionMember};

/// 按索引取 `block` 内第 `idx` 条指令并下转型为 `&T`（类型不符则断言失败）。
pub fn require_inst<T: InstructionMember>(block: &Block, idx: usize) -> &T {
  let instructions = block.get_instructions();
  assert!(idx < instructions.len());

  let inst = instructions[idx];
  let typed = T::get_if(unsafe { &*inst });
  assert!(
    typed.is_some(),
    "instruction {idx} is not of the expected type"
  );

  // SAFETY: get_if 命中表明 inst 指向 T 类型实例；仅抹去 const 以便复用既有
  // 可变访问路径（C++ NotNull<Instruction> 语义对应）。
  unsafe { &*(typed.unwrap() as *const T) }
}
