use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tovector::lua_tovector, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 调用方须保证：`l` 为存活调用帧、`narg` 为可读实参栈槽；非 vector 时 `tag_error` 经 `l` 抛错不返回。
/// 返回的 `*const f32` 指向该槽 TValue 内联 vector 数据，仅在该槽未被覆写、对象未被 GC 回收期间有效。
/// cpp laux.cpp:266 `luaL_checkvector`
pub fn lua_l_checkvector(l: &mut LuaState, narg: i32) -> *const f32 {
  // 消费形：指针由 `lua_tovector` 的 C 镜像边界单点带出，此处仅做 null 判据透传；
  // 调用侧统一经 `vector_shared::check_vector`/`opt_vector` 门面收成 `[f32; 4]` 值窗，
  // 签名保持 laux C 镜像（cpp laux.cpp:266）不动。
  let v = lua_tovector(l, narg);
  if v.is_null() {
    // SAFETY: 契约保证 `l` 为存活调用帧（`&mut` 接收者承载存活）；非 vector 时
    // `tag_error` 经 `l` 抛错不返回
    unsafe { tag_error(l, narg, LuaType::Vector as i32) }
  }
  v
}
