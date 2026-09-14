use ulua_vm::records::lua_table::LuaTable;

use crate::{
  enums::size_x_64::SizeX64,
  functions::{byte_reg::byte_reg, dword_reg::dword_reg},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

const K_STACK_OFFSET_TO_LOCALS: i32 = 16 + 32;
const SIZEOF_INSTRUCTION: i32 = 4;
// EmitCommon.h: kOffsetOfInstructionC = 3, kLuaNodeSizeLog2 = 5
const K_OFFSET_OF_INSTRUCTION_C: i32 = 3;
const K_LUA_NODE_SIZE_LOG2: i32 = 5;

fn s_code() -> OperandX64 {
  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS + 8,
  )
}

pub fn get_table_node_at_cached_slot(
  build: &mut AssemblyBuilderX64,
  tmp: RegisterX64,
  node: RegisterX64,
  table: RegisterX64,
  pcpos: i32,
) {
  CODEGEN_ASSERT!(tmp != node);
  CODEGEN_ASSERT!(table != node);

  build.mov(
    OperandX64::reg(node),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      table,
      core::mem::offset_of!(LuaTable, node) as i32,
    ),
  );

  // compute cached slot
  build.mov(OperandX64::reg(tmp), s_code());
  build.movzx(
    dword_reg(tmp),
    OperandX64::mem(
      SizeX64::Byte,
      RegisterX64::NOREG,
      1,
      tmp,
      pcpos * SIZEOF_INSTRUCTION + K_OFFSET_OF_INSTRUCTION_C,
    ),
  );
  build.and_(
    OperandX64::reg(byte_reg(tmp)),
    OperandX64::mem(
      SizeX64::Byte,
      RegisterX64::NOREG,
      1,
      table,
      core::mem::offset_of!(LuaTable, nodemask8) as i32,
    ),
  );

  // LuaNode* n = &h->node[slot];
  build.shl(
    OperandX64::reg(dword_reg(tmp)),
    OperandX64::imm(K_LUA_NODE_SIZE_LOG2),
  );
  build.add(OperandX64::reg(node), OperandX64::reg(tmp));
}
