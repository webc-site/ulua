use gxhash::HashMap;

use crate::{records::bc_op::BcOp, type_aliases::reg::Reg};

/// cpp 的 `invalidAfter = 255` / `multiReturnStart = 0xFF` 哨兵值：`Reg` 为 `u8`，
/// 取 `Reg::MAX` 表示「失效区间覆盖全部寄存器」/「多返回段起点在寄存器号尽头」。
pub(crate) const PRODUCER_SENTINEL: Reg = Reg::MAX;

/// 建图热路径的寄存器 → 生产者映射：`gxhash::HashMap`（AES 加速哈希，评审
/// r5 口径：哈希表统一 gxhash，无序语义同 std）。`Reg` 全域合法，无哨兵空键，
/// `DenseHashMap` 不适用。
type ProducerMap = HashMap<Reg, BcOp>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlockProducers {
  pub(crate) own: ProducerMap,
  pub(crate) cached: ProducerMap,
  pub(crate) multi_return: BcOp,
  pub(crate) multi_return_start: Reg,
  pub(crate) invalid_after: i32,
}

impl Default for BlockProducers {
  fn default() -> Self {
    Self {
      own: HashMap::default(),
      cached: HashMap::default(),
      multi_return: BcOp::new(),
      multi_return_start: 0,
      invalid_after: PRODUCER_SENTINEL as i32,
    }
  }
}
