//! 单指针转发 C 入口蹦床的收口单点。
//!
//! 收口前 `register_types_library.rs` 的 `type_lib_thunk!` 与
//! `register_type_user_data.rs` 的 `tud_thunk!` 是两份逐字相同的宏（仅宏名与
//! Safety 注释措辞不同），现合并为本宏。cpp 侧类型函数 runtime 把每个 analysis
//! 函数以 `lua_CFunction` 注册进 VM；Rust 侧 analysis 函数的形参已是 vm 侧
//! `&mut LuaState`（本 crate 不再有 `LuaState` 镜像类型），而 VM 回调签名是
//! `lua_CFunction` 的裸 `*mut lua_State`，故需一层 `extern "C-unwind"` 蹦床把
//! C-ABI 指针对象重建为独占借用并转发。本宏把该骨架收口，调用点只声明
//! 「蹦床名 + 真函数路径」。

/// 生成 `LuaCfunction` 形状的 `extern "C-unwind"` 蹦床，转发到收
/// `&mut ulua_vm::records::lua_state::LuaState` 的 analysis 级函数。
///
/// 用法：
/// ```ignore
/// c_thunk!(create_any_thunk, create_any, @ref);
/// ```
/// `@ref` 旗标沿用 `capi_*_l_cint!(m, n @ref)` 诸族的形记法，就地标明「真函数收
/// 独占引用」。旗标前带逗号是该诸族与本宏的唯一形差：`$real` 取 `path` 片段，而
/// `@` 不在 `path` 的后随集内（宏只能以 `,` 收尾该片段），`ident` 片段则无此限制。
/// 本宏曾有收下裸指针的裸透传臂，随 analysis 入口全数改收引用而后零消费者，
/// 按 review.md §7 删除（零消费者的臂既不参与展开检验，又属死文本）。
macro_rules! c_thunk {
  ($thunk:ident, $real:path, @ref) => {
    unsafe extern "C-unwind" fn $thunk(l: *mut ulua_vm::records::lua_state::LuaState) -> i32 {
      // Safety: 本蹦床是 C-ABI 边界，形参形状由 `lua_CFunction` 约定固定。Lua 只会
      // 在活跃 lua_State 的调用帧上回调已注册的 C 函数，故 `l` 非空、指向存活且本次
      // 调用独占的 VM 状态（与 C++ `lua_CFunction` 入口契约相同），据此可把它重建为
      // 独占 `&mut` 借用，借用窗严格止于 `$real` 返回。`$real` 的前置条件（存活
      // state、调用期独占其栈）由单线程类型函数 runtime 的调度逐次满足。
      unsafe { $real(&mut *l) }
    }
  };
}

pub(crate) use c_thunk;
