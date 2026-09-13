use alloc::{string::String, vec::Vec};

use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::{
  records::{
    bc_block::BcBlock, bc_imm::BcImm, bc_inst::BcInst, bc_op::BcOp, bc_phi::BcPhi, bc_proj::BcProj,
    bc_vm_const::BcVmConst, debug_local_bytecode_graph::DebugLocal, table_shape::TableShape,
    typed_local_bytecode_graph::TypedLocal,
  },
  type_aliases::reg_map::RegMap,
};

pub type VmConst = BcVmConst;

#[derive(Debug, Clone, Default)]
pub struct BcFunction {
  pub maxstacksize: u8,
  pub numparams: u8,
  pub nups: u8,
  pub is_vararg: bool,
  pub flags: u8,

  pub blocks: Vec<BcBlock>,
  pub instructions: Vec<BcInst>,
  pub constants: Vec<VmConst>,
  pub immediates: Vec<BcImm>,
  pub phis: Vec<BcPhi>,
  pub projections: Vec<BcProj>,
  pub table_shapes: Vec<TableShape>,

  pub entry_block: BcOp,
  pub exit_block: BcOp,

  pub type_info: String,
  pub upvalue_types: Vec<LuauBytecodeType>,
  pub local_types: Vec<TypedLocal>,
  pub protos: Vec<u32>,

  pub debugname: String,
  pub linedefined: u32,
  pub upvalue_names: Vec<String>,
  pub locals: Vec<DebugLocal<'static>>,

  pub regs: RegMap,
}
