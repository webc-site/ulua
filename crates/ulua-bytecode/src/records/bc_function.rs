use std::{collections::HashMap, string::String, vec::Vec};

use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::{
  records::{
    bc_block::BcBlock, bc_imm::BcImm, bc_inst::BcInst, bc_op::BcOp, bc_op_hash::BcOpHash,
    bc_phi::BcPhi, bc_proj::BcProj, bc_vm_const::BcVmConst, class_shape::ClassShape,
    debug_local_bytecode_graph::DebugLocal, table_shape::TableShape,
    typed_local_bytecode_graph::TypedLocal,
  },
  type_aliases::reg::Reg,
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
  /// cpp: `BcFunction::classShapes`（`cpp/Bytecode/include/Luau/BytecodeGraph.h:415`）
  pub class_shapes: Vec<ClassShape>,

  pub entry_block: BcOp,
  pub exit_block: BcOp,

  /// cpp `std::string typeInfo`：类型编码字节流（LBC_TYPE_* 原始字节）。
  pub type_info: Vec<u8>,
  pub upvalue_types: Vec<LuauBytecodeType>,
  pub local_types: Vec<TypedLocal>,
  pub protos: Vec<u32>,

  pub debugname: String,
  pub linedefined: u32,
  pub upvalue_names: Vec<String>,
  pub locals: Vec<DebugLocal>,

  /// cpp `BcFunction::regs` 即 `std::unordered_map<BcOp, Reg, BcOpHash>`：
  /// 保持 std HashMap 而非 DenseHashMap——键域内不存在可证明不可达的哨兵值
  /// （kind 判别最小的合法 op 也可能全零），硬选哨兵会重演 B3 的相撞风险。
  pub regs: HashMap<BcOp, Reg, BcOpHash>,
}
