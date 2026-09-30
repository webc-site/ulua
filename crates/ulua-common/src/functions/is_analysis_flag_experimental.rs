/// cpp/Common/include/Luau/ExperimentalFlags.h 的 `isAnalysisFlagExperimental`：
/// 列表内 flag 尚未定稿或存在已知缺陷，命令行工具默认不开启。
pub fn is_analysis_flag_experimental(flag: &str) -> bool {
  const K_LIST: &[&str] = &[
    "LuauInstantiateInSubtyping",      // 需先修 lua-apps 侧问题
    "LuauFixIndexerSubtypingOrdering", // 修复假阴性，需先修 lua-apps 侧问题
    "LuauSolverV2",
    "UseNewLuauTypeSolverDefaultEnabled", // 会改变 cli 工具默认求解器
    "LuauRefactorStringSemanticSubtyping", // 需先修 lua-apps 侧问题
  ];

  K_LIST.contains(&flag)
}
