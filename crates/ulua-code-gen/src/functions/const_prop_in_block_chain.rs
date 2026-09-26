use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    const_prop_in_block::const_prop_in_block, get_live_out_value_count::get_live_out_value_count,
    save_block_exit_state::save_block_exit_state,
    setup_block_entry_state_optimize_const_prop::setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state,
  },
  macros::{codegen_assert::CODEGEN_ASSERT, op_a_ref::op_a_ref},
  records::{
    const_prop_state::ConstPropState, ir_block::K_BLOCK_FLAG_SAFE_ENV_CHECK,
    ir_builder::ConstantMap, ir_function::IrFunction,
  },
};

/// 起始块以索引传入，链遍历沿索引推进；全部块/指令访问经函数数组按索引即时绑定，
/// 不再持有 `*mut IrFunction` 别名——`function`/`constant_map` 为 build 的互斥字段借用，
/// `ConstPropState` 门面在 callee 内部派生同源视图并顺序交割。
pub fn const_prop_in_block_chain(
  function: &mut IrFunction,
  constant_map: &mut ConstantMap,
  visited: &mut [u8],
  starting_idx: u32,
  state: &mut ConstPropState,
) {
  state.clear();

  setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state(
    function,
    starting_idx,
    state,
  );

  let start_sortkey = function.blocks[starting_idx as usize].sortkey;
  let mut chain_pos = 0u32;
  let mut block_idx = starting_idx;
  let mut has_block = true;

  while has_block {
    CODEGEN_ASSERT!(visited[block_idx as usize] == 0);
    visited[block_idx as usize] = 1;

    if state.in_safe_env
      && (function.blocks[block_idx as usize].flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0
    {
      function.blocks[block_idx as usize].flags &= !K_BLOCK_FLAG_SAFE_ENV_CHECK;
    }

    const_prop_in_block(function, constant_map, block_idx, state);

    if function.blocks[block_idx as usize].kind == IrBlockKind::Dead {
      break;
    }

    function.blocks[block_idx as usize].sortkey = start_sortkey;
    function.blocks[block_idx as usize].chainkey = chain_pos;
    chain_pos += 1;

    // cpp: `IrInst& termInst = function.instructions[block->finish];`
    // 链遍历是热路径，只取终止指令的 Copy 字段（cmd 与第 0 个操作数），
    // 不再克隆整条 IrInst（含操作数 SmallVector）。
    let finish = function.blocks[block_idx as usize].finish;
    let (term_cmd, target_op) = {
      let term = &function.instructions[finish as usize];
      (term.cmd, op_a_ref(term))
    };

    let mut next_block: Option<u32> = None;

    if term_cmd == IrCmd::JUMP && target_op.kind() == IrOpKind::Block {
      let target_idx = target_op.index();
      // 按原短路次序读 Copy 字段与本地 `visited`（纯读）。
      let target_candidate = {
        let target = &function.blocks[target_idx as usize];
        target.use_count == 1
          && visited[target_idx as usize] == 0
          && target.kind != IrBlockKind::Fallback
      };

      if target_candidate {
        // cpp OptimizeConstProp.cpp:3672-3673：live-out 非空直接终止链，出口状态不落盘
        if get_live_out_value_count(function, target_idx) != 0 {
          break;
        }

        function.blocks[block_idx as usize].expected_next_block = target_idx;
        next_block = Some(target_idx);
      }
    }

    save_block_exit_state(function, block_idx, state);

    match next_block {
      Some(idx) => block_idx = idx,
      None => has_block = false,
    }
  }
}
