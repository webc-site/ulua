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
pub struct BcSetList<'a, T = VmConst> {
  pub(crate) base: BcInstHelper<'a>,
  _marker: PhantomData<T>,
}

impl<'a, T> BcSetList<'a, T> {
  /// cpp `BcSetList::kParamStartInput`（BytecodeOps.h:322）：ops[0]=startIndex、
  /// ops[1]=count、ops[2]=target，参数从 ops[3] 起。
  pub const K_PARAM_START_INPUT: u32 = 3;

  /// 持有图的唯一可变借用 + 指令 `BcOp`（见 `BcReturn::from`）。
  pub fn from(graph: &'a mut BcFunction, inst: BcOp) -> Self {
    Self {
      base: BcInstHelper::new(graph, inst),
      _marker: PhantomData,
    }
  }

  pub fn count(&mut self) -> i32 {
    self.base.int_imm_input(1)
  }

  pub fn set_count(&mut self, value: u32) {
    self.base.set_imm_input(1, value as i32);
  }

  pub fn params(&self) -> Vec<BcOp> {
    self.base.slice_inputs(Self::K_PARAM_START_INPUT)
  }
}

impl<T> BcInstHelperCreate for BcSetList<'_, T> {
  const OPCODE: LuauOpcode = LuauOpcode::LOP_SETLIST;
}
