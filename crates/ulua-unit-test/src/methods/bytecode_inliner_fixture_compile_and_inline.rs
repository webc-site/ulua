use ulua_bytecode::{
  enums::bc_op_kind::BcOpKind,
  functions::inline_call::inline_call,
  records::{bc_function::BcFunction, bc_op::BcOp},
};
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;

impl BytecodeInlinerFixture {
  pub fn compile_and_inline(
    &mut self,
    src: &str,
    call_idx: u32,
    optimization_level: i32,
  ) -> Option<(BcFunction<'_>, BcFunction<'_>)> {
    let (mut inlinee, mut caller) = self.build_bytecode(src, optimization_level)?;

    let mut call = BcOp::new();
    let mut idx = 0u32;
    for (i, inst) in caller.instructions.iter().enumerate() {
      if inst.op == LuauOpcode::LOP_CALLFB {
        if idx == call_idx {
          call = BcOp::with(BcOpKind::Inst, i as u32);
          break;
        }
        idx += 1;
      }
    }

    assert_ne!(call.kind, BcOpKind::None);
    // cpp 测试同样传 callerFbVecSize=0（`BytecodeCallInliner.test.cpp` 用默认实参）
    if !inline_call(&mut caller, &mut inlinee, call, 0, 0) {
      return None;
    }

    Some((inlinee, caller))
  }
}
