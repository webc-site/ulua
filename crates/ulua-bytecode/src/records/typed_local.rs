use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

/// cpp `BytecodeBuilder::TypedLocal` 与 `BcFunction` 图侧 `TypedLocal` 实为同一数据
/// （类型编码 + 寄存器 + PC 区间），abs-r139 合并为单源，不再按 cpp 宿主类拆双份。

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TypedLocal {
  pub(crate) r#type: LuauBytecodeType,
  pub(crate) reg: u8,
  pub(crate) startpc: u32,
  pub(crate) endpc: u32,
}

impl Default for TypedLocal {
  fn default() -> Self {
    Self {
      r#type: LuauBytecodeType::LBC_TYPE_NIL,
      reg: 0,
      startpc: 0,
      endpc: 0,
    }
  }
}
