//! crate 内部共用：取栈顶函数的 `short_src`（coverage/counters dump 的公共前置步，
//! 合并两处逐字重复的 `LuaDebug{}` + `lua_getinfo("s")` + 取串序列）。

use alloc::string::String;

use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

/// `lua_getinfo` 选项串「取 short_src」：review.md §10 后 `what` 形参为选项字节窗
/// （`auxgetinfo` 全窗迭代，无终止 NUL 语义），故这里就是选项本身，不再补 `\0`。
const GETINFO_S_OPT: &[u8] = b"s";

/// `lua_getinfo(l, -1, "s")` 后提取 `short_src`；未回填即空串（对应 cpp 内嵌
/// `char[256]` 取不到即空的观察行为）。
///
/// `l` 为有效 VM 状态且栈顶是一个可 `lua_getinfo` 的函数（调用点守卫）。
// DELIBERATE DEVIATION（review.md §9.3）：`lua_getinfo` 的 c-API 句柄边界——出参
// `&mut LuaDebug` 已是原生记录（`short_src` 为定长 `ShortSrc` 载体，写端一处截断），
// 读面直接取有效字节窗，旧「判空 + NUL 扫描解码」的 `cstr_cow` 折转随指针字段消亡。
pub(crate) fn stack_function_name(l: &mut LuaState) -> String {
  // C++ `LuaDebug ar = {}`：`Default` 逐字段给出「未回填即空」初值（安全、可编译期折叠）
  let mut ar = LuaDebug::default();
  // Safety: `lua_getinfo` 为 unsafe 导出；`l.as_mut_ptr()` 是本帧独占借用的地址，
  // `&mut ar` 是本地独占借用交 lua_getinfo 填充，前提（栈顶可查询的函数）由调用点
  // 守卫成立；`GETINFO_S_OPT` 为只读静态选项窗且不含 `f`，不压栈。
  unsafe { lua_getinfo(l.as_mut_ptr(), -1, GETINFO_S_OPT, &mut ar) };
  // `ShortSrc::bytes()` 即写端截断后的有效字节；lossy 解码与旧 `cstr_cow` 消费面同点
  // （VM 串体可为任意字节，dump 文案面用替换字符兜底）。
  String::from_utf8_lossy(ar.short_src.bytes()).into_owned()
}
