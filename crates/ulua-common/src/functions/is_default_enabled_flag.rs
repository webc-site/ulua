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
    && !matches!(
      name,
      "LuauBackedgeHeapCheck" | "LuauCallFeedback" | "LuauEmitCallFeedback"
        // SETTABLE 哈希直插（GetHashNodeAddrNum 三 op）：nsieve 整负载证伪零收益，
        // 且 x64 臂 SIGSEGV 未根除（CI bench/windows 双炸），回炉前默认关——
        // 代码保留（LuauJitSettableHashInline 旗标可显式打开）。
        | "LuauJitSettableHashInline"
        // FORN trace 层（阶段二 PoC）：PoC 级验证期默认关（--fflag 双态 A/B 纪律）。
        | "LuauJitFornTrace"
        // FORN trace 内 FMA 折叠：单舍入 ≠ 双舍入，默认关保逐位红线（旗标隔离）。
        | "LuauTraceFmaFold"
        // 新式紧凑子类型原因渲染：cpp 默认 false 且非实验性，随 setLuauFlagsDefault
        // 自动点亮；本移植默认刻意保持关，使 `ulua-analyze` 裸跑复现 cpp flags-off
        // 渲染（golden 套件 flags-off 形态基线，禁为凑绿翻默认）。flags-on 紧凑形态
        // 经 render_type_path 可达，由 --fflags=true / --fflags=LuauNewTypePathErrorMessages 显式开启。
        | "LuauNewTypePathErrorMessages"
    )
}
