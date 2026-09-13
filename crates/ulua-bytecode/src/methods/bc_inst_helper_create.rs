use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::{bc_function::BcFunction, bc_inst_helper::BcInstHelper};

pub trait BcInstHelperCreate {
  const OPCODE: LuauOpcode;
}

impl<'a> BcInstHelper<'a> {
  pub fn create<T>(graph: &'a mut BcFunction) -> BcInstHelper<'a>
  where
    T: BcInstHelperCreate,
  {
    let op = graph.add_inst();
    {
      let inst = graph.inst_op(op);
      inst.op = T::OPCODE;
    }
    let graph_ptr = graph as *mut BcFunction;
    let inst = unsafe { (*graph_ptr).inst(op) };
    BcInstHelper::new(unsafe { &mut *graph_ptr }, inst)
  }
}
