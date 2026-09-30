extern crate alloc;
use alloc::string::String;
use core::{
  ffi::c_int,
  fmt::{Arguments, Write},
};
use std::iter::repeat_n;

use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::get_op_length::get_op_length,
  macros::luau_insn_ops::luau_insn_op,
};
use ulua_vm::{enums::lua_type::LuaType, records::proto::Proto};

use crate::{
  enums::include_use_info::IncludeUseInfo,
  functions::{
    append_register_set::append_register_set,
    proto_views::{locvars, name_str},
    to_string_ir_dump::{
      to_string as to_string_function, to_string_ir_to_string_context_ir_block_u32,
    },
  },
  records::{
    assembly_state::RegisterSet, block_iterator_wrapper::BlockIteratorWrapper,
    dump::IrToStringContext, ir_block::IrBlock, ir_function::IrFunction,
  },
  type_aliases::ir::Instruction,
};

pub fn dump(function: &mut IrFunction) -> String {
  let result = to_string_function(function, IncludeUseInfo::Yes);

  std::println!("{}", result);

  result
}

pub(crate) fn append(result: &mut String, args: Arguments<'_>) {
  // write! 返回 fmt::Result；截断可接受（C++ 版同样
  // 截断到 256 字节缓冲区）。
  let _ = result.write_fmt(args);
}

const K_DETAILS_ALIGN_COLUMN: i32 = 60;

pub(crate) fn pad_to_detail_column(result: &mut String, line_start: usize) {
  let pad = K_DETAILS_ALIGN_COLUMN - (result.len() as i32 - line_start as i32);
  if pad > 0 {
    // 追加 `pad` 个空格。
    result.extend(repeat_n(' ', pad as usize));
  }
}

pub fn append_label_regset(
  ctx: &mut IrToStringContext,
  reg_sets: &[RegisterSet],
  block_idx: usize,
  name: &str,
) {
  if block_idx < reg_sets.len() {
    let rs = &reg_sets[block_idx];

    if rs.regs.iter().any(|&r| r != 0) || rs.vararg_seq {
      append(ctx.result, format_args!("|{{{}|", name));
      append_register_set(ctx, rs, "|");
      append(ctx.result, format_args!("}}"));
    }
  }
}

pub fn append_block_set(ctx: &mut IrToStringContext, blocks: BlockIteratorWrapper<'_>) {
  for (index, target) in blocks.enumerate() {
    if index != 0 {
      append(ctx.result, format_args!(", "));
    }

    let block: &IrBlock = &ctx.blocks[target as usize];
    to_string_ir_to_string_context_ir_block_u32(ctx, block, target);
  }
}

/// cpp `IrDump` 的 tag 名查表（"tnil"/"tboolean"...）。
/// 判别式与名字的单一真相在 [`LuaType::tag_name`]。
pub(crate) fn get_tag_name(tag: u8) -> &'static str {
  match LuaType::from_c_int(tag as c_int) {
    Some(ty) => ty.tag_name(),
    None => unreachable!("Unknown type tag"),
  }
}

/// cpp `tryFindLocalName`：按 (寄存器号, pc) 在局部变量表里查首个命中的变量名。
///
/// 契约：`proto` 指向存活 Proto；返回的 `&str` 寿命随该借用（名字串随 Proto 可达）。
pub(crate) fn try_find_local_name(proto: &Proto, reg: i32, pcpos: i32) -> Option<&str> {
  // 顺序扫描命中首个匹配即返回（对齐 cpp 语义）；表为空时自然迭代零次。
  locvars(proto)
    .iter()
    .find(|local| reg == local.reg as i32 && pcpos >= local.startpc && pcpos < local.endpc)
    .and_then(|local| name_str(local.varname as *const _, proto))
}

pub fn get_instruction_count(insns: &[Instruction]) -> u32 {
  let mut count: u32 = 0;
  let mut i: usize = 0;

  while i < insns.len() {
    count += 1;
    let op = luau_insn_op(insns[i]) as u8;
    let op_enum = LuauOpcode::from(op);
    let len = get_op_length(op_enum).max(1) as usize;
    i += len;
  }

  count
}

pub fn is_printable_string_constant(bytes: &[u8]) -> bool {
  bytes.iter().all(|&b| b >= b' ')
}
