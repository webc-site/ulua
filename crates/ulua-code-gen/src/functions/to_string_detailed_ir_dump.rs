use ulua_common::FFlag::LuauCodegenVmExitSync;

use crate::{
  enums::include_use_info::IncludeUseInfo,
  functions::{
    append::append, append_register_set::append_register_set,
    get_jump_target_extra_live_in::get_jump_target_extra_live_in,
    has_side_effects::has_side_effects, is_non_terminating_jump::is_non_terminating_jump,
    pad_to_detail_column::pad_to_detail_column, to_string_ir_dump::to_string as to_string_inst,
    to_string_ir_dump_alt_d::to_string as to_string_op,
  },
  records::{ir_block::IrBlock, ir_inst::IrInst, ir_to_string_context::IrToStringContext},
};

pub fn to_string_detailed(
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

  if LuauCodegenVmExitSync.get() {
    let sync_opt = ctx.vm_exit_info.find(&inst_idx).cloned();

    if let Some(sync) = sync_opt
      && !sync.reg_stores.is_empty()
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
}
