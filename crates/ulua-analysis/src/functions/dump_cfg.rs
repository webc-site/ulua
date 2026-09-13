extern crate alloc;

use alloc::string::String;
use core::fmt::Write;

use crate::{
  functions::{block_kind_name::block_kind_name, dump_block::dump_block},
  records::control_flow_graph::ControlFlowGraph,
};

pub fn dump_cfg(cfg: &ControlFlowGraph) -> String {
  let mut result = String::new();
  for (i, &block) in cfg.blocks.iter().enumerate() {
    unsafe {
      let block = &*block;
      let _ = write!(result, "Block {} ({}", i, block_kind_name(block.kind));
      if !block.debug_name.is_empty() {
        let _ = write!(result, " \"{}\"", block.debug_name);
      }
      result.push(')');

      let successors = block.get_successors();
      if !successors.is_empty() {
        result.push_str(" -> [");
        for (j, &succ) in successors.iter().enumerate() {
          if j > 0 {
            result.push_str(", ");
          }
          if let Some(k) = cfg.blocks.iter().position(|&b| b == succ) {
            let _ = write!(result, "B{}", k);
          }
        }
        result.push(']');
      }

      result.push_str(":\n");
      result.push_str(&dump_block(block, &cfg.use_defs));
    }
  }
  result
}
