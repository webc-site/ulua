use crate::records::lua_state::LuaState;

/// 读该状态所属主线程指针。B 档契约前移（参照 `abs_index`/`isyielded` 先例）：
/// 原 `*mut LuaState` 存活契约改由 `&` 接收者的引用有效性规则在调用点承载；
/// 被调体仅解引用实例建档时写入的 `global` 指针字段取 `mainthread` 指针值，
/// 返回值原样透传裸指针、本体不解引用。cpp `lapi.cpp:247`。
pub fn lua_mainthread(l: &LuaState) -> *mut LuaState {
  // Safety: `l` 为经 VM 构造的存活 `LuaState` 引用，其 `global` 字段自建档起即指向
  // 该实例所属的 `global_State` 且终身有效；此处仅读其 `mainthread` 一个指针字段，
  // 不解引用所指元素。
  unsafe { (*l.global).mainthread }
}
