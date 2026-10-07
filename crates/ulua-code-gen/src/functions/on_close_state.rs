use alloc::boxed::Box;
use core::{
  ptr::{from_mut, null_mut},
  sync::atomic::Ordering,
};

use ulua_vm::records::{
  lua_execution_callbacks::{FORN_HEAT_ARMED, lua_ExecutionCallbacks},
  lua_state::LuaState,
};

use crate::{
  enums::options::CodeGenContextKind, functions::get_code_gen_context::get_code_gen_context,
  records::base_code_gen_context::BaseCodeGenContext,
};

/// 与 cpp `lua_ExecutionCallbacks{}` 聚合初始化逐字段一致的零值 ecb。
///
/// `context: null_mut()` 为 DELIBERATE DEVIATION / 保留理由：`ecb` 是 VM 的宿主回调表
/// （`ulua_vm::records::lua_execution_callbacks::lua_ExecutionCallbacks`，`#[repr(C)]`），
/// 其 `context` 字段是**类型擦除的宿主 user-data**：安装端
/// `functions::initialize_execution_callbacks` 写入 `code_gen_context as *mut c_void`
/// （由 `luau_codegen_create` 的 `Box::into_raw` 移交所有权），读取端
/// `functions::get_code_gen_context` 做
/// `(*global).ecb.context.cast::<BaseCodeGenContext>()` 后 `as_mut()` 还原 `Option`。
/// 槽位跨 crate 且被 cast 还原，字段类型只能是 `*mut c_void`；此处的 null 不是
/// 「缺席哨兵」的散点写法，而是 cpp `L->global->ecb = lua_ExecutionCallbacks{}`
/// 整体清零语义（CodeGenContext.cpp:374）的忠实形态——回调函数槽用 `Option` 的
/// `None` 表达未安装，context 随全表归零一并回到「无宿主」状态。
fn zero_execution_callbacks() -> lua_ExecutionCallbacks {
  lua_ExecutionCallbacks {
    context: null_mut(),
    close: None,
    destroy: None,
    enter: None,
    disable: None,
    getmemorysize: None,
    gettypemapping: None,
    getcounterdata: None,
    inlinefunction: None,
    trace_forn_enter: None,
    forn_heat_proto: 0,
    forn_heat_pc: 0,
    forn_heat_target: 0,
    trace_forn_backedge: None,
  }
}

/// 状态关闭钩子：注销 code-gen 上下文并把 global 的 ecb 回调表复位为零值。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_close_state(l: *mut LuaState) {
  if l.is_null() {
    return;
  }

  // Safety: `l` 已判空，指向存活 LuaState，其 `global` 或为 null 或指向存活 global_State。
  let global = unsafe { (*l).global };
  if global.is_null() {
    return;
  }

  // cpp CodeGenContext.cpp:371-375:
  //   static void onCloseState(LuaState* L)
  //   {
  //       getCodeGenContext(L)->onCloseState();
  //       L->global->ecb = lua_ExecutionCallbacks{};
  //   }
  // onCloseState 为虚调用：Standalone 上下文在此销毁自身，Shared 为 no-op。
  // Rust 侧以 kind 判别静态分支——原先经 fn 槽 shim 的「取指针→下行转型→&mut 自毁」
  // 链条已删除，这里只保留唯一一处所有权回收边界。
  // Safety: `get_code_gen_context` 依其 `# Safety` 返回串行窗口内独占借用；Standalone
  // 上下文由 luau_codegen_create 的 `Box::into_raw` 泄漏登记，本回调在 state 关闭时至多
  // 触发一次，紧随其后 ecb 整体清零，不再有其它持有者。先在只读判定后结束借用，再以
  // `from_mut` 显式转手裸指针做配对回收（Shared 由创建者持有，不得在此回收）。
  if let Some(ctx) = unsafe { get_code_gen_context(l) }
    && ctx.kind == CodeGenContextKind::Standalone
  {
    let raw: *mut BaseCodeGenContext = from_mut(ctx);
    drop(unsafe { Box::from_raw(raw) });
  }

  // Safety: `global` 已判空，覆写活结构的 ecb 字段为默认值（与 cpp 语义一致）。
  unsafe { (*global).ecb = zero_execution_callbacks() };
  // T3 单载荷门：进程级武装槽随 state 关闭解除——计数单元（注册表 Box）随
  // forn_traces 整体 drop，槽内悬垂指针必须先行清除。
  FORN_HEAT_ARMED.store(null_mut(), Ordering::Relaxed);
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn on_close_state_export(l: *mut LuaState) {
  // Safety: 导出 C ABI 入口原样转发 l 给同契约 unsafe fn on_close_state; 该内部函数自带 l/global
  // 判空早返回, 故传空亦安全, 满足被调前置条件。
  unsafe { on_close_state(l) };
}
