use alloc::vec::Vec;

use ulua_common::fint::LuauCodeGenMinLinearBlockPath;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    collect_direct_block_jump_path::collect_direct_block_jump_path,
    const_prop_in_block::const_prop_in_block, replace_ir_utils::replace_ir_function_ir_op_ir_op_at,
    setup_block_entry_state_optimize_const_prop::setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state,
  },
  macros::{codegen_assert::CODEGEN_ASSERT, op_a_ref::op_a_ref},
  records::{const_prop_state::ConstPropState, ir_builder::IrBuilder},
};

/// 起始块以索引传入；对 blocks/instructions 的访问一律经 `build.function` 字段路径按索引
/// 即时绑定，块/函数借用为互不相交的字段借用（function / constant_map），
/// 与 ConstPropState 门面顺序交割——本函数零 unsafe、零裸指针。
pub fn try_create_linear_block(
  build: &mut IrBuilder,
  visited: &mut Vec<u8>,
  starting_idx: u32,
  state: &mut ConstPropState,
) {
  let block_idx = starting_idx;

  CODEGEN_ASSERT!(visited[block_idx as usize] == 0);
  visited[block_idx as usize] = 1;

  let term_inst_idx = build.function.blocks[block_idx as usize].finish;
  let term_inst = build.function.instructions[term_inst_idx as usize].clone();

  if term_inst.cmd != IrCmd::JUMP {
    return;
  }

  let target_op = op_a_ref(&term_inst);
  if target_op.kind() != IrOpKind::Block {
    return;
  }

  if build.function.blocks[target_op.index() as usize].use_count == 1 {
    return;
  }

  let target_block_idx = target_op.index();
  let path = collect_direct_block_jump_path(&mut build.function, visited, block_idx);

  if (path.len() as i32) < LuauCodeGenMinLinearBlockPath.get() {
    return;
  }

  state.clear();

  setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state(
    &mut build.function,
    block_idx,
    state,
  );

  const_prop_in_block(
    &mut build.function,
    &mut build.constant_map,
    block_idx,
    state,
  );

  // 校验 target 未变化（cpp OptimizeConstProp.cpp:3852）
  // 若重传播后块仍发生变化，说明首轮常量传播未收敛，放弃线性化
  let current_target = {
    let inst = &build.function.instructions[term_inst_idx as usize];
    op_a_ref(inst).index()
  };

  if build.function.blocks[block_idx as usize].finish != term_inst_idx
    || current_target != target_block_idx
  {
    return;
  }

  // 路径首块仍需多个前驱持有，否则后续块替换会提前杀死块链
  if build.function.blocks[target_block_idx as usize].use_count == 1 {
    return;
  }

  let starting_sort_key = build.function.blocks[block_idx as usize].sortkey;
  let starting_chain_key = build.function.blocks[block_idx as usize].chainkey;

  let new_block = build.block(IrBlockKind::Linearized);
  visited.push(0);

  build.begin_block(new_block);

  build.function.blocks[new_block.index() as usize].sortkey = starting_sort_key;
  build.function.blocks[new_block.index() as usize].chainkey = starting_chain_key + 1;
  build.function.blocks[block_idx as usize].expected_next_block = new_block.index();

  replace_ir_function_ir_op_ir_op_at(&mut build.function, term_inst_idx, 0, new_block);

  build.clone(path.clone(), true);

  if build.function.cfg.r#in.len() == new_block.index() as usize {
    CODEGEN_ASSERT!(build.function.cfg.r#in.len() == build.function.cfg.out.len());
    CODEGEN_ASSERT!(build.function.cfg.r#in.len() == build.function.cfg.def.len());

    build
      .function
      .cfg
      .r#in
      .push(build.function.cfg.r#in[path[0] as usize]);
    // 不变式：cpp 同源直接取 path.back()/cfg.in 尾项，此处 path 非空由上方
    // `path.len() >= LuauCodeGenMinLinearBlockPath(缺省 2)` 早退保证。
    let tail_idx = *path
      .last()
      .expect("path 非空：长度已 >= LuauCodeGenMinLinearBlockPath(>=1) 方可至此");
    let tail_out = build.function.cfg.out[tail_idx as usize];
    build.function.cfg.out.push(tail_out);
    build.function.cfg.def.push(Default::default());

    // 即上一步刚推入的尾项，直接复用其值，免再次 `last()`
    let out_vararg_seq = tail_out.vararg_seq;
    let def_idx = build.function.cfg.def.len() - 1;

    for &path_block_idx in &path {
      let path_def = build.function.cfg.def[path_block_idx as usize];
      // 一次可变借用聚合归并，寄存器组用迭代器 zip 免去逐下标取用
      let def = &mut build.function.cfg.def[def_idx];
      for (d, p) in def.regs.iter_mut().zip(path_def.regs) {
        *d |= p;
      }

      if path_def.vararg_seq && out_vararg_seq {
        def.vararg_seq = true;
        def.vararg_start = path_def.vararg_start;
      }
    }

    build
      .function
      .cfg
      .predecessors_offsets
      .push(build.function.cfg.predecessors.len() as u32);
    build.function.cfg.predecessors.push(block_idx);
  }

  const_prop_in_block(
    &mut build.function,
    &mut build.constant_map,
    new_block.index(),
    state,
  );
}
