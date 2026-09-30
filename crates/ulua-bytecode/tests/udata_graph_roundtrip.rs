//! UDATA 版 KS 指令（GETUDATAKS/SETUDATAKS/NAMECALLUDATA）的 graph 往返测试。
//!
//! 对齐 cpp：`BytecodeGraphParser.h:666-691、712-720` 把这三条算子的 aux 拆成
//! 「低 16 位 KV 常量索引（`LUAU_INSN_AUX_KV16`）+ 高 16 位缓存 slot
//! （`LUAU_INSN_AUX_SLOT`）」两条图输入，`BytecodeGraphSerializer.h:397-414、431-439`
//! 再按 `[VmConst, Int]` 布局打包回 aux。拆分写错会让带 slot 缓存的字节码
//! 在往返后拿到错误的常量索引或直接越界。

use ulua_bytecode::{
  enums::bc_op_kind::BcOpKind,
  functions::{
    from_function_bytecode::from_function_bytecode,
    to_function_bytecode_bytecode_graph::to_function_bytecode_comp_time_bc_function,
  },
  records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef},
};
use ulua_common::enums::luau_opcode::LuauOpcode;

/// 函数级字节码 blob 的指令段原始字节。
/// 头部布局：4 字节函数头 + 1 字节类型段标记，其后 VarInt(类型段长度)、
/// VarInt(指令数)、指令字（每条 4 字节）。
fn code_section(blob: &[u8]) -> &[u8] {
  let mut offset = 5usize;

  let read_var_int = |data: &[u8], offset: &mut usize| -> u32 {
    let mut result = 0u32;
    let mut shift = 0u32;
    loop {
      let byte = data[*offset];
      *offset += 1;
      result |= ((byte & 0x7F) as u32) << shift;
      if byte & 0x80 == 0 {
        break;
      }
      shift += 7;
    }
    result
  };

  let type_info_size = read_var_int(blob, &mut offset) as usize;
  offset += type_info_size;
  let code_size = read_var_int(blob, &mut offset) as usize;

  &blob[offset..offset + code_size * size_of::<u32>()]
}

/// 指令段解码为小端指令字。
fn code_words(section: &[u8]) -> Vec<u32> {
  let mut words = Vec::with_capacity(section.len() / size_of::<u32>());
  let mut off = 0usize;
  while off + size_of::<u32>() <= section.len() {
    words.push(u32::from_le_bytes(
      section[off..off + size_of::<u32>()].try_into().unwrap(),
    ));
    off += size_of::<u32>();
  }
  words
}

/// `kv16 | slot << 16`——cpp `VM_PATCH_AUX_SLOT` 的打包布局。
fn packed_aux(kv: u32, slot: u32) -> u32 {
  kv | (slot << 16)
}

#[test]
fn udata_ks_aux_survives_graph_roundtrip() {
  // 1) 用 builder 造一条含三个 UDATA 算子、且每条都带非零缓存 slot 的函数
  let name = b"field".to_vec();
  let mut bcb = BytecodeBuilder::new(None);

  let mid = bcb.begin_function(1, false);
  let sid = bcb.add_constant_string(StringRef::from_slice(&name)) as u32;

  // r0 = "field"
  bcb.emit_ad(LuauOpcode::LOP_LOADK, 0, sid as i16);
  // r1 = r0.field（缓存 slot 7）
  bcb.emit_abc(LuauOpcode::LOP_GETUDATAKS, 1, 0, 0);
  bcb.emit_aux(packed_aux(sid, 7));
  // r0.field = r1（缓存 slot 9）
  bcb.emit_abc(LuauOpcode::LOP_SETUDATAKS, 0, 1, 0);
  bcb.emit_aux(packed_aux(sid, 9));
  // r2/r3 = r0:field()（缓存 slot 11）；NAMECALL 必须紧跟 CALL
  bcb.emit_abc(LuauOpcode::LOP_NAMECALLUDATA, 2, 0, 0);
  bcb.emit_aux(packed_aux(sid, 11));
  bcb.emit_abc(LuauOpcode::LOP_CALL, 2, 1, 2);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 2, 1, 0);

  bcb.end_function(4, 0, 0, 0);
  bcb.set_main_function(mid);

  let blob = bcb.get_function_data(mid);
  let strings = bcb.get_string_table();

  let original_section = code_section(&blob).to_vec();
  let original = code_words(&original_section);
  // 自检：三条 UDATA 的 aux 字确实带着非零 slot，否则本测试退化成空跑
  assert_eq!(original[2], packed_aux(sid, 7), "GETUDATAKS 的 aux 字");
  assert_eq!(original[4], packed_aux(sid, 9), "SETUDATAKS 的 aux 字");
  assert_eq!(original[6], packed_aux(sid, 11), "NAMECALLUDATA 的 aux 字");

  // 2) 解析成 graph：aux 必须被拆成 `[…, VmConst, Int]` 两条输入
  let mut function = from_function_bytecode(&blob, &strings).expect("函数字节码应可解析");

  let mut checked = 0usize;
  for insn in &function.instructions {
    let expected_slot = match insn.op {
      LuauOpcode::LOP_GETUDATAKS => 7,
      LuauOpcode::LOP_SETUDATAKS => 9,
      LuauOpcode::LOP_NAMECALLUDATA => 11,
      _ => continue,
    };
    checked += 1;

    // GETUDATAKS/NAMECALLUDATA: [VmReg, Int, VmConst, Int]
    // SETUDATAKS:               [VmReg, VmReg, Int, VmConst, Int]
    let aux_pos = insn.ops.len() - 2;
    assert_eq!(
      insn.ops[aux_pos].kind,
      BcOpKind::VmConst,
      "倒数第二项应为 KV 常量"
    );
    assert_eq!(
      insn.ops[aux_pos + 1].kind,
      BcOpKind::Imm,
      "末项应为 slot 立即数"
    );
    assert_eq!(insn.ops[aux_pos].index, sid, "KV 常量索引不得混入 slot 位");

    let imm_op = insn.ops[aux_pos + 1];
    let imm = &function.immediates[imm_op.index as usize];
    // 该立即数由 parser 以 Int 写入（add_imm_input_bc_inst_i32）
    let slot = imm.as_int();
    assert_eq!(slot as u32, expected_slot, "slot 应取自 aux 高 16 位");
  }
  assert_eq!(checked, 3, "三条 UDATA 算子都应出现在图中");

  // 3) 序列化回字节码：指令段必须与输入逐字节相同（aux 重新打包回 kv|slot<<16）
  let repacked = to_function_bytecode_comp_time_bc_function(&mut function);
  assert_eq!(original_section, code_section(&repacked));
}
