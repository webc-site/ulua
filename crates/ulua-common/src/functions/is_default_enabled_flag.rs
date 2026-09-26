use crate::functions::is_analysis_flag_experimental::is_analysis_flag_experimental;

/// cpp `setLuauFlagsDefault`（CLI/src/Flags.cpp）的开关谓词：
/// 仅 `Luau*` 前缀且非实验性的 flag 随默认开启。
///
/// 已知偏差：`LuauExportValueSyntax`/`LuauExportValueTypecheck`
/// 的 export 语法在本移植中会错编 Closure 捕获的 exported local（upvalue 寄存器，
/// 可能产生越界字节码），修复 codegen 前一律保持关闭。
pub fn is_default_enabled_flag(name: &str) -> bool {
  name.starts_with("Luau")
    && !is_analysis_flag_experimental(name)
    && name != "LuauExportValueSyntax"
    && name != "LuauExportValueTypecheck"
}
