use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    compute_block_live_in_reg_set::compute_block_live_in_reg_set, predecessors::predecessors,
    require_variadic_sequence::require_variadic_sequence, successors::successors,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, register_set::RegisterSet},
};

pub fn compute_cfg_live_in_out_reg_sets(function: &mut IrFunction) {
  function.cfg.def.clear();
  function.cfg.out.clear();

  let block_count = function.blocks.len();

  function.cfg.r#in.clear();
  function
    .cfg
    .r#in
    .resize(block_count, RegisterSet::default());
  function.cfg.def.resize(block_count, RegisterSet::default());
  function.cfg.out.resize(block_count, RegisterSet::default());

  let mut captured_regs = [0u64; 4];

  for block_idx in 0..block_count {
    let block = function.blocks[block_idx];

    if block.kind == IrBlockKind::Dead {
      continue;
    }

    let mut def_rs = function.cfg.def[block_idx];
    let in_rs = compute_block_live_in_reg_set(function, &block, &mut def_rs, &mut captured_regs);
    function.cfg.def[block_idx] = def_rs;
    function.cfg.r#in[block_idx] = in_rs;
  }

  function.cfg.captured.regs = captured_regs;

  let mut worklist = Vec::new();
  let mut in_worklist = vec![false; block_count];

  for block_idx in 0..block_count {
    let block = function.blocks[block_idx];

    if block.kind == IrBlockKind::Dead {
      continue;
    }

    worklist.push(block_idx as u32);
    in_worklist[block_idx] = true;
  }

  while let Some(block_idx) = worklist.pop() {
    in_worklist[block_idx as usize] = false;

    let curr = function.blocks[block_idx as usize];

    let successors_it = successors(&function.cfg, block_idx);
    for succ_idx in successors_it {
      let succ = function.blocks[succ_idx as usize];

      if curr.kind != IrBlockKind::Fallback && succ.kind == IrBlockKind::Fallback {
        CODEGEN_ASSERT!(successors(&function.cfg, block_idx).size() != 1);
        continue;
      }

      let succ_rs = function.cfg.r#in[succ_idx as usize];
      for word in 0..4 {
        function.cfg.out[block_idx as usize].regs[word] |= succ_rs.regs[word];
      }

      if succ_rs.vararg_seq {
        CODEGEN_ASSERT!(
          !function.cfg.out[block_idx as usize].vararg_seq
            || function.cfg.out[block_idx as usize].vararg_start == succ_rs.vararg_start
        );

        function.cfg.out[block_idx as usize].vararg_seq = true;
        function.cfg.out[block_idx as usize].vararg_start = succ_rs.vararg_start;
      }
    }

    let old_in_rs = function.cfg.r#in[block_idx as usize];
    let def_rs = function.cfg.def[block_idx as usize];
    let out_rs = function.cfg.out[block_idx as usize];

    for word in 0..4 {
      function.cfg.r#in[block_idx as usize].regs[word] |= out_rs.regs[word] & !def_rs.regs[word];
    }

    if out_rs.vararg_seq {
      let in_rs = &mut function.cfg.r#in[block_idx as usize];
      require_variadic_sequence(in_rs, &def_rs, out_rs.vararg_start);
    }

    let in_rs = function.cfg.r#in[block_idx as usize];
    if in_rs.regs != old_in_rs.regs || in_rs.vararg_seq != old_in_rs.vararg_seq {
      for pred_idx in predecessors(&function.cfg, block_idx) {
        if !in_worklist[pred_idx as usize] {
          worklist.push(pred_idx);
          in_worklist[pred_idx as usize] = true;
        }
      }
    }
  }

  function.cfg.written.regs = [0; 4];
  function.cfg.written.vararg_seq = false;
  function.cfg.written.vararg_start = 0;

  for block_idx in 0..block_count {
    let block = function.blocks[block_idx];

    if block.kind == IrBlockKind::Dead {
      continue;
    }

    let def_rs = function.cfg.def[block_idx];
    for word in 0..4 {
      function.cfg.written.regs[word] |= def_rs.regs[word];
    }

    if def_rs.vararg_seq {
      if !function.cfg.written.vararg_seq || def_rs.vararg_start < function.cfg.written.vararg_start
      {
        function.cfg.written.vararg_start = def_rs.vararg_start;
      }

      function.cfg.written.vararg_seq = true;
    }
  }

  if !function.proto.is_null() {
    let entry_in = function.cfg.r#in[0];
    CODEGEN_ASSERT!(!entry_in.vararg_seq);

    for i in 0..256usize {
      let live = (entry_in.regs[i / 64] & (1u64 << (i % 64))) != 0;
      unsafe {
        CODEGEN_ASSERT!(!live || i < (*function.proto).numparams as usize);
      }
    }
  }
}
