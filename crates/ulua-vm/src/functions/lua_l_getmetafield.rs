use core::ffi::c_char;

use crate::{
  functions::{lua_pushstring::lua_pushstring, lua_rawget::lua_rawget},
  records::lua_state::LuaState,
};

/// 键入栈后的元表 rawget 主体（`lua_getmetatable` 已将元表压到 -2，调用方已压入键）：
/// 命中时把元方法留在栈顶并移除元表，未命中弹回键与元表。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v43 收形后
/// rawget/判空/弹栈全经安全被调与门面，体内无裸操作，故降为安全 `fn`）：调用时栈顶须为键、
/// -2 为元表（[`lua_l_getmetafield_bytes`] / [`lua_l_getmetafield`] 的前置即此）。
fn metafield_rawget(l: &mut LuaState) -> i32 {
  lua_rawget(l, -2);

  if l.is_nil(-1) {
    l.pop(2); // remove metatable and metafield
    0
  } else {
    l.remove(-2); // remove only metatable
    1
  }
}

/// [`lua_l_getmetafield`] 的字节切片核心（§10：Rust 内部调用方一律走此形，
/// 不构造 NUL 结尾缓冲）。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v43 收形后
/// 取元表/压键/查表全经安全被调与门面，体内无裸操作，故降为安全 `fn`）：调用方须保证 `l` 处于
/// 可分配/GC 的受保护帧；`obj` 为合法栈索引（`lua_getmetatable` 读该值元表），`event` 为元表键
/// 字节切片（intern 后 `lua_rawget` 查元表）；命中时把元方法留在栈顶并移除元表。
/// cpp/VM/src/laux.cpp:279 luaL_getmetafield。
pub fn lua_l_getmetafield_bytes(l: &mut LuaState, obj: i32, event: &[u8]) -> i32 {
  if !l.get_metatable(obj) {
    return 0; // no metatable
  }

  l.push_bytes(event);
  metafield_rawget(l)
}

/// # Safety
/// `l` 的存活与独占由 `&mut LuaState` 接收者承载（r16-v47 收形，导出壳经
/// `capi_shell_l_obj_event!` 的 `@ref` 臂在壳帧内重建该引用，借用窗止于当次调用）；`l` 须处于
/// 可分配/GC 的受保护帧；`obj` 为合法栈索引（`lua_getmetatable` 读该值元表）；`event` 须为 NUL
/// 结尾 C 串或 null（C ABI 契约：null 经 `lua_pushstring` 折算为 nil 键，恒未命中），且存活期覆盖
/// 本次扫描+拷贝窗口；命中时把元方法留在栈顶并移除元表。cpp/VM/src/laux.cpp:279 luaL_getmetafield。
///
/// `unsafe fn` 屏障按 r16-v21 判例保留：`l` 侧收形后体内已无裸解引用（取元表/压键/查表全经门面与
/// 安全被调），但 `event` 这枚不受 Rust 类型约束的裸 C 串仍原样转手交 `lua_pushstring`（其体内
/// `cstr_bytes` 扫描为真实裸操作）——屏障留在本函，不把该前提降级成隐含约定。
pub unsafe fn lua_l_getmetafield(l: &mut LuaState, obj: i32, event: *const c_char) -> i32 {
  unsafe {
    if !l.get_metatable(obj) {
      return 0; // no metatable
    }

    // 保 C 契约的 null→nil 键分支；非空即与 `lua_l_getmetafield_bytes(cstr_bytes(event))` 同路径
    // SAFETY: `event` 的存活/NUL 结尾前提即上方契约所列；`&mut *l` 一次性重借用即垫片期望的接收者形
    lua_pushstring(&mut *l, event);
    metafield_rawget(l)
  }
}
