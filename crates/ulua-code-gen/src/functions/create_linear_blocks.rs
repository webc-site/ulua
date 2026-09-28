use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::try_create_linear_block::try_create_linear_block,
  records::{const_prop_state::ConstPropState, ir_builder::IrBuilder},
};

pub fn create_linear_blocks(build: &mut IrBuilder) {
  // state 门面的 `build` 裸指针在构造语句内交割；后续对 build.function/constant_map 的
  // 访问全部为字段路径借用，与门面顺序交错、不同时存活——不再有裸指针别名。
  let mut state = ConstPropState::const_prop_state_const_prop_state(build);
  let mut visited = vec![0u8; build.function.blocks.len()];

  // 线性化会在遍历中追加新块，故必须用索引循环，且有意不触达新增块（cpp 同款注释）
  let original_block_count = build.function.blocks.len();

  for i in 0..original_block_count {
    // 每轮重读块属性：try_create_linear_block 可能 push 新块导致 blocks 重新分配，
    // 索引不受影响
    let kind = build.function.blocks[i].kind;

    if matches!(kind, IrBlockKind::Fallback | IrBlockKind::Dead) || visited[i] != 0 {
      continue;
    }

    try_create_linear_block(build, &mut visited, i as u32, &mut state);
  }
}
