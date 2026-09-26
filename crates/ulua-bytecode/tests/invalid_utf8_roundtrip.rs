//! B2 回归：非法 UTF-8 字符串常量必须逐字节回环（cpp `readString` 是非拥有原始视图）。
//!
//! 旧实现在 `fromFunctionBytecode` 走 `String::from_utf8_lossy` + 泄漏：
//! 非法字节被改写成 U+FFFD，回写常量表即永久污染、去重键失真。
//! 现在字符串常量以 `&'static [u8]` 保留原始字节。

use ulua_bytecode::{
  enums::bc_vm_const_kind::BcVmConstKind,
  functions::{
    from_function_bytecode::from_function_bytecode,
    to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  },
  records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef},
};
use ulua_common::enums::luau_opcode::LuauOpcode;

#[path = "common/leak_bytes.rs"]
mod leak_bytes_support;

use leak_bytes_support::leak_bytes;

/// 含非法起始字节、孤立续字节与内嵌 NUL 的字节串。
const ILLEGAL: &[u8] = b"\xff\xfe\x80not-utf8\x00end";

#[test]
fn invalid_utf8_string_constant_roundtrips_byte_for_byte() {
  let leaked = leak_bytes(ILLEGAL);

  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  let cid = bcb.add_constant_string(StringRef::from_slice(leaked));
  // write_line_info 要求指令/行号表非空：发一条 LOADNIL r0 占位
  bcb.emit_ad(LuauOpcode::LOP_LOADNIL, 0, 0);
  bcb.end_function(1, 0, 0, 0);

  let data = bcb.get_function_data(0);
  let strings = bcb.get_string_table();
  assert_eq!(strings[cid as usize], ILLEGAL, "字符串表槽位必须原样存字节");

  let mut graph = from_function_bytecode(&data, &strings).expect("自产函数块必须可解析");
  let c = graph.constants[cid as usize];
  assert_eq!(c.kind(), BcVmConstKind::String);
  let parsed = c.as_string();
  assert_eq!(parsed, ILLEGAL, "解析不得把非法 UTF-8 改写为 U+FFFD");

  // 回写→再解析：逐字节守恒（to_function 走同一 &[u8] 通道）
  let mut bcb2 = BytecodeBuilder::new(None);
  let data2 = to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut bcb2, &mut graph);
  assert!(!data2.is_empty(), "回写函数块不得为空");
  assert_eq!(data2, data, "同一图两次序列化必须逐字节一致");

  let strings2 = bcb2.get_string_table();
  let graph2 = from_function_bytecode(&data2, &strings2).expect("回写函数块必须可再解析");
  let parsed2 = graph2.constants[0].as_string();
  assert_eq!(parsed2, ILLEGAL, "再解析后字节仍须原样");
}

#[test]
fn distinct_invalid_byte_strings_do_not_collide_after_lossy() {
  // 两个不同的非法字节串：lossy 会把它们改写成同一个 U+FFFD 序列导致去重碰撞，
  // 按字节保留则必须是两条不同常量。
  let a = leak_bytes(b"\xff\xfe");
  let b = leak_bytes(b"\xf0\x90");

  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  let ca = bcb.add_constant_string(StringRef::from_slice(a));
  let cb = bcb.add_constant_string(StringRef::from_slice(b));

  assert_ne!(ca, cb, "不同字节的非法串不得因 lossy 归并");
}
