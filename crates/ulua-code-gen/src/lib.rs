extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;
pub mod traits;
pub mod type_aliases;

/// 本 crate 私有 FastFlag（对齐 cpp 侧定义于 CodeGen 域的旗标）
pub mod fflag {
  // cpp: CodeGen/src/IrUtils.cpp propagateTagsFromPredecessors 使用的跳过死前驱开关
  ulua_common::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_SKIP_DEAD_PREDECESSOR_TAGS,
    LuauCodegenSkipDeadPredecessorTags
  );
}
