use alloc::string::String;

use ulua_vm::{
  functions::{
    lua_l_checklstring::lua_l_checklstring_ref, lua_setsafeenv::lua_setsafeenv,
    luau_load::luau_load,
  },
  macros::lua_environindex::LUA_ENVIRONINDEX,
  records::lua_state::LuaState,
};

use crate::functions::{compile_source::compile_source, state_ref::state};

/// `loadstring` 全局函数（cpp Repl.cpp 的 lua_loadstring），经 `setup_state`
/// 注册进 VM，由 VM 在 Lua 调用点回调。
///
/// DELIBERATE DEVIATION（review.md §9.3）：本函数以 `lua_CFunction` 形态（`extern
/// "C-unwind"`）装入 VM 并在 Lua 调用点被 C ABI 回调，故签名保留裸 `*mut LuaState`
/// （返回值用原生 `i32`）；其内部对 `luau_load`/`lua_setsafeenv` 的 `unsafe` 调用
/// 均为 ulua-vm c-API 边界。字符串取形走 ref 门面（`&[u8]` 出栈），cpp
/// `luaL_checklstring(L, 1, &len)` 的长度出参裸指针、`luaL_optlstring` 的 NULL
/// 长度哨兵与 `c_slice`/`cstr_cow` 指针解码随之消失（review.md §2/§10）。
///
/// # Safety
///
/// `l` 必须是 VM 在调用本 `lua_CFunction` 时传入的当前有效线程状态，且栈上
/// 参数布局符合 Lua/C API 调用约定（索引 1 为待检查的字符串参数）。
pub(crate) unsafe extern "C-unwind" fn lua_loadstring(l: *mut LuaState) -> i32 {
  // Safety: `# Safety` 契约保证 `l` 非空、活跃，经 `state` 门面物化后全走安全方法
  // （仍是 unsafe fn 的 `lua_*` 导出在各块内论证）。
  let l = state(l);
  // Safety: 索引 1 为本次调用的实参槽位；非字符串即报错发散（不返回），返回的切片由
  // 栈槽持有、在本次调用期内有效。
  // r16-p28 锚定形：长度快照先行，使首窗在实参判定前即结束借用。
  let source_len = lua_l_checklstring_ref(l, 1).len();
  // cpp `luaL_optlstring(L, 2, s, NULL)`：槽 2 缺席/nil 时默认值即第一参数的
  // 串本体，否则按 checklstring 取形；长度出参本就弃用。
  let use_source_name = l.is_none_or_nil(2);
  let name_len = if use_source_name {
    source_len
  } else {
    // Safety: 索引 2 为本次调用的实参槽位；数字自动转换、非转换类型按 cpp 抛
    // "string expected" 发散。
    lua_l_checklstring_ref(l, 2).len()
  };

  // Safety: `lua_setsafeenv` 为 unsafe 导出；仅改环境表的 safe 标志位。形参
  // `enabled` 系 ulua-vm 侧原生 `i32`，不再绕 `c_int`（review.md §7）。
  unsafe { lua_setsafeenv(l, LUA_ENVIRONINDEX, false as i32) };

  // loadstring 参数可为任意字节串；Rust 编译管线要求 &str（UTF-8），
  // 非法序列 lossy 替换，替代 from_utf8_unchecked 的 UB
  let source = String::from_utf8_lossy(lua_l_checklstring_ref(l, 1)).into_owned();

  // 源名按 cpp 的 C 串消费规则止于首个 NUL（原 cstr_cow 的解码面在切片上等价
  // 复刻），再与 source 同款 lossy 规则转 String；窗内即快照所指槽的当前串
  let name_window = lua_l_checklstring_ref(l, if use_source_name { 1 } else { 2 });
  let name_end = name_window
    .iter()
    .position(|&byte| byte == 0)
    .unwrap_or(name_len);
  let chunkname = String::from_utf8_lossy(&name_window[..name_end]).into_owned();

  let bytecode = compile_source(&source);

  // Safety: `luau_load` 为 unsafe 导出；chunkname 为本帧 String（luau_load 按 cpp
  // strlen 规则读取），bytecode 是本帧 Vec，仅在本次调用窗口内借用。
  if unsafe { luau_load(l, &chunkname, &bytecode, 0) } == 0 {
    return 1;
  }

  // 报错路径把 nil 插到错误消息前作为多返回值（栈操作配平）。
  l.push_nil();
  l.insert(-2); // put before error message
  2 // return nil plus error message
}
