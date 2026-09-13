use core::marker::PhantomData;

use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  methods::{bc_function_as::BcInstType, bc_inst_helper_create::BcInstHelperCreate},
  records::{
    bc_function::{BcFunction, VmConst},
    bc_inst::BcInst,
    bc_inst_helper::BcInstHelper,
    bc_ref::BcRef,
  },
  type_aliases::reg::Reg,
};

#[derive(Debug)]
pub struct BcGetVarArgs<'a, T = VmConst> {
  pub(crate) base: BcInstHelper<'a>,
  _marker: PhantomData<T>,
}

impl<'a, T> BcGetVarArgs<'a, T> {
  pub const K_START_REG_INPUT: u32 = 0;

  /// # Safety
  ///
  /// `graph` must point to a valid, initialized `BcFunction`.
  pub unsafe fn from(graph: *mut BcFunction, inst: BcRef<'a, BcInst>) -> Self {
    Self {
      base: unsafe { BcInstHelper::new(&mut *graph, inst) },
      _marker: PhantomData,
    }
  }

  pub fn values_count(&mut self) -> i32 {
    self.base.int_imm_input(1)
  }

  pub fn set_values_count(&mut self, value: u32) {
    self.base.set_imm_input(1, value as i32);
  }

  pub fn start_reg(&self) -> Reg {
    self.base.operator_deref().ops[Self::K_START_REG_INPUT as usize].index as Reg
  }
}

impl<T> BcInstType for BcGetVarArgs<'_, T> {
  const OPCODE: i32 = LuauOpcode::LOP_GETVARARGS as i32;
}

impl<T> BcInstHelperCreate for BcGetVarArgs<'_, T> {
  const OPCODE: LuauOpcode = LuauOpcode::LOP_GETVARARGS;
}
