use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  bc_function::BcFunction, bc_inst::BcInst, bc_inst_helper::BcInstHelper, bc_ref::BcRef,
};

pub trait BcInstType {
  const OPCODE: i32;
}

impl<'a> BcInstHelper<'a> {
  pub fn from<T>(graph: &'a mut BcFunction, inst: BcRef<'a, BcInst>) -> T
  where
    T: BcInstType + From<(&'a mut BcFunction, BcRef<'a, BcInst>)>,
  {
    LUAU_ASSERT!(inst.operator_deref().op as i32 == T::OPCODE);
    T::from((graph, inst))
  }
}
