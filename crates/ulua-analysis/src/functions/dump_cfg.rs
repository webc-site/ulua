extern crate alloc;

use alloc::string::String;
use core::fmt::Write;

use crate::{
  functions::{block_kind_name::block_kind_name, dump_block::dump_block},
  records::{block_registry::resolve_block, control_flow_graph::ControlFlowGraph},
};

pub fn dump_cfg(cfg: &ControlFlowGraph) -> String {
  let mut result = String::new();
  for (i, &block) in cfg.blocks.iter().enumerate() {
    // cfg.blocks 句柄经注册表解析（见 `block_registry` 模块契约）：转储只读
    // kind/debug_name/successors/use_defs，单线程内无人可变借用该 CFG。
    {
      let block = resolve_block(block).expect("BlockId 为构建期 register_block 发放的存活句柄");
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
