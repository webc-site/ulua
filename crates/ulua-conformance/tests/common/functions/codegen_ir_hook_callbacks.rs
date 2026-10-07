//! cpp `ConformanceIrHooks.h` 各 IR hook 的 Rust 壳：唯一职责是把 hook 交付的
//! 成员名字节切片（`&[u8]`）解码为 callee 使用的 `&str`。metamethod 两钩子不带
//! 成员名，callee 签名即 hook 签名，测试直接挂原函数，不经本文件。

use core::str::from_utf8;

use ulua_code_gen::records::ir_builder::IrBuilder;

use crate::common::functions::{
  userdata_access::userdata_access, userdata_access_bytecode_type::userdata_access_bytecode_type,
  userdata_namecall::userdata_namecall,
  userdata_namecall_bytecode_type::userdata_namecall_bytecode_type, vector_access::vector_access,
  vector_access_bytecode_type::vector_access_bytecode_type, vector_namecall::vector_namecall,
  vector_namecall_bytecode_type::vector_namecall_bytecode_type,
};

/// hook 成员名字节切片 → `&str`；非法 UTF-8 按空名处理（与旧壳 `unwrap_or("")` 一致）。
#[inline]
fn member_str(member: &[u8]) -> &str {
  from_utf8(member).unwrap_or("")
}

/// `vectorAccessBytecodeType` 壳。
pub fn vector_access_bytecode_type_callback(member: &[u8]) -> u8 {
  vector_access_bytecode_type(member_str(member))
}

/// `vectorNamecallBytecodeType` 壳。
pub fn vector_namecall_bytecode_type_callback(member: &[u8]) -> u8 {
  vector_namecall_bytecode_type(member_str(member))
}

/// `userdataAccessBytecodeType` 壳。
pub fn userdata_access_bytecode_type_callback(r#type: u8, member: &[u8]) -> u8 {
  userdata_access_bytecode_type(r#type, member_str(member))
}

/// `userdataNamecallBytecodeType` 壳。
pub fn userdata_namecall_bytecode_type_callback(r#type: u8, member: &[u8]) -> u8 {
  userdata_namecall_bytecode_type(r#type, member_str(member))
}

/// `vectorAccess` 壳。
pub fn vector_access_callback(
  build: &mut IrBuilder,
  member: &[u8],
  result_reg: i32,
  source_reg: i32,
  pcpos: i32,
) -> bool {
  vector_access(build, member_str(member), result_reg, source_reg, pcpos)
}

/// `vectorNamecall` 壳。
pub fn vector_namecall_callback(
  build: &mut IrBuilder,
  member: &[u8],
  arg_res_reg: i32,
  source_reg: i32,
  params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  vector_namecall(
    build,
    member_str(member),
    arg_res_reg,
    source_reg,
    params,
    results,
    pcpos,
  )
}

/// `userdataAccess` 壳。
pub fn userdata_access_callback(
  build: &mut IrBuilder,
  r#type: u8,
  member: &[u8],
  result_reg: i32,
  source_reg: i32,
  pcpos: i32,
) -> bool {
  userdata_access(
    build,
    r#type,
    member_str(member),
    result_reg,
    source_reg,
    pcpos,
  )
}

/// `userdataNamecall` 壳。
pub fn userdata_namecall_callback(
  build: &mut IrBuilder,
  r#type: u8,
  member: &[u8],
  arg_res_reg: i32,
  source_reg: i32,
  params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  userdata_namecall(
    build,
    r#type,
    member_str(member),
    arg_res_reg,
    source_reg,
    params,
    results,
    pcpos,
  )
}
