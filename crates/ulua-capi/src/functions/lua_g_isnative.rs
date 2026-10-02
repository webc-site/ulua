//! 本文件对应 `ulua_luaG_isnative` 导出符号（源：ulua-vm/src/functions/lua_g_isnative.rs）。
//! vm 侧已前移 `&LuaState` 引用形，无法再由 `functions/shells.rs` 中透传裸形的共用宏
//! `capi_shell_l_int!` 直呼（宏体语义不得改，同族其余透传壳零行为变化），故本壳从宏
//! 模板退役、按本树 `functions/lua_xmove.rs`/`lua_status.rs` 显式壳先例写
//! `extern "C-unwind"` 一行调用：唯一差异是在本帧把 `l` 重建为只读引用后转调。
use core::ffi::c_int;

use ulua_vm::{functions::lua_g_isnative, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_luaG_isnative`），除把 `l` 在本帧重建为只读引用外，仅
/// 透传至 `ulua_vm::functions::lua_g_isnative::lua_g_isnative(&*l, level)`，零业务
/// 逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该
///   状态的其它访问单线程驱动（不得跨 OS 线程并发）——`&*l` 引用重建前提（宏壳旧契约
///   同款首条「非空」，判空守卫于引用构造前无新增位：本壳与宏壳旧形一致地不加运行期
///   判空，违约即同判 UB，行为逐位中性）；
/// - `level`：值型参数（帧深度），越界由被调方短路返回 0；
/// - 其余安全前置条件与被调函数的文档契约一致。
#[unsafe(export_name = "ulua_luaG_isnative")]
pub unsafe extern "C-unwind" fn lua_g_isnative(l: *mut LuaState, level: c_int) -> c_int {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`，本帧 `&*l` 重建即时结束
  // 借用窗口；被调方只读 ci/base_ci 帧槽指针算术，不写状态、不重入。
  lua_g_isnative::lua_g_isnative(unsafe { &*l }, level)
}
