use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser},
  type_aliases::reg::Reg,
};

impl<'a> BytecodeGraphParser<'a> {
  pub fn find_producers_up_to_top(&mut self, block: BcOp, reg: Reg) -> Vec<BcOp> {
    // We assume it called only for search of var return calls.
    LUAU_ASSERT!(block.index < self.producers.len() as u32);

    let multi_return_start;
    let multi_return;
    {
      let block_producers = &self.producers[block.index as usize];
      LUAU_ASSERT!(block_producers.multi_return.kind == BcOpKind::Inst);
      multi_return_start = block_producers.multi_return_start;
      multi_return = block_producers.multi_return;
    }

    // So we need to find all producers from reg to blockProducers.multi_return_start.
    // cpp 用 `multiReturnStart - reg + 1` 预留容量；两个操作数都是 u8 且源自不可信
    // 字节码，差为负时在 Rust 里会 panic 或回绕成天文数字再触发巨量分配，故按
    // saturating 计算。容量只是提示，输出序列与 cpp 保持一致。
    let mut res = Vec::with_capacity(
      usize::from(multi_return_start)
        .saturating_sub(usize::from(reg))
        .saturating_add(1),
    );

    let mut r = reg;
    while r < multi_return_start {
      let static_reg_op = self.find_producer_bc_op_reg(block, r);
      LUAU_ASSERT!(static_reg_op.is_some());
      res.push(static_reg_op.unwrap());
      r += 1;
    }

    res.push(multi_return);

    // multireturn is consumed, clean it up
    let block_producers = &mut self.producers[block.index as usize];
    block_producers.multi_return = BcOp::new();
    block_producers.multi_return_start = 0xFF;

    res
  }
}
