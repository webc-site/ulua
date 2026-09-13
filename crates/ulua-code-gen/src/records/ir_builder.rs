//! Node: `cxx:Record:Luau.CodeGen:CodeGen/include/Luau/IrBuilder.h:21:ir_builder`
//! Source: `CodeGen/include/Luau/IrBuilder.h` (IrBuilder.h:21-..., hand-ported; fields only)

use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{host_ir_hooks::HostIrHooks, ir_function::IrFunction, ir_op::IrOp},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopInfo {
  pub step: IrOp,
  pub startpc: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstantKey {
  pub kind: IrConstKind,
  pub value: u64,
}

#[derive(Debug)]
pub struct IrBuilder {
  pub host_hooks: *const HostIrHooks, // const HostIrHooks&
  pub in_terminated_block: bool,
  pub interrupt_requested: bool,

  pub active_fastcall_fallback: bool,
  pub fastcall_fallback_return: IrOp,
  pub cmd_skip_target: i32,

  pub function: IrFunction,

  pub active_block_idx: u32,

  /// Block index at the bytecode instruction
  pub inst_index_to_block: Vec<u32>,

  pub numeric_loop_stack: Vec<LoopInfo>,

  pub constant_map: DenseHashMap<ConstantKey, u32>,
}
