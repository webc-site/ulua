use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  records::{cfg_info::CfgInfo, ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

pub fn compute_cfg_block_edges(function: &mut IrFunction) {
  let info: &mut CfgInfo = &mut function.cfg;

  // 清除既有数据
  info.predecessors_offsets.clear();
  info.successors_offsets.clear();

  // 计算前驱 block 边
  info.predecessors_offsets.reserve(function.blocks.len());
  info.successors_offsets.reserve(function.blocks.len());

  let mut edge_count: u32 = 0;

  for block in &function.blocks {
    info.predecessors_offsets.push(edge_count);
    edge_count = edge_count.wrapping_add(block.use_count as u32);
  }

  info.predecessors.resize(edge_count as usize, 0);
  info.successors.resize(edge_count as usize, 0);

  edge_count = 0;

  for (block_idx, block) in function.blocks.iter().enumerate() {
    info.successors_offsets.push(edge_count);

    if block.kind == IrBlockKind::Dead {
      continue;
    }

    for inst_idx in block.start..=block.finish {
      let inst: &IrInst = &function.instructions[inst_idx as usize];

      let mut check_op = |op: &IrOp| {
        if op.kind() == IrOpKind::Block {
          // 借用前驱列表偏移作为写入游标（稍后回调）
          let pred_pos = info.predecessors_offsets[op.index() as usize] as usize;
          info.predecessors[pred_pos] = block_idx as u32;

          info.predecessors_offsets[op.index() as usize] += 1;

          info.successors[edge_count as usize] = op.index();
          edge_count += 1;
        }
      };

      for op in &inst.ops {
        check_op(op);
      }
    }
  }

  // 前一轮循环里前驱列表偏移被当作迭代游标使用
  // 减去 block use 计数把它们回调（predecessor 数 == block uses）
  for (block_idx, block) in function.blocks.iter().enumerate() {
    info.predecessors_offsets[block_idx] -= block.use_count as u32;
  }
}
