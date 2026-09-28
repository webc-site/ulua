use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TypedUpval {
  pub(crate) r#type: LuauBytecodeType,
}
