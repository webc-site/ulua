//! [findings T1] 损坏字节码的负向回归（cpp `tests/BytecodeCompiler.test.cpp:114-200`）。
//!
//! `from_function_bytecode` 的契约：输入不可信，任何截断/非法 tag/越界计数
//! 一律以 `None` 收口、不得 panic（cpp 上游对应路径多为 release no-op 的
//! `LUAU_ASSERT`）。既有 tests/ 只覆盖自产 blob 的正向回环，本文件补齐
//! 负向面，同时守住建图期越界常量索引的 `error` 位收口（S3）。

use ulua_bytecode::functions::from_function_bytecode::from_function_bytecode;
use ulua_common::enums::{luau_bytecode_tag::LuauBytecodeTag, luau_opcode::LuauOpcode};

/// LEB128 无符号变长整数（与 `try_read_var_int_64` 的读法成对）。
fn var_int(v: u32) -> Vec<u8> {
  let mut v = v as u64;
  let mut out = Vec::new();
  loop {
    let byte = (v & 0x7f) as u8;
    v >>= 7;
    out.push(if v == 0 { byte } else { byte | 0x80 });
    if v == 0 {
      return out;
    }
  }
}

/// 编码一条指令：op 占低 8 位，A 占 8..15，`d` 占 16..31（与 LOADK/LOADN/RETURN 语义一致）。
fn word(op: LuauOpcode, a: u8, d: u32) -> Vec<u8> {
  ((op as u32) | ((a as u32) << 8) | (d << 16))
    .to_le_bytes()
    .to_vec()
}

/// 手工拼一个最小合法函数块：lineinfo/debuginfo 均关闭，逐字段被读取端消费，
/// 因此任意真前缀都必须解析失败。
fn minimal_blob(code: &[u8], const_bytes: &[u8], sizek: u32) -> Vec<u8> {
  let mut v = vec![1u8, 0, 0, 0, 0]; // maxstacksize, numparams, nups, is_vararg, flags
  v.extend(var_int(0)); // types_size
  v.extend(var_int((code.len() / 4) as u32)); // codesize（字长）
  v.extend_from_slice(code);
  v.extend(var_int(sizek));
  v.extend_from_slice(const_bytes);
  v.extend(var_int(0)); // protos size
  v.extend(var_int(1)); // linedefined
  v.extend(var_int(0)); // debugname：string_id 0 == 空串
  v.push(0); // lineinfo 关闭
  v.push(0); // debuginfo 关闭
  v
}

fn number_const() -> Vec<u8> {
  let mut v = vec![LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8];
  v.extend(1.0f64.to_le_bytes());
  v
}

/// 自拼的最小 blob 必须解析成功（否则下面的负向断言全是空转）。
#[test]
fn handcrafted_minimal_blob_parses() {
  let code = [
    word(LuauOpcode::LOP_LOADN, 0, 2),
    word(LuauOpcode::LOP_RETURN, 0, 2),
  ]
  .concat();
  let blob = minimal_blob(&code, &number_const(), 1);
  assert!(from_function_bytecode(&blob, &[]).is_some());
}

/// 逐字节截断：blob 的每一段都是必读字段，任意真前缀都须以 `None` 收口。
#[test]
fn truncated_blob_returns_none_for_every_prefix() {
  let code = [
    word(LuauOpcode::LOP_LOADN, 0, 2),
    word(LuauOpcode::LOP_RETURN, 0, 2),
  ]
  .concat();
  let blob = minimal_blob(&code, &number_const(), 1);
  for k in 0..blob.len() {
    assert!(
      from_function_bytecode(&blob[..k], &[]).is_none(),
      "前缀长度 {k}/{} 必须判为损坏字节码",
      blob.len()
    );
  }
}

/// 非法常量 tag：cpp 原址只有 `LUAU_ASSERT`，这里必须 `None` 而非 debug 断言中断。
#[test]
fn unknown_constant_tag_returns_none() {
  let blob = minimal_blob(&[], &[200], 1);
  assert!(from_function_bytecode(&blob, &[]).is_none());
}

/// table shape length=33（> `TableShape::K_MAX_LENGTH`）：越界计数须 `None`，
/// 不得按 cpp 栈数组式写法越界写。
#[test]
fn table_shape_length_overflow_returns_none() {
  let mut consts = vec![LuauBytecodeTag::LBC_CONSTANT_TABLE.0 as u8];
  consts.extend(var_int(33));
  let blob = minimal_blob(&[], &consts, 1);
  assert!(from_function_bytecode(&blob, &[]).is_none());
}

/// LOADK 引用越界常量索引（sizek=1 但 d=7）：建图期 `add_vm_const_input`
/// 以 `error` 位收口，`rebuild_graph` 返回 `None` → 整体 `None`（S3 回归）。
#[test]
fn loadk_out_of_range_constant_index_returns_none() {
  let code = [
    word(LuauOpcode::LOP_LOADK, 0, 7),
    word(LuauOpcode::LOP_RETURN, 0, 2),
  ]
  .concat();
  let blob = minimal_blob(&code, &number_const(), 1);
  assert!(from_function_bytecode(&blob, &[]).is_none());
}
