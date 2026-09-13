use ulua_bytecode::{
  enums::bc_op_kind::BcOpKind, functions::inline_call::inline_call, records::bc_op::BcOp,
  type_aliases::comp_time_bc_function::CompTimeBcFunction,
};
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;

impl BytecodeInlinerFixture {
  pub fn compile_and_inline(
    &mut self,
    src: &str,
    call_idx: u32,
  ) -> Option<(CompTimeBcFunction, CompTimeBcFunction)> {
    let (mut inlinee, mut caller) = self.build_bytecode(src, 0)?;

    let mut call = BcOp::new();
    let mut idx = 0u32;
    for (i, inst) in caller.instructions.iter().enumerate() {
      if inst.op == LuauOpcode::LOP_CALLFB {
        if idx == call_idx {
          call = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Inst, i as u32);
          break;
        }
        idx += 1;
      }
    }

    assert_ne!(call.kind, BcOpKind::None);
    if !inline_call(&mut caller, &mut inlinee, call, 0) {
      return None;
    }

    Some((inlinee, caller))
  }
}
