extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;
pub mod type_aliases;

/// 本 crate 私有 FastFlag（对齐 cpp 侧定义于 Bytecode 域的旗标）
pub mod fflag {
  // cpp: Bytecode 读取链路使用的 LuauCostModel（定义于 VM/src/lvmload.cpp，此处就近定义）
  ulua_common::LUAU_FASTFLAGVARIABLE!(LUAU_BYTECODE_COST_MODEL, LuauCostModel);
}
