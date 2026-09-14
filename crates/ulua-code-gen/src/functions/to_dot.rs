extern crate alloc;

use alloc::string::String;
use core::ffi::c_void;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  functions::{append::append, append_blocks::append_blocks},
  records::{ir_function::IrFunction, ir_to_string_context::IrToStringContext},
};

pub fn to_dot(function: &IrFunction, include_inst: bool) -> String {
  let mut result = String::new();

  {
    let mut ctx = IrToStringContext {
      result: &mut result,
      blocks: &function.blocks,
      constants: &function.constants,
      cfg: &function.cfg,
      vm_exit_info: &function.vm_exit_info,
      proto: function.proto as *mut c_void,
    };

    ctx.result.push_str("digraph CFG {\n");
    ctx.result.push_str("node[shape=record]\n");

    append_blocks(
      &mut ctx,
      function,
      include_inst,
      /* include_in */ true,
      /* include_out */ true,
      /* include_def */ true,
    );

    for i in 0..function.blocks.len() {
      let block = function.blocks[i];

      if block.start == !0u32 {
        continue;
      }

      let mut inst_idx = block.start;
      while inst_idx != !0u32 && inst_idx <= block.finish {
        let inst = &function.instructions[inst_idx as usize];

        for op in inst.ops.as_slice() {
          if op.kind() == IrOpKind::Block {
            if function.blocks[op.index() as usize].kind != IrBlockKind::Fallback {
              append(
                ctx.result,
                format_args!("b{} -> b{} [weight=10];\n", i as u32, op.index()),
              );
            } else {
              append(
                ctx.result,
                format_args!("b{} -> b{};\n", i as u32, op.index()),
              );
            }
          }
        }

        inst_idx += 1;
      }
    }

    ctx.result.push_str("}\n");
  }

  result
}
