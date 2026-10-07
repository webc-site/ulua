//! Source: `VM/src/lapi.cpp:1477`
//!
//! `lua_newbuffer_push_ref` — allocate a managed buffer object of `sz` bytes and
//! push it on the stack. Runs a GC step and the thread write-barrier first,
//! exactly like the C++ public API.

use crate::{
  functions::{lapi_barrier::lua_c_threadbarrier_lapi, lua_b_newbuffer::lua_b_newbuffer},
  macros::{api_incr_top::api_incr_top, lua_c_check_gc::lua_c_check_gc, setbufvalue::setbufvalue},
  records::lua_state::LuaState,
};

/// 新建 buffer 并压栈的 `()` 形核心：GC 步进 → 线程写屏障 → 栈余量 → `lua_b_newbuffer`
/// 分配 → `setbufvalue` 写入本槽 → 抬栈顶，次序与 cpp `lua_newbuffer`（lapi.cpp:1477）
/// 逐指令一致；仅缺末端的「返回数据块裸指针」折形——数据读回一律走
/// `lua_tobuffer*`/`buffer_data_ref` 切片面（r12 T10 窄腰收口）。
///
/// r12-w6b T9 裁决（消灭裸指针返回面）：旧形 `lua_newbuffer(l, sz) -> *mut c_void` 的
/// 返回指针实测全仓零终端消费——vm 内 `buffer_create`（buffer_create.rs:16）、
/// `buffer_fromstring`（buffer_fromstring.rs:27）与 ulua-rt `buffer.rs::c_newbuffer`
/// （经 `sys.rs:51` re-export，调用点 buffer.rs:317）均以语句形弃返回值；
/// `ulua-capi` 实测零导出壳（本符号不在 C ABI 导出表内，无镜像义务）；唯一转发面
/// `ulua-conformance` 门面 `safe_api.rs::newbuffer`（safe_api.rs:856）的终端调用点
/// api.rs:137、api.rs:170、gc.rs:100 亦全为语句形弃值——故本期（承接 r12 T9 在旧注释中
/// 的登记「门面收窄为单元形后，本函数返回值改 ()」）将本体收窄为 `()` 形并同步收窄
/// 该门面，裸指针返回形整体消灭，不保留垫片。
///
/// # Safety（内部窄窗契约，签名安全：调用方无需 unsafe 上下文）
/// 1. `l` 为正在执行的 API 帧的存活 `LuaState`，栈顶有压入结果的余量（扩容经
///    `ensure_stack_space` 可移动栈，其前不派生任何槽窗）；
/// 2. `sz` 为合法缓冲长度：`sz > MAX_BUFFER_SIZE` 时 `lua_b_newbuffer` 经 `luaM_toobig`
///    以 longjmp 发散（不可跨 Rust 帧 unwind），故须在受保护帧内调用——ulua-rt
///    `buffer.rs` 的 pcall 蹦床即此契约的消费方论证；
/// 3. GC 步进与分配可发生——同 2，须在受保护帧内调用。
pub fn lua_newbuffer_push_ref(l: &mut LuaState, sz: usize) {
  // SAFETY: 契约 1——`l` 为存活帧：GC 点与线程屏障只触及 global 与 gray 链；
  // `ensure_stack_space(1)` 即 cpp `ensure_stack(L, 1)`；`lua_b_newbuffer` 按 `sz` 分配
  // （契约 2/3 的 toobig 发散与 GC 可重入由受保护帧前提兜住）；`setbufvalue` 只写扩容
  // 后的当前 `top` 槽三字段，`api_incr_top` 断言后抬顶，与 cpp 末端仅差弃回数据指针
  unsafe {
    lua_c_check_gc!(l.as_mut_ptr());
    lua_c_threadbarrier_lapi(l.as_mut_ptr());
    l.ensure_stack_space(1);
    let b = lua_b_newbuffer(l, sz);
    setbufvalue!(l.as_mut_ptr(), (*l.as_mut_ptr()).top, b);
    api_incr_top!(l.as_mut_ptr());
  }
}
