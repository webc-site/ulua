//! Source: `Common/include/Luau/Bytecode.h`

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LuauBytecodeTag(pub u32);

impl LuauBytecodeTag {
  /// Bytecode version; runtime supports [MIN, MAX], compiler emits TARGET by default but may emit a higher version when flags are enabled
  pub const LBC_VERSION_MIN: Self = Self(3);
  pub const LBC_VERSION_MAX: Self = Self(14);
  pub const LBC_VERSION_TARGET: Self = Self(9);
  /// `LBC_VERSION_CLASSES`（Bytecode.h:524）：Luau Classes 实验格式，
  /// `getVersion` 在 `DebugLuauUserDefinedClasses` 开启时发出。
  pub const LBC_VERSION_CLASSES: Self = Self(100);

  /// Type encoding version
  pub const LBC_TYPE_VERSION_MIN: Self = Self(1);
  pub const LBC_TYPE_VERSION_MAX: Self = Self(3);
  pub const LBC_TYPE_VERSION_TARGET: Self = Self(3);

  /// Types of constant table entries
  pub const LBC_CONSTANT_NIL: Self = Self(0);
  pub const LBC_CONSTANT_BOOLEAN: Self = Self(1);
  pub const LBC_CONSTANT_NUMBER: Self = Self(2);
  pub const LBC_CONSTANT_STRING: Self = Self(3);
  pub const LBC_CONSTANT_IMPORT: Self = Self(4);
  pub const LBC_CONSTANT_TABLE: Self = Self(5);
  pub const LBC_CONSTANT_CLOSURE: Self = Self(6);
  pub const LBC_CONSTANT_VECTOR: Self = Self(7);
  pub const LBC_CONSTANT_TABLE_WITH_CONSTANTS: Self = Self(8);
  pub const LBC_CONSTANT_INTEGER: Self = Self(9);
  pub const LBC_CONSTANT_CLASS_SHAPE: Self = Self(10);
  pub const LBC_CONSTANT_VECTORD: Self = Self(11);

  /** WARNING: This must always be last. */
  pub const LBC_CONSTANT__COUNT: Self = Self(12);
}
