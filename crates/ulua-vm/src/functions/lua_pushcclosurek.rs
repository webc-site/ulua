//! Source: `VM/src/lapi.cpp:742-760` (hand-ported)

use core::{ffi::c_char, ptr::addr_of_mut};

use crate::{
  functions::{
    c_slice, c_slice_mut, getcurrenv::getcurrenv, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_f_new_cclosure::lua_f_new_cclosure,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top,
    iswhite::iswhite, lua_c_check_gc::lua_c_check_gc, setclvalue::setclvalue, setobj_2_n::setobj2n,
  },
  records::{gc_object::GCObject, lua_state::LuaState},
  type_aliases::{
    lua_c_function::LuaCFunction, lua_continuation::LuaContinuation, t_value::TValue,
  },
};

/// 闭包 k 值（upvalue）登记的切片核心：`l` 以独占引用传入（存活由类型保证），待捕获值窗
/// 与闭包 upvals 堆块各自收口为 `&[TValue]`/`&mut [TValue]` 切片，逐格 `setobj2n` 与 cpp
/// 倒序写等价（同一对 (upvals[k], top+k)，写入互不交叠，顺序不可观察）。
///
/// 全流程顺序与 cpp lapi.cpp:742 逐指令一致：`api_check` → GC 点 → 线程屏障 →
/// 栈余量 → 元素数断言 → 建闭包 → 写 f/cont/debugname → 出栈 nup → 登记 → 闭包入槽 →
/// 白度断言 → 抬栈顶。
///
/// # Safety（内部窄窗契约，签名安全：调用方无需 unsafe 上下文）
/// 1. `l` 为正在执行的 API 帧的存活 `LuaState`，`r#fn` 遵循 Lua C 函数约定（cpp:783
///    `api_check(fn)`）；
/// 2. `nup >= 0` 且自栈顶起 `nup` 槽为已压入的可捕获值（`api_checknelems` 的判据），
///    据此派生的 `&[TValue]` 窗在登记期间可读、且与闭包自有 upvals 堆块不相交；
/// 3. `debugname` 为空或在闭包存活期内保持有效的 NUL 串（VM 只存指针不复制，cpp 形
///    `cl->c.debugname = debugname`），`cont` 为可在可 yield 路径安全调用的回调；
/// 4. `lua_c_check_gc`/`lua_f_new_cclosure` 可分配、可触发 GC——须在受保护帧内调用。
pub(crate) fn lua_pushcclosurek_ref(
  l: &mut LuaState,
  r#fn: LuaCFunction,
  debugname: *const c_char,
  nup: i32,
  cont: LuaContinuation,
) {
  // SAFETY: 契约 1/2——`l` 为存活帧：两条断言与 `api_checknelems` 只读该帧 top/base/ci 字段，
  // GC 点与线程屏障只触及 global 与 gray 链；`ensure_stack_space(1)` 即 cpp
  // `ensure_stack(L, 1)`，扩容可移动栈，故其前不派生任何槽窗
  unsafe {
    api_check!(l, r#fn.is_some());
    api_check!(l, nup >= 0);
    lua_c_check_gc!(l.as_mut_ptr());
    lua_c_threadbarrier_lapi(l.as_mut_ptr());
    l.ensure_stack_space(1);
    api_checknelems!(l, nup);
  }

  // SAFETY: 契约 1/3/4——闭包按 size_cclosure(nup) 分配、env 取当前帧；`cc` 指向刚分配且
  // 本函数独占的闭包自有字段（f/cont/debugname 为载荷值写入，debugname 只存指针、
  // 存活期由契约 3 钉住）；出栈后 `pending`（top 起 nup 槽）与 `captured`（闭包 upvals
  // 堆块）两窗不相交，逐格 `setobj2n` 即 cpp 倒序写的同一对复制
  unsafe {
    let cl = lua_f_new_cclosure(l.as_mut_ptr(), nup, getcurrenv(l.as_mut_ptr()));
    let cc = addr_of_mut!((*cl).inner.c);
    (*cc).f = r#fn;
    (*cc).cont = cont;
    (*cc).debugname = debugname;

    // 出栈 nup：待捕获值窗自新 top 起
    l.top = l.top.sub(nup as usize);

    // k 值登记（切片形）：dst=闭包 upvals 堆块、src=出栈后的栈值窗，二者不相交
    let captured: &mut [TValue] =
      c_slice_mut(addr_of_mut!((*cc).upvals) as *mut TValue, nup as usize);
    let pending: &[TValue] = c_slice(l.top, nup as usize);
    for (dst, src) in captured.iter_mut().zip(pending) {
      setobj2n!(l, dst as *mut TValue, src as *const TValue);
    }

    setclvalue!(l, l.top, cl);
    ulua_common::LUAU_ASSERT!(iswhite!(cl as *mut GCObject));
    api_incr_top!(l);
  }
}

/// C-ABI 镜像垫片：把 `lua_pushcclosurek_ref` 的独占引用形折回 cpp
/// `lua_pushcclosurek`（`VM/src/lapi.cpp:742`）的 `lua_State*` 形，语义零差。
///
/// r12-w4b 按 T9 形实测裁决**保留**：全仓消费面 C 形入口真实存在且为零业务折形——
/// 跨 crate 消费方 `ulua-rt`（`sys.rs` re-export，`state.rs` 的 `push_named_closure` /
/// `push_anonymous_closure` 门面）、`ulua-require`（`push_closure.rs`）、`ulua-web`
/// （`wasm.rs` 的 sandbox print 钩子），测试门面 `ulua-conformance`（`safe_api.rs` 的
/// cfunction/closurek 两形）、`ulua-cli-test`（`require_by_string.rs` 三处），`ulua-vm` 内
/// 另有 7 处 C 形调用点（`records/lua_state/stack.rs` 两方法、`luaopen_base` 两处、
/// `luaopen_coroutine`、`f_ccall`、`cowrap`）；`ulua-capi` 侧实测零导出壳（本符号不在
/// C ABI 导出表内）。函数体保持对 `lua_pushcclosurek_ref` 的一行折形委托。
/// 后续票建议：把上述消费点逐个迁至 ref 核心（Rust 侧即免 unsafe 上下文），迁毕删本垫片。
///
/// # Safety
/// `l` 指向存活 `LuaState`；`fn` 非空且遵循 Lua C 函数约定（cpp lapi.cpp:783 `api_check(fn)`）；
/// `nup >= 0` 且栈顶已压入 `nup` 个可捕获上值（`api_checknelems`）；`debugname` 为空或须在
/// 闭包存活期内保持有效的 NUL 串；`cont` 为可在可 yield 路径安全调用的回调。cpp lapi.cpp:781.
/// 其余前提与被调核心的 `# Safety` 契约逐条同一。
pub unsafe fn lua_pushcclosurek(
  l: *mut LuaState,
  r#fn: LuaCFunction,
  debugname: *const c_char,
  nup: i32,
  cont: LuaContinuation,
) {
  // SAFETY: 契约保证 `l` 非空且指向存活 LuaState，本帧重建独占引用后即结束借用窗口
  unsafe { lua_pushcclosurek_ref(&mut *l, r#fn, debugname, nup, cont) }
}
