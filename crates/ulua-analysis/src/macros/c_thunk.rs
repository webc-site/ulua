//! 单指针转发 C 入口蹦床的收口单点。
//!
//! 收口前 `register_types_library.rs` 的 `type_lib_thunk!` 与
//! `register_type_user_data.rs` 的 `tud_thunk!` 是两份逐字相同的宏（仅宏名与
//! Safety 注释措辞不同），现合并为本宏。cpp 侧类型函数 runtime 把每个 analysis
//! 函数以 `lua_CFunction` 注册进 VM；Rust 侧 analysis 函数的形参是本 crate 的
//! 不透明 `LuaState` 镜像类型，而 VM 回调签名是 vm 侧不透明 `lua_State`，故需
//! 一层 `extern "C-unwind"` 蹦床做地址透传转型并转发。本宏把该骨架收口，调用
//! 点只声明「蹦床名 + 真函数路径」。

/// 生成 `LuaCfunction` 形状的 `extern "C-unwind"` 蹦床，转发到声明在本 crate
/// 不透明 `LuaState` 上的 analysis 级函数。
///
/// 用法：
/// ```ignore
/// c_thunk!(create_unknown_thunk, create_unknown);
/// c_thunk!(create_any_thunk, create_any, @ref);
/// ```
/// 第二形（r16-v42）是**引用重建**变体：真函数已把首参收形为 `&mut LuaState`（analysis
/// 侧 Rust 面不再持裸指针），蹦床仍按 `lua_CFunction` 约定收 vm 侧裸 `*mut lua_State`
/// （C-ABI 镜像红线，导出形状逐字不变），只在本帧把该指针重建为独占 `&mut` 借用、
/// 借用窗严格止于真函数返回。裸透传形与 `@ref` 形按真函数签名二选一，不得凭偏好混用。
/// 旗标前带逗号是本宏与 `capi_*_l_cint!(m, n @ref)` 诸族的唯一形差：`$real` 取 `path`
/// 片段，而 `@` 不在 `path` 的后随集内（宏只能以 `,` 收尾该片段），`ident` 片段则无此限制。
macro_rules! c_thunk {
  ($thunk:ident, $real:path, @ref) => {
    unsafe extern "C-unwind" fn $thunk(l: *mut ulua_vm::records::lua_state::LuaState) -> i32 {
      // Safety: Lua 只会在活跃 lua_State 的调用帧上回调已注册的 C 函数，传入的
      // `l` 非空、指向存活且本次调用独占的 VM 状态（与 C++ `lua_CFunction` 入口
      // 契约相同），故可把它重建为独占 `&mut` 引用；analysis 侧 `LuaState` 是不透明
      // 镜像类型，与 vm 侧不透明 `lua_State` 间的指针转换是地址不变的透传，借用窗
      // 止于 `$real` 返回。`$real` 的前置条件（存活 state、调用期独占其栈）由单线程
      // 类型函数 runtime 的调度逐次满足。
      unsafe { $real(&mut *(l as *mut crate::type_aliases::lua_state::LuaState)) }
    }
  };
  ($thunk:ident, $real:path) => {
    unsafe extern "C-unwind" fn $thunk(l: *mut ulua_vm::records::lua_state::LuaState) -> i32 {
      // Safety: Lua 只会在活跃 lua_State 的调用帧上回调已注册的 C 函数，传入的
      // `l` 非空、指向存活且本次调用独占的 VM 状态（与 C++ `lua_CFunction` 入口
      // 契约相同）；analysis 侧 `LuaState` 是不透明镜像类型，与 vm 侧不透明
      // `lua_State` 间的指针转换是地址不变的透传。`$real` 的前置条件（存活
      // state、调用期独占其栈）由单线程类型函数 runtime 的调度逐次满足。
      unsafe { $real(l as *mut crate::type_aliases::lua_state::LuaState) }
    }
  };
}

pub(crate) use c_thunk;
