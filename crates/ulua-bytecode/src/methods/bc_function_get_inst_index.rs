use core::ptr;

use crate::records::{bc_function::BcFunction, bc_inst::BcInst};

impl BcFunction {
  /// cpp `BcFunction::getInstIndex`：由指令指针反查下标。
  ///
  /// 只能用于本函数 `instructions` 里的指令。cpp 版在 release 下（`LUAU_ASSERT`
  /// 被编译掉）对越界/非同源指针做 `offset_from` 属 UB，且 `as u32` 静默截断；
  /// 这里改为 `ptr::eq` 定位 + 越界 panic。签名保持 `-> u32` 不变。
  pub fn get_inst_index(&self, inst: &BcInst) -> u32 {
    let index = self
      .instructions
      .iter()
      .position(|candidate| ptr::eq(candidate, inst))
      .unwrap_or_else(|| panic!("BcFunction::get_inst_index: inst is not from this function"));

    u32::try_from(index).expect("BcFunction::get_inst_index: inst index overflows u32")
  }
}
