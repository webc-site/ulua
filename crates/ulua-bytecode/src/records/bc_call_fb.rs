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
pub struct BcCallFB<'a, T = VmConst> {
  pub(crate) base: BcInstHelper<'a>,
  _marker: PhantomData<T>,
}

impl<'a, T> BcCallFB<'a, T> {
  pub const K_PARAM_START_INPUT: u32 = 4;

  /// 以 `&'a mut BcFunction`（唯一可变借用）+ 指令 `BcOp` 构造视图；
  /// 取代旧的 `unsafe fn from(*mut BcFunction, BcRef<'a, BcInst>)` 双借用把戏。
  pub fn from(graph: &'a mut BcFunction, inst: BcOp) -> Self {
    Self {
      base: BcInstHelper::new(graph, inst),
      _marker: PhantomData,
    }
  }

  pub fn params(&self) -> Vec<BcOp> {
    self.base.slice_inputs(Self::K_PARAM_START_INPUT)
  }

  pub fn param_count(&mut self) -> i32 {
    self.base.int_imm_input(0)
  }

  pub fn set_param_count(&mut self, value: u32) {
    self.base.set_imm_input(0, value as i32);
  }

  pub fn return_count(&mut self) -> i32 {
    self.base.int_imm_input(1)
  }

  pub fn set_return_count(&mut self, value: u32) {
    self.base.set_imm_input(1, value as i32);
  }

  pub fn fb_slot(&mut self) -> i32 {
    self.base.int_imm_input(2)
  }

  pub fn set_fb_slot(&mut self, value: i32) {
    self.base.set_imm_input(2, value);
  }

  pub fn target(&mut self) -> BcOp {
    self.base.get_bc_op(3)
  }

  pub fn set_target(&mut self, value: BcOp) {
    self.base.set_bc_op(3, value);
  }

  pub fn op(&self) -> BcOp {
    self.base.op()
  }
}

impl<T> BcInstHelperCreate for BcCallFB<'_, T> {
  const OPCODE: LuauOpcode = LuauOpcode::LOP_CALLFB;
}
