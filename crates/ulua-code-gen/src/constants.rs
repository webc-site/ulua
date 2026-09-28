//! CodeGen 域内常量（不属于 VM 枚举、仅在 IR/native context 派发中出现的值）。

/// cpp/CodeGen/include/Luau/IrData.h:504 `LBF_IR_MATH_LOG2 = 256`：
/// CodeGen 内部虚拟 builtin id（math.log(x, 2) 折叠为 libm_log2 调用），
/// 超出 VM 侧 LuauBuiltinFunction 枚举范围，故与 LBF_* 共用 id 空间但单独定义。
pub(crate) const LBF_IR_MATH_LOG2: i32 = 256;
