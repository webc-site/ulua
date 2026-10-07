use core::fmt::Arguments;

use crate::{
  functions::{cstr_bytes, fmt_cstr_buf::fmt_cstr_buf},
  macros::{incr_top::incr_top, lua_s_new::lua_s_new, setsvalue::setsvalue},
  records::{lua_l_strbuf::LUA_BUFFERSIZE, lua_state::LuaState},
};

/// fstring 族的切片 ref 核心（r12 T9 形）：cpp `luaO_pushvfstring`
/// （`VM/src/lobject.cpp:130-138`）的全部真实逻辑落在 `&mut LuaState` 签名——
/// `format_args!` 写入固定栈缓冲并以 NUL 截断（与 cpp vsnprintf 展开语义一致，
/// 单点在 `fmt_cstr_buf`），经 `cstr_bytes` 门面取首个 NUL 前视图后由 `lua_s_new`
/// intern 并压入栈顶（分配、可触发 GC 写屏障）。
///
/// Rust 移植里变参已由调用方 `format_args!` 直接参数化（cpp 为 va_list），fmt 串
/// 不再被运行期解析，故本形无 fmt 参数。返回值收口为 `()`：消费面实测本核心的
/// 三个转发包（`lua_pushfstring_l`/`lua_pushvfstring`/`lua_o_pushfstring`）中，
/// 返回指针（cpp `return svalue(L->top - 1)`）唯一需求点登记在 `lua_pushfstring_l`
/// 的折形里，其余全链零消费——`svalue!` 裸读不再出现在本文件。
///
/// # Safety
/// 契约三要素：`l` 须为正在执行的 C 函数帧的存活 `LuaState`（存活由接收者引用
/// 承载），调用点处于可 GC 的受保护帧——`incr_top!` 经 `luaD_checkstack(1)` 保证
/// top 槽可写后 `top++`；格式化输出须为 ASCII——Rust fmt 输出 UTF-8 而 `cstr_bytes`
/// 按 strlen 取值，内嵌 NUL/多字节会截错（与 cpp vsnprintf 截断语义逐位一致的既定
/// 契约）；结果串借出窗口由栈槽存活钉住，本形返回 `()` 不借出任何裸指针。
/// cpp lobject.cpp:130。
pub(crate) unsafe fn lua_o_pushvfstring_ref(l: &mut LuaState, args: Arguments<'_>) {
  // 对应 cpp lobject.cpp:131-132 `char result[LUA_BUFFERSIZE]; vsnprintf(...)`
  let mut buffer = [0u8; LUA_BUFFERSIZE];
  fmt_cstr_buf(&mut buffer, args);

  // SAFETY: 契约保证 `l` 存活且栈顶可写、格式化输出为 NUL 截断的可读区，
  // `lua_s_new`/`setsvalue!`/`incr_top!` 的其余前提由契约与扩容成立
  unsafe {
    // 对应 cpp lobject.cpp:134 `setsvalue(L, L->top, luaS_new(L, result))`；
    // buffer 为 NUL 截断的 C 串，经 cstr_bytes 扫首个 NUL 得切片（保持 strlen 语义）
    setsvalue!(l, l.top, lua_s_new(l, cstr_bytes(buffer.as_ptr().cast())));
    // 对应 cpp ldo.h:27-31 `incr_top`（luaD_checkstack(1) + top++）
    incr_top!(l);
  }
}
