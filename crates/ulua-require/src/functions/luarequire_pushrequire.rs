use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{lua_require::lua_require, push_closure::push_closure},
  records::navigation_context::RequireHost,
};

/// require 闭包的调试名（cpp `debugname`，静态字节窗，**不含终止 NUL**——VM 只存
/// 引用，`dumpclosure` 按全长窗写出，保留 NUL 会在 GC 台账里多一个可见 NUL 字节）。
const REQUIRE_DEBUGNAME: &[u8] = b"require";

/// 建立并压入 require 闭包（cpp `luarequire_pushrequire`），不注册全局。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`（宿主 Lua/C API 句柄，本 crate 归 ulua-vm
/// C-API 真边界裁定）；`host` 装箱进与闭包同寿命的 userdata 后由 GC 终结。
pub unsafe fn luarequire_pushrequire<C: RequireHost + 'static>(l: *mut LuaState, host: C) -> i32 {
  // Safety: l 是宿主按 Lua/C API 提供的有效 LuaState；host 为调用方持有的
  // 静态生命周期值，经 push_closure 装箱移交；`lua_require::<C>` 是本 crate 静态
  // 存活闭包体按同一宿主类型 `C` 的单态化实例（coerce 为 C 函数指针），其运行期
  // 按 upvalue(1) 以 `C` 取回宿主。
  unsafe { push_closure(l, host, Some(lua_require::<C>), REQUIRE_DEBUGNAME) }
}
