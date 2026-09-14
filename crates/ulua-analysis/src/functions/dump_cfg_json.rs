extern crate alloc;

use alloc::{string::String, vec, vec::Vec};
use core::fmt::Write;

use crate::{
  functions::{
    dump_instruction::dump_instruction, index_of_block::index_of_block, json_escape::json_escape,
  },
  records::control_flow_graph::ControlFlowGraph,
};

pub fn dump_cfg_json(cfg: &ControlFlowGraph) -> String {
  // iongraph requires every "loopheader" block to have exactly one predecessor
  // marked "backedge" (asserted in Graph.ts). A loop Header is any block whose
  // predecessor has a higher index than itself (the back-edge comes from within
  // the loop body). Pre-compute both flags plus per-block loopDepth in a single
  // pass; loopDepth is the number of loops enclosing the block, and iongraph
  // uses it to resolve each block to its innermost loop Header.
  let mut is_loop_header: Vec<bool> = vec![false; cfg.blocks.len()];
  let mut is_backedge: Vec<bool> = vec![false; cfg.blocks.len()];
  let mut backedges: Vec<(usize, usize)> = Vec::new(); // (loop Header index, back-edge source index)
  for (i, &block) in cfg.blocks.iter().enumerate() {
    unsafe {
      for pred in (*block).get_predecessors() {
        let pred_idx = index_of_block(cfg, *pred);
        if pred_idx > i {
          is_loop_header[i] = true;
          is_backedge[pred_idx] = true;
          backedges.push((i, pred_idx));
        }
      }
    }
  }
  let mut loop_depth: Vec<i32> = vec![0; cfg.blocks.len()];
  for (i, depth) in loop_depth.iter_mut().enumerate() {
    for be in &backedges {
      if be.0 <= i && i <= be.1 {
        *depth += 1;
      }
    }
  }

  let mut out = String::from(
    "{\"functions\":[{\"name\":\"cfg\",\"passes\":[{\"name\":\"CFG\",\"mir\":{\"blocks\":[",
  );

  let mut next_instr_id: i32 = 1;
  for (i, &block) in cfg.blocks.iter().enumerate() {
    unsafe {
      let block = &*block;
      if i > 0 {
        out.push(',');
      }

      let _ = write!(out, "{{\"number\":{}", i);
      let _ = write!(out, ",\"loopDepth\":{}", loop_depth[i]);

      out.push_str(",\"attributes\":[");
      let mut first_attr = true;
      if is_loop_header[i] {
        out.push_str("\"loopheader\"");
        first_attr = false;
      }
      if is_backedge[i] {
        if !first_attr {
          out.push(',');
        }
        out.push_str("\"backedge\"");
      }
      out.push(']');

      out.push_str(",\"predecessors\":[");
      let preds = block.get_predecessors();
      for (j, pred) in preds.iter().enumerate() {
        if j > 0 {
          out.push(',');
        }
        let _ = write!(out, "{}", index_of_block(cfg, *pred));
      }
      out.push(']');

      out.push_str(",\"successors\":[");
      let succs = block.get_successors();
      for (j, succ) in succs.iter().enumerate() {
        if j > 0 {
          out.push(',');
        }
        let _ = write!(out, "{}", index_of_block(cfg, *succ));
      }
      out.push(']');

      out.push_str(",\"instructions\":[");
      let instructions = block.get_instructions();
      for (j, instr) in instructions.iter().enumerate() {
        if j > 0 {
          out.push(',');
        }
        let _ = write!(out, "{{\"id\":{}", next_instr_id);
        next_instr_id += 1;
        let _ = write!(
          out,
          ",\"opcode\":\"{}\"",
          json_escape(&dump_instruction(*instr, &cfg.use_defs))
        );
        out
          .push_str(",\"attributes\":[],\"inputs\":[],\"uses\":[],\"memInputs\":[],\"type\":\"\"}");
      }
      out.push_str("]}");
    }
  }

  // Close blocks array and mir object, then add an empty lir (iongraph touches p.lir.blocks unconditionally).
  out.push_str("]},\"lir\":{\"blocks\":[]}");
  // Close: pass object, passes array, function object, functions array, root object.
  out.push_str("}]}]}");
  out
}
