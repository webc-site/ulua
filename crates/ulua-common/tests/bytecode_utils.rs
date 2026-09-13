//! Inline port of the opcode-helper semantics from `Common/include/Luau/BytecodeUtils.h`
//! and `Common/src/BytecodeWire.cpp` (varint decoding, jump targets, fast-call
//! classification), plus `ExperimentalFlags.h` (experimental analysis flags).

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fallthrough::is_fallthrough,
    is_fast_call::is_fast_call, is_jump_d::is_jump_d, is_loop_jump::is_loop_jump,
    is_skip_c::is_skip_c, read_var_int::read_var_int, read_var_int_64::read_var_int_64,
  },
};

#[test]
fn fast_call_classification_matches_cpp() {
  // C++ isFastCall 覆盖整个 fastcall 系列，含 LOP_FASTPCALL。
  assert!(is_fast_call(LuauOpcode::LOP_FASTCALL));
  assert!(is_fast_call(LuauOpcode::LOP_FASTCALL1));
  assert!(is_fast_call(LuauOpcode::LOP_FASTCALL2));
  assert!(is_fast_call(LuauOpcode::LOP_FASTCALL2K));
  assert!(is_fast_call(LuauOpcode::LOP_FASTCALL3));
  assert!(is_fast_call(LuauOpcode::LOP_FASTPCALL));
  assert!(!is_fast_call(LuauOpcode::LOP_JUMP));
  assert!(!is_fast_call(LuauOpcode::LOP_LOADB));
}

#[test]
fn jump_classification_matches_cpp() {
  assert!(is_jump_d(LuauOpcode::LOP_JUMP));
  assert!(is_jump_d(LuauOpcode::LOP_JUMPBACK));
  assert!(is_jump_d(LuauOpcode::LOP_CMPPROTO));
  assert!(!is_jump_d(LuauOpcode::LOP_JUMPX));
  assert!(is_skip_c(LuauOpcode::LOP_LOADB));
  assert!(!is_skip_c(LuauOpcode::LOP_JUMP));
  assert!(!is_fallthrough(LuauOpcode::LOP_RETURN));
  assert!(is_fallthrough(LuauOpcode::LOP_ADD));
  assert!(is_loop_jump(LuauOpcode::LOP_JUMPBACK));
  assert!(is_loop_jump(LuauOpcode::LOP_FORNLOOP));
  assert!(!is_loop_jump(LuauOpcode::LOP_JUMP));
}

#[test]
fn op_length_matches_cpp() {
  assert_eq!(get_op_length(LuauOpcode::LOP_NAMECALL), 2);
  assert_eq!(get_op_length(LuauOpcode::LOP_FASTCALL3), 2);
  assert_eq!(get_op_length(LuauOpcode::LOP_NEWCLASS), 2);
  assert_eq!(get_op_length(LuauOpcode::LOP_FASTPCALL), 1);
  assert_eq!(get_op_length(LuauOpcode::LOP_LOADB), 1);
  assert_eq!(get_op_length(LuauOpcode::LOP_ADD), 1);
}

#[test]
fn jump_target_matches_cpp() {
  use ulua_common::{
    functions::get_jump_target::getJumpTarget,
    macros::{luau_insn_c::luau_insn_c, luau_insn_d::luau_insn_d, luau_insn_op::luau_insn_op},
  };

  // 编码辅助：op 在低 8 位，D 为 16..31 位（有符号），C 为 24..31 位。
  let jump = luau_insn_op(LuauOpcode::LOP_JUMP as u32) | (5u32 << 16); // D = 5
  assert_eq!(luau_insn_d(jump), 5);
  assert_eq!(get_jump_target(jump, 10), 16);
  // camelCase 别名同一路径。
  assert_eq!(getJumpTarget(jump, 10), 16);
  // isSkipC 且 C != 0：pc + C + 1。
  let loadb = luau_insn_op(LuauOpcode::LOP_LOADB as u32) | (3u32 << 24); // C = 3
  assert_eq!(luau_insn_c(loadb), 3);
  assert_eq!(get_jump_target(loadb, 10), 14);
  // isSkipC 且 C == 0：非跳转，返回 -1。
  let loadb0 = luau_insn_op(LuauOpcode::LOP_LOADB as u32);
  assert_eq!(get_jump_target(loadb0, 10), -1);
  // 非跳转指令：-1。
  assert_eq!(
    get_jump_target(luau_insn_op(LuauOpcode::LOP_ADD as u32), 10),
    -1
  );
}

#[test]
fn varint_roundtrip_matches_cpp_do_while() {
  // C++ readVarInt64 是 do-while：至少读一个字节，低 7 位逐段拼接。
  let mut offset = 0;
  assert_eq!(read_var_int_64(&[0x00], &mut offset), 0);
  assert_eq!(offset, 1);
  let mut offset = 0;
  assert_eq!(read_var_int_64(&[0x7f], &mut offset), 0x7f);
  let mut offset = 0;
  // 多字节：0xff 0x00 → 0x7f。
  assert_eq!(read_var_int_64(&[0xff, 0x00], &mut offset), 0x7f);
  assert_eq!(offset, 2);
  let mut offset = 0;
  assert_eq!(read_var_int(&[0xac, 0x02], &mut offset), 300);
}

mod experimental_flags {
  use std::ptr;

  use ulua_common::functions::is_analysis_flag_experimental::is_analysis_flag_experimental;

  #[test]
  fn list_matches_experimental_flags_h() {
    unsafe {
      assert!(is_analysis_flag_experimental(
        c"LuauInstantiateInSubtyping".as_ptr()
      ));
      assert!(is_analysis_flag_experimental(
        c"LuauFixIndexerSubtypingOrdering".as_ptr()
      ));
      assert!(is_analysis_flag_experimental(c"LuauSolverV2".as_ptr()));
      assert!(is_analysis_flag_experimental(
        c"UseNewLuauTypeSolverDefaultEnabled".as_ptr()
      ));
      assert!(is_analysis_flag_experimental(
        c"LuauRefactorStringSemanticSubtyping".as_ptr()
      ));
      assert!(!is_analysis_flag_experimental(
        c"LuauConstraintGraph".as_ptr()
      ));
      assert!(!is_analysis_flag_experimental(ptr::null()));
    }
  }
}
