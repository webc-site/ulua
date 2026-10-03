//! userdata 分配面（cpp `lua_newuserdatatagged`）：GC 点 → 线程屏障 → 栈余量 →
//! 分配 → 压栈 → 交回载荷指针。
//!
//! 裸指针保留：载荷存活期由 GC 判定，类型集合非静态可穷举（宿主按 tag 自定义载荷，
//! review.md §2/§4 保留条款）。返回值不折成 `&mut [u8]`：`&mut` 借用必须锚定一个
//! Rust 生命周期，而本指针的有效性跨越「对象被 `luaC_freeallobjects`/回收前」这段
//! 由 GC 决定的窗口，任何借用形态都要么伪造存活期（不成立的所有权证明）要么把
//! `&mut LuaState` 独占借出而与后续 `lua_push*`/`lua_setmetatable` 逐次重入互斥。
//! B1c 台账裁定：维持 `*mut c_void` + 本文档契约；`&mut [u8]` 改造的波及面
//! （`ulua-analysis` 两处、`ulua-vm/records/lua_state/stack.rs`、`lua_b_newproxy`
//! 与 conformance/unit 测试的 C API 镜像面）超出本票半径，交后续统一裁决。

use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_u_newudata::lua_u_newudata,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, checkliveness::checkliveness,
    isblack::isblack, lua_c_check_gc::lua_c_check_gc, lua_utag_limit::LUA_UTAG_LIMIT,
    utag_proxy::UTAG_PROXY,
  },
  records::{gc_object::GCObject, lua_state::LuaState},
};

/// `lua_newuserdatatagged` / `lua_newuserdatataggedwithmetatable` 共享核心：
/// GC 点 → 线程屏障 → 栈余量 → 分配 →（`with_metatable` 时挂 `udatamt[tag]`）→
/// 压栈返回 payload 指针；操作顺序与 cpp lapi.cpp 逐位一致。
/// # Safety：`l` 须为存活 `LuaState` 且栈顶留有压栈余量；`with_metatable` 时
/// `tag` 须在真实 utag 界内（否则 `udatamt[tag]` 越界读并挂错元表），`false` 时
/// 额外放行 `UTAG_PROXY`。
pub(crate) unsafe fn new_udata_impl(
  l: *mut LuaState,
  sz: usize,
  tag: i32,
  with_metatable: bool,
) -> *mut c_void {
  // SAFETY: 契约保证 `l` 为存活调用帧且 size 经上限检查，新 userdata 按 size 分配、tag 元表查找在注册界内
  unsafe {
    api_check!(
      l,
      (tag as u32) < LUA_UTAG_LIMIT as u32 || (!with_metatable && tag == UTAG_PROXY)
    );
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(&mut *l, 1);

    let u = lua_u_newudata(l, sz, tag);

    if with_metatable {
      ulua_common::LUAU_ASSERT!(!isblack!(u as *mut GCObject));

      let h = (*l).gs_ref().udatamt[tag as usize];
      api_check!(l, !h.is_null());

      (*u).metatable = h;
    }

    (*(*l).top).value.gc = u as *mut GCObject;
    (*(*l).top).tt = LuaType::UserData as i32;
    checkliveness!((*l).global, (*l).top);
    api_incr_top!(l);

    (*u).data.as_mut_ptr().cast()
  }
}

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_newuserdatatagged(l: *mut LuaState, sz: usize, tag: i32) -> *mut c_void {
  // SAFETY: 转发共享核心；`with_metatable = false` 保持放行 `UTAG_PROXY` 的原校验宽度
  unsafe { new_udata_impl(l, sz, tag, false) }
}
