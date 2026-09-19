use alloc::vec::Vec;
use core::marker::PhantomData;

use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  methods::{bc_function_as::BcInstType, bc_inst_helper_create::BcInstHelperCreate},
  records::{
    bc_function::{BcFunction, VmConst},
    bc_inst::BcInst,
    bc_inst_helper::BcInstHelper,
    bc_op::BcOp,
    bc_ref::BcRef,
  },
};

#[derive(Debug)]
pub struct BcReturn<'a, T = VmConst> {
  pub(crate) base: BcInstHelper<'a>,
  _marker: PhantomData<T>,
}

impl<'a, T> BcReturn<'a, T> {
  pub const K_VALUES_START_INPUT: u32 = 1;

  /// # Safety
  ///
  /// `graph` must point to a valid, initialized `BcFunction`.
  pub unsafe fn from(graph: *mut BcFunction, inst: BcRef<'a, BcInst>) -> Self {
    Self {
      base: unsafe { BcInstHelper::new(&mut *graph, inst) },
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

impl<T> BcInstType for BcReturn<'_, T> {
  const OPCODE: i32 = LuauOpcode::LOP_RETURN as i32;
}

impl<T> BcInstHelperCreate for BcReturn<'_, T> {
  const OPCODE: LuauOpcode = LuauOpcode::LOP_RETURN;
}
