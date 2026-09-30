use crate::{enums::lua_status::LuaStatus, records::lua_state::LuaState};

/// 判定该线程是否处于复位态（帧指针回到固定区且状态码为 `Ok`）。B 档契约前移
/// （参照 `abs_index`/`lua_status` 先例）：原 `*mut LuaState` 存活契约改由 `&`
/// 接收者的引用有效性规则在调用点承载；本体仅比较 `ci`/`base_ci`/`base`/`top`
/// 四个指针字段值与 `status` 一个 `u8` 字段，指针比较不解引用所指元素，零
/// unsafe 操作，签名转 safe fn。cpp `lstate.cpp:201`。
pub fn lua_isthreadreset(l: &LuaState) -> i32 {
  (l.ci == l.base_ci && l.base == l.top && l.status == LuaStatus::Ok as u8) as i32
}
