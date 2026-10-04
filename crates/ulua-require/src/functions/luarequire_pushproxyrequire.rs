use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{lua_proxyrequire::lua_proxyrequire, push_closure::push_closure},
  records::navigation_context::RequireHost,
};

/// proxyrequire 闭包的调试名（NUL 结尾静态字节串）。
const PROXY_REQUIRE_DEBUGNAME: &[u8] = b"proxyrequire\0";

/// 建立并压入 proxyrequire 闭包（cpp `luarequire_pushproxyrequire`）：以
/// `(path, requirerChunkname)` 两参按既有模块视角解析路径。
///
/// 保留 `unsafe fn` + 裸句柄形参的裁定（review.md §2 判定 1）：本导出是跨 crate
/// `pub` 的宿主入口（`ulua-cli-test/tests/require_by_string.rs` 直接以 `*mut
/// LuaState` 调用），改形需同步该禁区调用点；函数体解引用 `l`（重建独占借用后交给
/// 已降形的 [`push_closure`]），故契约仍须由签名强制。收形仅下沉到体内：裸指针的
/// 解引用点收敛为一次物化，其后的装箱/挂闭包全在安全 `fn` 里完成。
///
/// # Safety
/// - `l`：必须指向存活的 `LuaState`（宿主 Lua/C API 句柄），且在本次调用窗口内无人
///   并发可变借用；调用后栈顶新增一个闭包值，由调用方负责配平。
/// - `host`：须为 `C: 'static` 的静态生命周期值，装箱进与闭包同寿命的 userdata 后由
///   GC 终结；`lua_proxyrequire::<C>` 与本 `C` 同源单态化。
pub unsafe fn luarequire_pushproxyrequire<C: RequireHost + 'static>(
  l: *mut LuaState,
  host: C,
) -> i32 {
  // Safety: 契约保证 l 为存活独占句柄，此处一次物化为 `&mut LuaState`；
  // push_closure 为安全 fn，其调用序契约（debugname 静态 NUL 串、受保护帧、
  // 函数指针与 C 同源单态化）在本调用点逐项成立。
  push_closure(
    unsafe { &mut *l },
    host,
    Some(lua_proxyrequire::<C>),
    PROXY_REQUIRE_DEBUGNAME,
  )
}
