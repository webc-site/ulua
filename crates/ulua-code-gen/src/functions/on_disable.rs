//! Source: `CodeGen/src/CodeGen.cpp:100`
//!
//! 禁用某 proto 的 native 代码：把其 entry 指回字节码，清除 exec target，
//! 并遍历所有线程的 Lua call stack，清除仍指向该 proto 的帧上的
//! `LUA_CALLINFO_NATIVE` 标志。

use core::ffi::c_void;
use std::ptr::eq;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::lua_m_visitgco::lua_m_visitgco,
  macros::{is_lua::isLua, lua_callinfo_native::LUA_CALLINFO_NATIVE},
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState, proto::Proto},
};

/// GC 全量遍历回调：禁用 codegen 时清理解析数据。
/// # Safety
/// `context` 为注册时传入的 `*mut Proto`，`gco` 指向存活 GC 对象（由 GC 遍历保证）。
unsafe fn on_disable_visitor(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  // Safety: 本回调由 `lua_m_visitgco` 在 GC 全量遍历（stop-the-world、单线程）中调用：`gco` 指向
  // 存活 GC 对象、`context` 为注册时传入的存活 `*mut Proto`。判 `gch.tt` 为 Thread 后 `as_thread_mut`
  // 得到存活 `LuaState`；`ci`/`base_ci` 框定该线程已分配的 CallInfo 栈区间，循环 `ci > base_ci`
  // 且逐步 `sub(1)` 界内递减；`isLua!` 为真时 `(*ci).func` 为存活 Lua 函数 TValue，`clvalue!` 取
  // Closure。写 `(*ci).flags` 清位是对存活 CallInfo 的独占更新，无 &mut 别名。
  // 下文各窄块统一简记「依契约」。
  let proto = context as *mut Proto;

  // Safety: 依契约——`gco` 指向存活 GC 对象，gch.tt 字段直读。
  if unsafe { (*gco).gch.tt as i32 != LuaType::Thread as i32 } {
    return false;
  }

  // Safety: 依契约——类型判定通过后 as_thread_mut 视作存活 LuaState。
  let th = unsafe { (*gco).as_thread_mut().unwrap() };

  // Safety: 依契约——`ci` 取自该存活线程当前帧，base_ci 界内。
  let mut ci = th.ci;
  // Safety: 依契约——`ci > base_ci` 保证当前帧在已分配 CallInfo 区间内。
  while ci > th.base_ci {
    // Safety: 依契约——isLua! 为真时 `(*ci).func` 为存活 Lua 函数 TValue，as_closure 取存活
    // Closure（短路次序与拆分前一致：先判 isLua）。
    if unsafe { isLua!(ci) && (*(*ci).func).as_closure().inner.l.p == proto } {
      // Safety: 依契约——对存活 CallInfo 的独占写位清理，无 &mut 别名。
      unsafe { (*ci).flags &= !(LUA_CALLINFO_NATIVE as u32) };
    }
    // Safety: 依契约——区间界内逐步递减。
    ci = unsafe { ci.sub(1) };
  }

  false
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_disable(l: *mut LuaState, proto: *mut Proto) {
  // Safety: `l` 为存活 `LuaState`、`proto` 为存活 `*mut Proto`（调用方 codegen disable 路径保证）。
  // 读写 `(*proto).codeentry/code/exectarget` 均为该存活对象上的字段操作（把入口指回字节码、清
  // 原生目标）；`lua_m_visitgco` 的 visitor 契约（context 存活、gco 由遍历提供）由下方 visitor 证成。
  // 以下各窄块统一援引本契约。

  // proto 已使用字节码则什么都不做
  if unsafe { eq((*proto).codeentry, (*proto).code) } {
    return;
  }

  // 确保 VM 不再为该 proto 调用 native 代码
  unsafe { (*proto).codeentry = (*proto).code as *const _ };

  // 阻止 native 代码进入带断点的 proto
  unsafe { (*proto).exectarget = 0 };

  unsafe { lua_m_visitgco(l, proto as *mut c_void, on_disable_visitor) };
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn on_disable_export(l: *mut LuaState, proto: *mut Proto) {
  // Safety: `extern "C-unwind"` FFI 壳，按 Lua/C 契约由 VM 宿主传入存活的 `LuaState` 与 `Proto`；
  // 本行原样转发给 `on_disable`，其 unsafe 前置条件由该宿主约定满足。
  unsafe {
    on_disable(l, proto);
  }
}
