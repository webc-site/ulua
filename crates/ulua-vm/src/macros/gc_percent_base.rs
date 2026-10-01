/// GC 百分比基准：`gcgoal`/`gcstepmul` 在 global_State 中以「百分数」存储（如 200 即
/// 200%），所有涉及它们的步长/目标换算都要除以该基准。锚 cpp lgc.cpp:1305/1339/1345/1421。
pub const GC_PERCENT_BASE: usize = 100;
