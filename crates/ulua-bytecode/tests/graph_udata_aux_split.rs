//! [findings#6] UDATA 常量 aux 拆分回归（cpp `BytecodeGraphParser.h:666-691/712-720`）。
//!
//! UDATA 变体（GETUDATAKS/SETUDATAKS/NAMECALLUDATA）的 aux 字携带
//! `KV16 常量索引 | SLOT << 16`，建图时必须拆成 VmConst + Imm 两个输入，
//! 与序列化端 `emit_ks_aux`（按 Aux16 常量 + flags imm 读回）成对；
//! 旧实现只挂了一个整 aux 的 VmConst，序列化端按位取第 4 个输入即错位。

use ulua_bytecode::{
  enums::bc_op_kind::BcOpKind,
  functions::{
    from_function_bytecode::from_function_bytecode,
    to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  },
  records::{
    bc_function::BcFunction, bc_inst::BcInst, bc_op::BcOp, bytecode_builder::BytecodeBuilder,
    string_ref::StringRef,
  },
};
use ulua_common::enums::luau_opcode::LuauOpcode;

#[path = "common/leak_bytes.rs"]
mod leak_bytes_support;

use leak_bytes_support::leak_bytes;

const FIELD_NAME: &[u8] = b"field";

/// 在图里找唯一使用 `op` 的指令。
fn find_inst<'a>(graph: &'a BcFunction<'a>, op: LuauOpcode) -> &'a BcInst {
  graph
    .instructions
    .iter()
    .find(|inst| inst.op == op)
    .unwrap_or_else(|| panic!("图中应存在 {op:?} 指令"))
}

fn imm_int(graph: &BcFunction<'_>, op: BcOp) -> i32 {
  use ulua_bytecode::enums::bc_imm_kind::BcImmKind;
  assert_eq!(graph.immediates[op.index as usize].kind(), BcImmKind::Int);
  graph.immediates[op.index as usize].as_int()
}

/// GETUDATAKS/SETUDATAKS：aux 必须拆成 KV16 常量 + SLOT imm 两个输入，且整体回环逐字节一致。
#[test]
fn getudataks_and_setudataks_aux_split_in_graph_and_roundtrip() {
  let name = leak_bytes(FIELD_NAME);

  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  let cid = bcb.add_constant_string(StringRef::from_slice(name)) as u32;
  bcb.emit_ad(LuauOpcode::LOP_LOADNIL, 0, 0);
  // GETUDATAKS r1 r0, aux = cid | slot(5) << 16
  bcb.emit_abc(LuauOpcode::LOP_GETUDATAKS, 1, 0, 0);
  bcb.emit_aux(cid | (5 << 16));
  // SETUDATAKS r0 r0, aux = cid | slot(6) << 16
  bcb.emit_abc(LuauOpcode::LOP_SETUDATAKS, 0, 0, 0);
  bcb.emit_aux(cid | (6 << 16));
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 1, 2, 0);
  bcb.end_function(2, 0, 0, 0);

  let data = bcb.get_function_data(0);
  let strings = bcb.get_string_table();
  let mut graph = from_function_bytecode(&data, &strings).expect("自产函数块必须可解析");

  let get = find_inst(&graph, LuauOpcode::LOP_GETUDATAKS);
  let get_ops: Vec<BcOp> = get.ops.as_slice().to_vec();
  // 首输入在建图期已把寄存器解析为生产者（此处 LOADNIL → Inst），后三个固定为
  // [Imm(C), VmConst(KV16), Imm(SLOT)]
  assert_eq!(get_ops.len(), 4, "GETUDATAKS 须有 4 个输入（对齐 cpp）");
  assert_eq!(get_ops[1].kind, BcOpKind::Imm);
  assert_eq!(get_ops[2].kind, BcOpKind::VmConst);
  assert_eq!(get_ops[3].kind, BcOpKind::Imm);
  assert_eq!(get_ops[2].index, cid, "KV16 输入须为低 16 位常量索引");
  assert_eq!(imm_int(&graph, get_ops[3]), 5, "SLOT 输入须为 aux 高 16 位");

  let set = find_inst(&graph, LuauOpcode::LOP_SETUDATAKS);
  let set_ops: Vec<BcOp> = set.ops.as_slice().to_vec();
  assert_eq!(set_ops.len(), 5, "SETUDATAKS 须有 5 个输入（对齐 cpp）");
  assert_eq!(set_ops[3].kind, BcOpKind::VmConst);
  assert_eq!(set_ops[4].kind, BcOpKind::Imm);
  assert_eq!(set_ops[3].index, cid, "SETUDATAKS KV16 输入须为常量索引");
  assert_eq!(imm_int(&graph, set_ops[4]), 6, "SETUDATAKS SLOT 输入须为 6");

  // 回环：序列化端 `emit_ks_aux` 以 KV16|SLOT<<16 重组 aux，须逐字节还原
  let mut bcb2 = BytecodeBuilder::new(None);
  let data2 = to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut bcb2, &mut graph);
  assert_eq!(data2, data, "UDATA aux 拆分后回写必须逐字节一致");
}

/// NAMECALLUDATA：同样拆成 KV16 常量 + SLOT imm，并在 CALL 配对下完成回环。
#[test]
fn namecalludata_aux_split_in_graph_and_roundtrip() {
  let name = leak_bytes(FIELD_NAME);

  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  let cid = bcb.add_constant_string(StringRef::from_slice(name)) as u32;
  bcb.emit_ad(LuauOpcode::LOP_LOADNIL, 0, 0);
  // NAMECALLUDATA r0, aux = cid | slot(7) << 16；其后必须紧跟 CALL
  bcb.emit_abc(LuauOpcode::LOP_NAMECALLUDATA, 0, 0, 0);
  bcb.emit_aux(cid | (7 << 16));
  bcb.emit_abc(LuauOpcode::LOP_CALL, 0, 1, 1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
  bcb.end_function(3, 0, 0, 0);

  let data = bcb.get_function_data(0);
  let strings = bcb.get_string_table();
  let mut graph = from_function_bytecode(&data, &strings).expect("自产函数块必须可解析");

  let call_site = find_inst(&graph, LuauOpcode::LOP_NAMECALLUDATA);
  let ops: Vec<BcOp> = call_site.ops.as_slice().to_vec();
  // 同 GETUDATAKS：首输入解析为生产者（LOADNIL → Inst），后三个 [Imm, VmConst, Imm]
  assert_eq!(ops.len(), 4, "NAMECALLUDATA 须有 4 个输入（对齐 cpp）");
  assert_eq!(ops[2].kind, BcOpKind::VmConst);
  assert_eq!(ops[3].kind, BcOpKind::Imm);
  assert_eq!(ops[2].index, cid, "NAMECALLUDATA KV16 输入须为常量索引");
  assert_eq!(imm_int(&graph, ops[3]), 7, "NAMECALLUDATA SLOT 输入须为 7");

  let mut bcb2 = BytecodeBuilder::new(None);
  let data2 = to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut bcb2, &mut graph);
  assert_eq!(data2, data, "NAMECALLUDATA aux 拆分后回写必须逐字节一致");
}
