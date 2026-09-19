use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    append::append, append_label_regset::append_label_regset, is_pseudo::is_pseudo,
    to_string_ir_dump::to_string as to_string_inst,
    to_string_ir_dump_alt_c::to_string_ir_to_string_context_ir_block_u32 as to_string_block,
  },
  records::{ir_function::IrFunction, ir_to_string_context::IrToStringContext},
};

pub fn append_blocks(
  ctx: &mut IrToStringContext,
  function: &IrFunction,
  include_inst: bool,
  include_in: bool,
  include_out: bool,
  include_def: bool,
) {
  // Launder the shared CfgInfo reference so the register-set reads don't alias
  // the &mut borrow of ctx.
  let cfg = ctx.cfg;

  for (i, &block) in function.blocks.iter().enumerate() {
    append(ctx.result, format_args!("b{} [", i as u32));

    if block.kind == IrBlockKind::Fallback {
      append(ctx.result, format_args!("style=filled;fillcolor=salmon;"));
    } else if block.kind == IrBlockKind::Bytecode {
      append(
        ctx.result,
        format_args!("style=filled;fillcolor=palegreen;"),
      );
    }

    ctx.result.push_str("label=\"{");
    to_string_block(ctx, &block, i as u32);

    if include_in {
      append_label_regset(ctx, &cfg.r#in, i, "in");
    }

    if include_inst && block.start != !0u32 {
      for inst_idx in block.start..=block.finish {
        let inst = &function.instructions[inst_idx as usize];

        // Skip pseudo instructions unless they are still referenced
        if is_pseudo(inst.cmd) && inst.use_count == 0 {
          continue;
        }

        ctx.result.push('|');
        to_string_inst(ctx, inst, inst_idx);
      }
    }

    if include_def {
      append_label_regset(ctx, &cfg.def, i, "def");
    }

    if include_out {
      append_label_regset(ctx, &cfg.out, i, "out");
    }

    ctx.result.push_str("}\"];\n");
  }
}
