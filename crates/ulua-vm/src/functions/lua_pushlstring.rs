use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_s_newlstr::lua_s_newlstr,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_c_check_gc::lua_c_check_gc,
    setsvalue::setsvalue,
  },
  records::lua_state::LuaState,
};

/// push-字符串族的切片 ref 核心（r12 T9 形）：全部真实逻辑落在 `&mut LuaState` +
/// `&[u8]` 签名——GC 检查、线程屏障、`ensure_stack(1)` 扩容、`lua_s_newlstr` 按
/// 切片全长 intern（无 NUL 依赖，长度即切片长度）后写 top 槽并 `api_incr_top`
/// 净压一层。cpp `lua_pushlstring`（`VM/src/lapi.cpp:744`）`luaS_newlstr(L, str, len)`
/// 的同形骨架；Rust 侧一切字节压栈都经本核心，不再折道 ptr+len 形。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载，
/// r12-w6d 收形降为安全 `fn`，同 `lua_l_checkstack`/`ensure_stack` 先例）：`l` 须为
/// 正在执行的 C 函数帧的存活 `LuaState`，调用点处于可 GC/可分配帧，top 槽写入合法性
/// 由体内 `ensure_stack(l, 1)` 扩容与栈不变量保证；体内裸操作面（`lua_c_check_gc!`
/// 宏臂展开的 `lua_c_step` 裸形、`setsvalue!`/`api_incr_top!` 对 `l.top`/`(*l.ci).top`
/// 的解引用）落窄 `unsafe` 块，被调方/宏各自的 `# Safety` 由上述契约满足。`s` 为普通
/// 借用切片，核心只界内拷入堆上 TString、不留存借出（返回 `()`，无跨调用悬垂面）。
pub fn lua_pushlstring_bytes(l: &mut LuaState, s: &[u8]) {
  // SAFETY: 契约保证 `l` 为存活调用帧；check_gc/线程屏障/扩容序与收口前逐位一致，
  // `setsvalue!` 写入的 top 槽由 `ensure_stack(l, 1)` 保证可写
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    setsvalue!(l, l.top, lua_s_newlstr(l, s));
    api_incr_top!(l);
  }
}

/// C-ABI 镜像垫片（cpp `lua_pushlstring`，`VM/src/lapi.cpp:744` 的
/// `(const char*, size_t)` 形）：只做一次性折形——`api_check!` 守 null 契约后把
/// `(s, len)` 折成 `&[u8]` 交切片 ref 核心 [lua_pushlstring_bytes]，本形不再含任何
/// 真实逻辑。
///
/// r12 T9 裁决保留：消费面实测——vm 内 Rust 消费方（`lua_pushstring`/`loadsafe`/
/// `add_value`/`push_onecapture`）已全部改直投字节切片核心，本形现存消费方仅为
/// ulua-vm 测试 `vm_contract.rs`/`strtable_intern.rs`/`vm_table_rehash.rs` 的
/// lua.h 镜像契约验证面（以 ptr+len 裸形断言与切片形同路径 intern），属跨 FFI
/// 边界契约的验证端，导出形保留。
///
/// # Safety
/// `l` 须为存活 `LuaState`（折形后转授核心契约）；`s` 非空时须对 `len` 字节可读
/// （null 违反 C 契约，由 `api_check!` 在 debug 配置拦截）；`len` 须与可读界一致，
/// 越界即 `from_raw_parts` 契约违约（UB）；核心界内拷毕即止，`s` 无需在返回后存续。
pub unsafe fn lua_pushlstring(l: *mut LuaState, s: *const c_char, len: usize) {
  api_check!(l, !s.is_null());
  // SAFETY: 契约保证 `s` 起 `len` 字节可读；len==0 时取空切片、不解引用指针
  let slice = if len == 0 {
    &[]
  } else {
    unsafe { from_raw_parts(s.cast::<u8>(), len) }
  };
  // SAFETY: 契约保证 `l` 为存活 LuaState，`&mut *l` 是一次性重借用、借用窗止于当句，
  // 即安全核心 [lua_pushlstring_bytes] 期望的接收者形
  lua_pushlstring_bytes(unsafe { &mut *l }, slice);
}
