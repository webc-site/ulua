use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::lua_requireinternal::lua_requireinternal, records::navigation_context::RequireHost,
};

/// proxyrequire 闭包体（cpp `requireLikeFunc` 的 proxy 形态）：泛型参数 `C` 为注入
/// 时的宿主类型，`luarequire_pushproxyrequire::<C>` 以 `Some(lua_proxyrequire::<C>)`
/// 具名实例化后 coerce 为 `LuaCFunction`，槽位读回零 `dyn`。
///
/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `LuaState`；upvalue(1) 须为
/// `push_closure::<C>` 以同一 `C` 装箱的宿主 userdata（由注入点与闭包体同源单态化
/// 保证）。
pub(crate) unsafe extern "C-unwind" fn lua_proxyrequire<C: RequireHost>(l: *mut LuaState) -> i32 {
  // Safety: 真 FFI 入口：l 是 VM 调 proxyrequire 闭包时传入的存活 state，入口
  // 一次重建独占借用；check_bytes 保证栈槽 2 为字符串（否则抛错发散）。
  let l = unsafe { &mut *l };
  // r16-p28 锚定形：窗口借用不能跨越随后以 `l` 为参的 lua_requireinternal，取 owned
  // 快照解耦（cpp `requireLikeFunc` 同点位本就复制 std::string，语义等价）。
  let requirer_chunkname = l.check_bytes(2).to_vec();
  // Safety: lua_requireinternal::<C> 按其自身契约操作本帧栈与 upvalue，`C` 与本
  // 闭包体的注入类型一致（同源单态化）。
  unsafe { lua_requireinternal::<C>(l, &requirer_chunkname) }
}
