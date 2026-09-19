use alloc::vec::Vec;
use core::marker::PhantomData;

use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  methods::bc_inst_helper_create::BcInstHelperCreate,
  records::{
    bc_function::{BcFunction, VmConst},
    bc_inst_helper::BcInstHelper,
    bc_op::BcOp,
  },
};

#[derive(Debug)]
pub struct BcReturn<'a, T = VmConst> {
  pub(crate) base: BcInstHelper<'a>,
  _marker: PhantomData<T>,
}

impl<'a, T> BcReturn<'a, T> {
  pub const K_VALUES_START_INPUT: u32 = 1;

  /// 持有图的唯一可变借用 + 指令 `BcOp`（旧的 `*mut BcFunction` + `BcRef` 双借用
  /// 构造会构成别名冲突，属 UB）。
  pub fn from(graph: &'a mut BcFunction, inst: BcOp) -> Self {
    Self {
      base: BcInstHelper::new(graph, inst),
      _marker: PhantomData,
    }
  }

  pub fn return_count(&mut self) -> i32 {
    self.base.int_imm_input(0)
  }

  pub fn set_return_count(&mut self, value: u32) {
    self.base.set_imm_input(0, value as i32);
  }

  pub fn values(&mut self) -> Vec<BcOp> {
    if self.return_count() == 0 {
      Vec::new()
    } else {
      self.base.slice_inputs(Self::K_VALUES_START_INPUT)
    }
  }
}

impl<T> BcInstHelperCreate for BcReturn<'_, T> {
  const OPCODE: LuauOpcode = LuauOpcode::LOP_RETURN;
}
