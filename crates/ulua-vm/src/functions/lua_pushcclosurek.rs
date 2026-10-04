//! Source: `VM/src/lapi.cpp:742-760` (hand-ported)
//!
//! review.md §10/判例3 收形：C-ABI 镜像垫片（`*mut LuaState` + `debugname:
//! *const c_char`）已删除——实测其 Rust 消费点已全部迁至本核心（`stack.rs`
//! 两方法、`cowrap`/`f_ccall`、`ulua-rt`/`ulua-web`/测试门面），签名收为独占
//! 引用 + 原生 `Option<&'static [u8]>` 调试名窗后，`l` 的存活由类型承载、
//! `debugname` 无指针解引用位，函数本体降为 safe `fn`（下述文档为调用序
//! 契约，正确性而非内存安全）。`ulua-capi` 侧实测零导出壳（本符号不在
//! C ABI 导出表内）。

use core::ptr::addr_of_mut;

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

/// 压入 C 闭包并登记 `nup` 个待捕获上值（cpp `lua_pushcclosurek` 唯一实现面）。
///
/// 闭包 k 值登记为切片核心：待捕获值窗与闭包 upvals 堆块各自收口为
/// `&[TValue]`/`&mut [TValue]` 切片，逐格 `setobj2n` 与 cpp 倒序写等价（同一对
/// (upvals[k], top+k)，写入互不交叠，顺序不可观察）。
///
/// 全流程顺序与 cpp lapi.cpp:742 逐指令一致：`api_check` → GC 点 → 线程屏障 →
/// 栈余量 → 元素数断言 → 建闭包 → 写 f/cont/debugname → 出栈 nup → 登记 →
/// 闭包入槽 → 白度断言 → 抬栈顶。
///
/// # Safety（调用序契约——正确性，非内存安全）
/// 1. `r#fn` 遵循 Lua C 函数约定（cpp:783 `api_check(fn)`）；
/// 2. `nup >= 0` 且自栈顶起 `nup` 槽为已压入的可捕获值（`api_checknelems` 的判据）；
/// 3. `debugname` 为空（`None`）或须在闭包存活期内保持有效的静态字节窗——
///    VM 只存引用不复制（cpp 形 `cl->c.debugname = debugname`），`'static` 是该
///    既有寿命契约的类型表达；
/// 4. `cont` 为可在可 yield 路径安全调用的回调；本函数可分配、可触发 GC，
///    须在受保护帧内调用。
///
/// 签名安全（依判例3 降 safe）：入参无调用方裸指针解引用位，`l` 存活与独占由
/// `&mut` 承载，以上契约仅约束调用时序/回调合法性（正确性，非内存安全）。
pub fn lua_pushcclosurek(
  l: &mut LuaState,
  r#fn: LuaCFunction,
  debugname: Option<&'static [u8]>,
  nup: i32,
  cont: LuaContinuation,
) {
  // SAFETY: 契约 1/2——两条断言与 `api_checknelems` 只读该帧 top/base/ci 字段，
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
  // 本调用独占的闭包自有字段（f/cont/debugname 为载荷值写入，debugname 只存引用、
  // 存活期由契约 3 钉住）；出栈后 `pending`（top 起 nup 槽）与 `captured`（闭包 upvals
  // 堆块）两窗不相交，逐格 `setobj2n` 即 cpp 倒序写的同一对复制
  unsafe {
    // cpp `luaF_newCclosure(L, nup, getcurrenv(L))` 实参求值序即先取当前 env 再建闭包；
    // 故先就地一次借出裸指针完成 `getcurrenv` 只读求值，再借出 `l` 建闭包，
    // 与原形参求值位点/顺序逐位一致
    let env = getcurrenv(l.as_mut_ptr());
    let cl = lua_f_new_cclosure(l, nup, env);
    let cc = addr_of_mut!((*cl).inner.c);
    (*cc).f = r#fn;
    (*cc).cont = cont;
    (*cc).debugname = debugname;

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
