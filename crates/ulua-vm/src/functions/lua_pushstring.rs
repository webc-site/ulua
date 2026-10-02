use core::ffi::c_char;

use crate::{
  functions::{cstr_bytes, lua_pushlstring::lua_pushlstring_bytes},
  records::lua_state::LuaState,
};

/// C-ABI 镜像垫片（cpp `lua_pushstring`，`VM/src/lapi.cpp:754`）：NULL 走 push nil
/// 分支；非空经既有 `cstr_bytes` 门面扫首个 NUL 得长度显式的字节视图，直投
/// [lua_pushlstring_bytes] 切片 ref 核心——与 cpp `lua_pushlstring(L, s, strlen(s))`
/// 逐位等价，且免去了旧形先 strlen 再经 ptr+len 垫片二次折回的裸构造绕行。
///
/// r12 T9 裁决保留：消费面实测——本形是 `const char*` C ABI 入口的 null→nil 键
/// 分支专属面（切片形无从表达 null）：现存唯一消费方 `lua_l_getmetafield`（C 形
/// `event`，null 折算 nil 键恒未命中，与其 bytes 姊妹形的分工注释互证）；
/// `l` 收 `&mut` 接收者形（存活由类型承载，同 `lua_l_optlstring` 门面先例）。
///
/// # Safety
/// `l` 须为存活 `LuaState` 且处于可 GC/可分配的受保护帧（两分支都向 `(*l).top` 净压
/// 1 个值，核心可分配并触发 GC）；`s` 允许 NULL（走 nil 分支），非空时须指向以 NUL
/// 结尾、且存活期覆盖本次扫描+拷贝窗口的合法 C 串（核心界内拷毕即止，`s` 无需在
/// 返回后存续）。cpp VM/src/lapi.cpp:754
pub(crate) unsafe fn lua_pushstring(l: &mut LuaState, s: *const c_char) {
  unsafe {
    if s.is_null() {
      l.push_nil();
    } else {
      // strlen 等价：cstr_bytes 零拷贝扫描至 NUL，长度即切片长度
      lua_pushlstring_bytes(l, cstr_bytes(s));
    }
  }
}
