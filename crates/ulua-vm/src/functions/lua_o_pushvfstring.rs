use core::{ffi::c_char, fmt::Arguments};

use crate::{
  functions::{cstr_bytes, fmt_cstr_buf::fmt_cstr_buf},
  macros::{incr_top::incr_top, lua_s_new::lua_s_new, setsvalue::setsvalue, svalue::svalue},
  records::{lua_l_strbuf::LUA_BUFFERSIZE, lua_state::LuaState},
};

/// cpp `luaO_pushvfstring`（`VM/src/lobject.cpp:130-138`）的 Rust 版。
///
/// cpp 用 vsnprintf 按 fmt 展开 va_list；Rust 移植里参数已由调用方经
/// `format_args!` 生成 `core::fmt::Arguments`，fmt 串不再被解析，故无 fmt 参数。
/// 写入固定栈缓冲并以 NUL 截断，与 vsnprintf 语义一致。
///
/// # Safety
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`：`incr_top!` 在栈顶空槽写入串值
/// （分配 `lua_s_new`、可触发 GC 写屏障），返回指针指向栈顶串内部、下一次操作 `l` 前
/// 有效；格式化输出须为 ASCII——Rust fmt 输出 UTF-8，而 `lua_s_new` 按 C 串 strlen 取值，
/// 内嵌 NUL/多字节会截错。cpp lobject.cpp:130。
pub unsafe fn lua_o_pushvfstring(l: *mut LuaState, args: Arguments<'_>) -> *const c_char {
  // 对应 cpp lobject.cpp:131-132 `char result[LUA_BUFFERSIZE]; vsnprintf(...)`
  let mut buffer = [0 as c_char; LUA_BUFFERSIZE];
  fmt_cstr_buf(&mut buffer, args);

  // Safety: 契约保证 `l` 栈顶可写且 fmt 与可变参按转换符严格匹配（错配即 UB），块内格式化结果经串原语压回 `l`
  unsafe {
    // 对应 cpp lobject.cpp:134 `setsvalue(L, L->top, luaS_new(L, result))`；
    // buffer 为 NUL 截断的 C 串，经 cstr_bytes 扫首个 NUL 得切片（保持 strlen 语义）
    setsvalue!(l, (*l).top, lua_s_new(l, cstr_bytes(buffer.as_ptr())));
    // 对应 cpp ldo.h:27-31 `incr_top`（luaD_checkstack(1) + top++）
    incr_top!(l);
    // 对应 cpp lobject.cpp:136 `return svalue(L->top - 1)`
    svalue!((*l).top.offset(-1))
  }
}
