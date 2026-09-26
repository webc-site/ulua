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

  let mut worklist = Vec::new();
  let mut in_worklist = vec![false; block_count];

  // 合并遍历：def/in 计算与 worklist 构建共用一次迭代（Dead 块两轮均跳过）
  for block_idx in 0..block_count {
    let block = function.blocks[block_idx];

    if block.kind == IrBlockKind::Dead {
      continue;
    }

    let mut def_rs = function.cfg.def[block_idx];
    let in_rs = compute_block_live_in_reg_set(function, &block, &mut def_rs, &mut captured_regs);
    function.cfg.def[block_idx] = def_rs;
    function.cfg.r#in[block_idx] = in_rs;

    worklist.push(block_idx as u32);
    in_worklist[block_idx] = true;
  }

  function.cfg.captured.regs = captured_regs;

  while let Some(block_idx) = worklist.pop() {
    in_worklist[block_idx as usize] = false;

    let curr = function.blocks[block_idx as usize];

    let mut out_rs = function.cfg.out[block_idx as usize];
    let successors_it = successors(&function.cfg, block_idx);
    for succ_idx in successors_it {
      let succ = function.blocks[succ_idx as usize];

      if curr.kind != IrBlockKind::Fallback && succ.kind == IrBlockKind::Fallback {
        CODEGEN_ASSERT!(successors(&function.cfg, block_idx).size() != 1);
        continue;
      }

      let succ_rs = function.cfg.r#in[succ_idx as usize];
      for (out_reg, &succ_reg) in out_rs.regs.iter_mut().zip(succ_rs.regs.iter()) {
        *out_reg |= succ_reg;
      }

      if succ_rs.vararg_seq {
        CODEGEN_ASSERT!(!out_rs.vararg_seq || out_rs.vararg_start == succ_rs.vararg_start);

        out_rs.vararg_seq = true;
        out_rs.vararg_start = succ_rs.vararg_start;
      }
    }
    function.cfg.out[block_idx as usize] = out_rs;

    let old_in_rs = function.cfg.r#in[block_idx as usize];
    let def_rs = function.cfg.def[block_idx as usize];

    for (in_reg, (&out_reg, &def_reg)) in function.cfg.r#in[block_idx as usize]
      .regs
      .iter_mut()
      .zip(out_rs.regs.iter().zip(def_rs.regs.iter()))
    {
      *in_reg |= out_reg & !def_reg;
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
    for (written_reg, &def_reg) in function.cfg.written.regs.iter_mut().zip(def_rs.regs.iter()) {
      *written_reg |= def_reg;
    }

    if def_rs.vararg_seq {
      if !function.cfg.written.vararg_seq || def_rs.vararg_start < function.cfg.written.vararg_start
      {
        function.cfg.written.vararg_start = def_rs.vararg_start;
      }

      function.cfg.written.vararg_seq = true;
    }
  }

  // `proto_view` 收口空指针判定与解引用（review.md §2），本函数不再手工判 `is_null`。
  if let Some(proto) = function.proto_view() {
    let entry_in = function.cfg.r#in[0];
    CODEGEN_ASSERT!(!entry_in.vararg_seq);

    // 每个存活的寄存器位必须落在参数范围内；只扫描置位位，避免逐位空转
    let numparams = proto.numparams as usize;
    for (word, &regs) in entry_in.regs.iter().enumerate() {
      let mut bits = regs;
      while bits != 0 {
        let bit = bits.trailing_zeros() as usize;
        bits &= bits - 1; // 清除最低置位位
        CODEGEN_ASSERT!(word * 64 + bit < numparams);
      }
    }
  }
}
