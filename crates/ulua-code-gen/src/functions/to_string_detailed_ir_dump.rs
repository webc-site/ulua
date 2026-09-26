use crate::{
  enums::{
    include_cfg_info::IncludeCfgInfo, include_reg_flow_info::IncludeRegFlowInfo,
    include_use_info::IncludeUseInfo,
  },
  functions::{
    append::append,
    append_block_set::append_block_set,
    append_register_set::append_register_set,
    get_jump_target_extra_live_in::get_jump_target_extra_live_in,
    has_side_effects::has_side_effects,
    is_entry_block::is_entry_block,
    is_non_terminating_jump::is_non_terminating_jump,
    pad_to_detail_column::pad_to_detail_column,
    predecessors::predecessors,
    successors::successors,
    to_string_ir_dump::{
      to_string_inst, to_string_ir_to_string_context_ir_block_u32 as to_string_block, to_string_op,
    },
  },
  records::{ir_block::IrBlock, ir_inst::IrInst, ir_to_string_context::IrToStringContext},
};

pub(crate) fn to_string_detailed(
  ctx: &mut IrToStringContext,
  block: &IrBlock,
  block_idx: u32,
  inst: &mut IrInst,
  inst_idx: u32,
  include_use_info: IncludeUseInfo,
) {
  let start = ctx.result.len();

  to_string_inst(ctx, inst, inst_idx);

  if include_use_info == IncludeUseInfo::Yes {
    pad_to_detail_column(ctx.result, start);

    if inst.use_count == 0 && has_side_effects(inst.cmd) {
      if is_non_terminating_jump(inst.cmd) {
        let extra_rs = get_jump_target_extra_live_in(ctx, block, block_idx, inst);

        if extra_rs.regs.iter().any(|&r| r != 0) || extra_rs.vararg_seq {
          append(ctx.result, format_args!("; %{}, extra in: ", inst_idx));
          append_register_set(ctx, &extra_rs, ", ");
          ctx.result.push('\n');
        } else {
          append(ctx.result, format_args!("; %{}\n", inst_idx));
        }
      } else {
        append(ctx.result, format_args!("; %{}\n", inst_idx));
      }
    } else {
      append(
        ctx.result,
        format_args!(
          "; useCount: {}, lastUse: %{}\n",
          inst.use_count, inst.last_use
        ),
      );
    }
  } else {
    ctx.result.push('\n');
  }

  if let Some(sync) = ctx
    .vm_exit_info
    .find(&inst_idx)
    .cloned()
    .filter(|sync| !sync.reg_stores.is_empty())
  {
    append(ctx.result, format_args!("   ; exit sync: "));

    let mut comma = false;

    for el in &sync.reg_stores {
      if comma {
        append(ctx.result, format_args!(", "));
      }
      comma = true;

      append(ctx.result, format_args!("R{}", el.reg));
    }

    comma = false;

    append(ctx.result, format_args!(", {{"));

    for arg_op in sync.arg_ops.as_slice() {
      if comma {
        append(ctx.result, format_args!(", "));
      }
      comma = true;

      to_string_op(ctx, *arg_op);
    }

    append(ctx.result, format_args!("}}"));
    append(ctx.result, format_args!("\n"));
  }
}

pub(crate) fn to_string_detailed_block(
  ctx: &mut IrToStringContext,
  block: &IrBlock,
  block_idx: u32,
  include_use_info: IncludeUseInfo,
  include_cfg_info: IncludeCfgInfo,
  include_reg_flow_info: IncludeRegFlowInfo,
) {
  // Launder 共享的 CfgInfo 引用，让寄存器集读取不与
  // ctx 的 &mut 借用别名（数据在 ctx 之外，这样可靠）。
  let cfg = ctx.cfg;

  // 报告 entry block 的 captured 寄存器
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

  // 前驱列表
  if include_cfg_info == IncludeCfgInfo::Yes
    && (block_idx as usize) < cfg.predecessors_offsets.len()
  {
    let pred = predecessors(cfg, block_idx);

    if !pred.empty() {
      append(ctx.result, format_args!("; predecessors: "));
      append_block_set(ctx, pred);
      append(ctx.result, format_args!("\n"));
    }
  }

  // 后继列表
  if include_cfg_info == IncludeCfgInfo::Yes && (block_idx as usize) < cfg.successors_offsets.len()
  {
    let succ = successors(cfg, block_idx);

    if !succ.empty() {
      append(ctx.result, format_args!("; successors: "));
      append_block_set(ctx, succ);
      append(ctx.result, format_args!("\n"));
    }
  }

  // Live-in VM 寄存器
  if include_reg_flow_info == IncludeRegFlowInfo::Yes && (block_idx as usize) < cfg.r#in.len() {
    let in_rs = cfg.r#in[block_idx as usize];

    if in_rs.regs.iter().any(|&r| r != 0) || in_rs.vararg_seq {
      append(ctx.result, format_args!("; in regs: "));
      append_register_set(ctx, &in_rs, ", ");
      append(ctx.result, format_args!("\n"));
    }
  }

  // Live-out VM 寄存器
  if include_reg_flow_info == IncludeRegFlowInfo::Yes && (block_idx as usize) < cfg.out.len() {
    let out_rs = cfg.out[block_idx as usize];

    if out_rs.regs.iter().any(|&r| r != 0) || out_rs.vararg_seq {
      append(ctx.result, format_args!("; out regs: "));
      append_register_set(ctx, &out_rs, ", ");
      append(ctx.result, format_args!("\n"));
    }
  }
}
