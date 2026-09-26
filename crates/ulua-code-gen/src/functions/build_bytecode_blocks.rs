use core::slice::from_raw_parts;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fast_call::is_fast_call,
  },
  macros::luau_insn_ops::luau_insn_op,
};

use crate::records::{bytecode_block::BytecodeBlock, ir_function::IrFunction};

// See also: 本 crate `macros/codegen_assert.rs` 的 `CODEGEN_ASSERT!`——形似义异：
// 该宏走 ulua_common `assert_fail` C-ABI 上报通道，此处是同名局部替身（标准
// `assert!` panic 语义），刻意不统一，勿合并。
macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

pub(crate) fn build_bytecode_blocks(function: &mut IrFunction, jump_targets: &[u8]) {
  CODEGEN_ASSERT!(!function.proto.is_null());

  // Safety: 上一行断言已确认 function.proto 非空, 且它是构造时接线、比本函数长寿的活 Proto,
  // 据此重建共享引用无别名冲突(只读遍历)。
  let proto = unsafe { &*function.proto };
  let code = if proto.code.is_null() || proto.sizecode <= 0 {
    &[][..]
  } else {
    unsafe { from_raw_parts(proto.code, proto.sizecode as usize) }
  };
  let bc_blocks = &mut function.bc_blocks;

  // 复用相同的 jump target，创建 VM 字节码基本块
  bc_blocks.push(BytecodeBlock {
    startpc: 0,
    finishpc: -1,
  });

  let mut previ = 0;
  let mut i = 0;

  while i < proto.sizecode {
    let pc_val = code[i as usize];
    let op_val = luau_insn_op(pc_val) as u8;
    // opcode 字节钳制转换：越界（损坏字节码）落 LopNop，与 C++ switch default 一致，避免 UB
    let op: LuauOpcode = LuauOpcode::from(op_val);

    let nexti = i + get_op_length(op);

    // 若指令是 jump target，从它开始新建 block
    if i != 0 && jump_targets[i as usize] != 0 {
      if let Some(last) = bc_blocks.last_mut() {
        last.finishpc = previ;
      }
      bc_blocks.push(BytecodeBlock {
        startpc: i,
        finishpc: -1,
      });
    }

    let target = get_jump_target(pc_val, i as u32);

    // 隐式 fallthrough 会终止当前 block，并可能开启新 block
    if target >= 0 && !is_fast_call(op) {
      if let Some(last) = bc_blocks.last_mut() {
        last.finishpc = i;
      }

      // 若 fallthrough 没有显式 jump，则开启新 block
      if jump_targets[nexti as usize] == 0 {
        bc_blocks.push(BytecodeBlock {
          startpc: nexti,
          finishpc: -1,
        });
      }
    }
    // Returns 直接终止 block
    else if op == LuauOpcode::LOP_RETURN
      && let Some(last) = bc_blocks.last_mut()
    {
      last.finishpc = i;
    }

    previ = i;
    i = nexti;
    CODEGEN_ASSERT!(i <= proto.sizecode);
  }
}
