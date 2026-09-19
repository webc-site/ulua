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
  // cpp: CodeGen/src/OptimizeConstProp.cpp 收缩替换后仍记录 CSE 映射
  ulua_common::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_SUBSTITUTE_REPLACEMENTS,
    LuauCodegenSubstituteReplacements
  );
  // cpp: CodeGen/src/OptimizeConstProp.cpp 合并块间传播 fallback 前驱的 tag
  ulua_common::LUAU_FASTFLAGVARIABLE!(
    LUAU_CODEGEN_PROPAGATE_FALLBACK_TAGS,
    LuauCodegenPropagateFallbackTags
  );
}
