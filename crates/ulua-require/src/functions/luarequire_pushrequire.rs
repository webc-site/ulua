use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{lua_require::lua_require, push_closure::push_closure},
  records::navigation_context::RequireHost,
};

/// require 闭包的调试名（NUL 结尾静态字节串，仅 `lua_pushcclosurek` 收口点转 C 指针）。
const REQUIRE_DEBUGNAME: &[u8] = b"require\0";

/// 建立并压入 require 闭包（cpp `luarequire_pushrequire`），不注册全局。
///
/// 收形（review.md §2）：`l` 的存活与独占前提已由 `&mut LuaState` 引用形承载，
/// `push_closure` 亦已降为安全 `fn`，故本函数随之为安全 `fn`——签名上的 `unsafe`
/// 原先只为转手裸句柄，不承载任何解引用契约。
///
/// 调用序契约（正确性，非内存安全）：`host` 为调用方持有的静态生命周期值，装箱进
/// 与闭包同寿命的 userdata 后由 GC 终结；`lua_require::<C>` 与本 `C` 同源单态化
/// （其运行期按 upvalue(1) 以 `C` 取回宿主）。净压一个闭包值。
pub(crate) fn luarequire_pushrequire<C: RequireHost + 'static>(l: &mut LuaState, host: C) -> i32 {
  push_closure(l, host, Some(lua_require::<C>), REQUIRE_DEBUGNAME)
}
