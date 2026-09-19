use core::marker::PhantomData;

use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  methods::bc_inst_helper_create::BcInstHelperCreate,
  records::{
    bc_function::{BcFunction, VmConst},
    bc_inst_helper::BcInstHelper,
    bc_op::BcOp,
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

  /// 持有图的唯一可变借用 + 指令 `BcOp`（见 `BcReturn::from`）。
  pub fn from(graph: &'a mut BcFunction, inst: BcOp) -> Self {
    Self {
      base: BcInstHelper::new(graph, inst),
      _marker: PhantomData,
    }
  }

  pub fn values_count(&mut self) -> i32 {
    self.base.int_imm_input(1)
  }

  pub fn start_reg(&self) -> Reg {
    self.base.operator_deref().ops[Self::K_START_REG_INPUT as usize].index as Reg
  }
}


impl<T> BcInstHelperCreate for BcGetVarArgs<'_, T> {
  const OPCODE: LuauOpcode = LuauOpcode::LOP_GETVARARGS;
}
