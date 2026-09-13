use crate::{
  enums::{
    include_cfg_info::IncludeCfgInfo, include_reg_flow_info::IncludeRegFlowInfo,
    include_use_info::IncludeUseInfo,
  },
  functions::{
    append::append, append_block_set::append_block_set, append_register_set::append_register_set,
    is_entry_block::is_entry_block, pad_to_detail_column::pad_to_detail_column,
    predecessors::predecessors, successors::successors,
    to_string_ir_dump_alt_c::to_string_ir_to_string_context_ir_block_u32 as to_string_block,
  },
  records::{ir_block::IrBlock, ir_to_string_context::IrToStringContext},
};

pub fn to_string_detailed(
  ctx: &mut IrToStringContext,
  block: &IrBlock,
  block_idx: u32,
  include_use_info: IncludeUseInfo,
  include_cfg_info: IncludeCfgInfo,
  include_reg_flow_info: IncludeRegFlowInfo,
) {
  // Launder the shared CfgInfo reference so register-set reads don't alias the
  // &mut borrow of ctx (the data lives outside ctx; this is sound).
  let cfg = ctx.cfg;

  // Report captured registers for entry block
  if include_reg_flow_info == IncludeRegFlowInfo::Yes
    && is_entry_block(block)
    && cfg.captured.regs.iter().any(|&r| r != 0)
  {
    append(ctx.result, format_args!("; captured regs: "));
    append_register_set(ctx, &cfg.captured, ", ");
    append(ctx.result, format_args!("\n\n"));
  }

  let start = ctx.result.len();

  to_string_block(ctx, block, block_idx);
  append(ctx.result, format_args!(":"));

  if include_use_info == IncludeUseInfo::Yes {
    pad_to_detail_column(ctx.result, start);
    append(
      ctx.result,
      format_args!("; useCount: {}\n", block.use_count),
    );
  } else {
    ctx.result.push('\n');
  }

  // Predecessor list
  if include_cfg_info == IncludeCfgInfo::Yes
    && (block_idx as usize) < cfg.predecessors_offsets.len()
  {
    let pred = predecessors(cfg, block_idx);

    if pred.it_begin < pred.it_end {
      append(ctx.result, format_args!("; predecessors: "));
      append_block_set(ctx, pred);
      append(ctx.result, format_args!("\n"));
    }
  }

  // Successor list
  if include_cfg_info == IncludeCfgInfo::Yes && (block_idx as usize) < cfg.successors_offsets.len()
  {
    let succ = successors(cfg, block_idx);

    if succ.it_begin < succ.it_end {
      append(ctx.result, format_args!("; successors: "));
      append_block_set(ctx, succ);
      append(ctx.result, format_args!("\n"));
    }
  }

  // Live-in VM regs
  if include_reg_flow_info == IncludeRegFlowInfo::Yes && (block_idx as usize) < cfg.r#in.len() {
    let in_rs = cfg.r#in[block_idx as usize];

    if in_rs.regs.iter().any(|&r| r != 0) || in_rs.vararg_seq {
      append(ctx.result, format_args!("; in regs: "));
      append_register_set(ctx, &in_rs, ", ");
      append(ctx.result, format_args!("\n"));
    }
  }

  // Live-out VM regs
  if include_reg_flow_info == IncludeRegFlowInfo::Yes && (block_idx as usize) < cfg.out.len() {
    let out_rs = cfg.out[block_idx as usize];

    if out_rs.regs.iter().any(|&r| r != 0) || out_rs.vararg_seq {
      append(ctx.result, format_args!("; out regs: "));
      append_register_set(ctx, &out_rs, ", ");
      append(ctx.result, format_args!("\n"));
    }
  }
}
