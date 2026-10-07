//! Source: `VM/src/lapi.cpp:742-760` (hand-ported)

use core::{
  ffi::c_char,
  ptr::{addr_of_mut, null_mut},
};

use crate::{
  functions::{
    c_slice, c_slice_mut, cstr_bytes, getcurrenv::getcurrenv,
    lapi_barrier::lua_c_threadbarrier_lapi, lua_f_new_cclosure::lua_f_new_cclosure,
    lua_s_newlstr::lua_s_newlstr,
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
/// debugname 收 `Option<&[u8]>`（载荷不含 NUL；`None` 即 cpp NULL 哨兵）：非空时经
/// `lua_s_newlstr` 当场 intern 复制为 TString 锚入闭包（cpp lapi.cpp:752
/// `cl->c.debugname = debugname ? luaS_new(L, debugname) : nullptr`），调用方缓冲仅在
/// 本调用期内借用，VM 不外存指针——上游已把裸指针契约收窄为「调用期有效」。
/// GC 语义：新闭包由 `lua_f_new_cclosure` 刚分配、此间无 GC 步进点，落 `setclvalue`
/// 前恒为白（尾部 `iswhite` 断言兜底），向白对象挂 intern 串引用免写屏障（cpp
/// lapi_barrier 通则：屏障只在对象可能已黑时触发）；串的存活由 traverseclosure 的
/// debugname 标记边保证。
///
/// 全流程顺序与 cpp lapi.cpp:742 逐指令一致：`api_check` → GC 点 → 线程屏障 →
/// 栈余量 → 元素数断言 → 建闭包 → 写 f/cont/debugname → 出栈 nup → 登记 → 闭包入槽 →
/// 白度断言 → 抬栈顶。
///
/// 调用序契约（签名安全：调用方无需 unsafe 上下文）
/// 1. `l` 为正在执行的 API 帧的存活 `LuaState`，`r#fn` 遵循 Lua C 函数约定（cpp:783
///    `api_check(fn)`）；
/// 2. `nup >= 0` 且自栈顶起 `nup` 槽为已压入的可捕获值（`api_checknelems` 的判据），
///    据此派生的 `&[TValue]` 窗在登记期间可读、且与闭包自有 upvals 堆块不相交；
/// 3. `cont` 为可在可 yield 路径安全调用的回调；
/// 4. `lua_c_check_gc`/`lua_f_new_cclosure`/intern 可分配、可触发 GC——须在受保护帧内调用。
pub(crate) fn lua_pushcclosurek_ref(
  l: &mut LuaState,
  r#fn: LuaCFunction,
  debugname: Option<&[u8]>,
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

  // SAFETY: 契约 1/2/4——闭包按 size_cclosure(nup) 分配、env 取当前帧；`cc` 指向刚分配且
  // 本函数独占的闭包自有字段（f/cont/debugname 为载荷值写入）；出栈后 `pending`（top 起
  // nup 槽）与 `captured`（闭包 upvals 堆块）两窗不相交，逐格 `setobj2n` 即 cpp 倒序写的
  // 同一对复制
  unsafe {
    // cpp `luaF_newCclosure(L, nup, getcurrenv(L))` 实参求值序即先取当前 env 再建闭包；
    // 收形后 `lua_f_new_cclosure` 首参借 `&mut l`，故先就地一次借出裸指针完成
    // `getcurrenv` 只读求值，再借出 `l` 建闭包，与原形参求值位点/顺序逐位一致
    let env = getcurrenv(l.as_mut_ptr());
    let cl = lua_f_new_cclosure(l, nup, env);
    let cc = addr_of_mut!((*cl).inner.c);
    (*cc).f = r#fn;
    (*cc).cont = cont;
    // debugname intern 复制（`lua_s_newlstr` 借 `&mut l`，与 `cc` 裸指针窗不冲突）：
    // 命中串表复用旧串、未命中新分配，均由 traverseclosure 的标记边钉住存活
    (*cc).debugname = match debugname {
      Some(name) => lua_s_newlstr(l, name),
      None => null_mut(),
    };

    // 出栈 nup：待捕获值窗自新 top 起（`rewind_top` 提交原语镜像原
    // `top = top.sub(nup)` 落值）
    l.rewind_top(nup as usize);

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
/// r12-w4b 按 T9 形实测裁决**保留**：全仓消费面 C 形入口真实存在（跨 crate 消费方
/// `ulua-rt`（`sys.rs` re-export，`state.rs` 的 `push_named_closure` /
/// `push_anonymous_closure` 门面）、`ulua-require`（`push_closure.rs`）、`ulua-web`
/// （`wasm.rs` 的 sandbox print 钩子），测试门面 `ulua-conformance`（`safe_api.rs` 的
/// cfunction/closurek 两形）、`ulua-cli-test`（`require_by_string.rs` 三处），`ulua-vm` 内
/// 另有 C 形调用点）。函数体保持对 `lua_pushcclosurek_ref` 的一行折形委托，NULL/串
/// 语义在本垫片内判空翻译（`cstr_bytes` 止于首个 NUL，载荷不含终止符）。
///
/// # Safety
/// `l` 指向存活 `LuaState`；`fn` 非空且遵循 Lua C 函数约定（cpp lapi.cpp:783 `api_check(fn)`）；
/// `nup >= 0` 且栈顶已压入 `nup` 个可捕获上值（`api_checknelems`）；`debugname` 为 null 或
/// 指向 NUL 结尾、**仅本次调用期内**有效的缓冲（VM 经 intern 复制，不外存指针）；`cont`
/// 为可在可 yield 路径安全调用的回调。cpp lapi.cpp:781. 其余前提与被调核心的调用序契约逐条同一。
pub unsafe fn lua_pushcclosurek(
  l: *mut LuaState,
  r#fn: LuaCFunction,
  debugname: *const c_char,
  nup: i32,
  cont: LuaContinuation,
) {
  // null 与空串语义分立：cpp `NULL` → 无调试名，`""` → 空名（getfuncname 返回 "" 而非
  // null），`cstr_bytes` 把 null 译成空切片会抹掉这一区分，故先行判空
  let name = if debugname.is_null() {
    None
  } else {
    // SAFETY: 非空分支契约担保 NUL 结尾且调用期内有效
    Some(unsafe { cstr_bytes(debugname) })
  };
  // SAFETY: 契约保证 `l` 非空且指向存活 LuaState，本帧重建独占引用后即结束借用窗口
  unsafe { lua_pushcclosurek_ref(&mut *l, r#fn, name, nup, cont) }
}
