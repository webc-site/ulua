use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::read::BytecodeRead, records::instruction::Instruction,
};

#[test]
fn test_instruction_basic_and_deref() {
  let insn = Instruction::new(0x1234_5678);
  assert_eq!(insn.raw(), 0x1234_5678);
  assert_eq!(*insn, 0x1234_5678);

  let raw: u32 = insn.into();
  assert_eq!(raw, 0x1234_5678);

  let from_u32 = Instruction::from(0xABCD_EF01);
  assert_eq!(from_u32.raw(), 0xABCD_EF01);
}

#[test]
fn test_instruction_fields_abc() {
  // OP: 0x12, A: 0x34, B: 0x56, C: 0x78
  let raw = 0x12 | (0x34 << 8) | (0x56 << 16) | (0x78 << 24);
  let insn = Instruction(raw);

  assert_eq!(insn.op(), 0x12);
  assert_eq!(insn.a(), 0x34);
  assert_eq!(insn.b(), 0x56);
  assert_eq!(insn.c(), 0x78);
}

#[test]
fn test_instruction_field_d_signed() {
  // 正偏移: D = 100
  let insn_pos = Instruction::encode_ad(LuauOpcode::LOP_JUMP, 0, 100);
  assert_eq!(insn_pos.d(), 100);

  // 负偏移: D = -1
  let insn_neg1 = Instruction::encode_ad(LuauOpcode::LOP_JUMPBACK, 0, -1);
  assert_eq!(insn_neg1.d(), -1);

  // 极值负偏移: D = -32768
  let insn_min = Instruction::encode_ad(LuauOpcode::LOP_JUMP, 0, i16::MIN);
  assert_eq!(insn_min.d(), i16::MIN);

  // 极值正偏移: D = 32767
  let insn_max = Instruction::encode_ad(LuauOpcode::LOP_JUMP, 0, i16::MAX);
  assert_eq!(insn_max.d(), i16::MAX);
}

#[test]
fn test_instruction_field_e_signed() {
  // 正偏移: E = 500
  let insn_pos = Instruction::encode_e(LuauOpcode::LOP_JUMPX, 500);
  assert_eq!(insn_pos.e(), 500);

  // 负偏移: E = -2
  let insn_neg = Instruction::encode_e(LuauOpcode::LOP_JUMPX, -2);
  assert_eq!(insn_neg.e(), -2);

  // 24 位最大值: 0x7FFFFF (8388607)
  let insn_max = Instruction::encode_e(LuauOpcode::LOP_JUMPX, 0x007F_FFFF);
  assert_eq!(insn_max.e(), 0x007F_FFFF);

  // 24 位负极值: -8388608
  let insn_min = Instruction::encode_e(LuauOpcode::LOP_JUMPX, -8388608);
  assert_eq!(insn_min.e(), -8388608);
}

#[test]
fn test_instruction_opcode_mapping() {
  let insn = Instruction::encode_abc(LuauOpcode::LOP_ADD, 1, 2, 3);
  assert_eq!(insn.opcode(), Some(LuauOpcode::LOP_ADD));
  assert_eq!(insn.luau_opcode(), LuauOpcode::LOP_ADD);

  // 超出范围的操作码 (0xFE)
  let invalid_insn = Instruction(0xFE);
  assert_eq!(invalid_insn.opcode(), None);
  assert_eq!(invalid_insn.luau_opcode(), LuauOpcode::LOP_NOP);
}

#[test]
fn test_instruction_aux_fields() {
  // aux_a, aux_b
  let aux = Instruction(0x0000_3412);
  assert_eq!(aux.aux_a(), 0x12);
  assert_eq!(aux.aux_b(), 0x34);

  // aux_kv, aux_kv16
  let aux_k = Instruction(0x0056_789A);
  assert_eq!(aux_k.aux_kv(), 0x0056_789A);
  assert_eq!(aux_k.aux_kv16(), 0x789A);

  // aux_slot
  let aux_s = Instruction(0x1234_5678);
  assert_eq!(aux_s.aux_slot(), 0x1234);

  // aux_kb
  assert_eq!(Instruction(0).aux_kb(), 0);
  assert_eq!(Instruction(1).aux_kb(), 1);

  // aux_not
  assert_eq!(Instruction(0x7FFF_FFFF).aux_not(), 0);
  assert_eq!(Instruction(0x8000_0000).aux_not(), 1);
}

#[test]
fn test_instruction_slice_conversions() {
  let raw_data: [u32; 4] = [0x11, 0x22, 0x33, 0x44];
  let insn_slice = Instruction::from_slice(&raw_data);
  assert_eq!(insn_slice.len(), 4);
  assert_eq!(insn_slice[0].raw(), 0x11);
  assert_eq!(insn_slice[3].raw(), 0x44);

  let raw_slice = Instruction::as_raw_slice(insn_slice);
  assert_eq!(raw_slice, &raw_data);

  let mut raw_mut: [u32; 2] = [10, 20];
  let insn_mut = Instruction::from_slice_mut(&mut raw_mut);
  insn_mut[0] = Instruction(100);
  assert_eq!(raw_mut[0], 100);
}

#[test]
fn test_instruction_bytecode_read() {
  let bytes = 0x1234_5678u32.to_ne_bytes();
  let insn = Instruction::from_bytes(&bytes);
  assert_eq!(insn.raw(), 0x1234_5678);
}

#[test]
fn test_instruction_jump_target() {
  let jump = Instruction::encode_ad(LuauOpcode::LOP_JUMP, 0, 5);
  assert_eq!(jump.jump_target(10), 16);
}
