use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::freeze::freeze,
  records::{
    frontend::Frontend, frontend_options::FrontendOptions, null_file_resolver::NullFileResolver,
    null_module_resolver::NullModuleResolver,
  },
};
use ulua_common::fflag;
use ulua_vm::{
  functions::lua_setfield::lua_setfield,
  macros::{lua_newtable::lua_newtable, lua_setglobal::lua_setglobal},
  records::lua_state::LuaState,
};

use crate::common::functions::{cstr::cstr, populate_rtti::populate_rtti};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_types_setup(l: *mut LuaState) {
  // 分析侧的 Frontend 搭建是纯 safe Rust（`new_boxed` 门面 + `freeze`），不触碰 `l`，
  // 故先于 unsafe 段成形。
  let _module_resolver = NullModuleResolver::new();
  let mut file_resolver = NullFileResolver::new();
  let mode = if fflag::DebugLuauForceOldSolver.get() {
    SolverMode::Old
  } else {
    SolverMode::New
  };

  // cpp `Conformance.test.cpp:2025`：`Frontend frontend{mode, &fileResolver, &configResolver}`。
  // `new_boxed` 在 safe 边界内完成「构造 → 堆上落位 → 自指针布线」全序列，本入口
  // 免手写 unsafe ctor + `wire_self_pointers`；`file_resolver` 是本函数局部且
  // frontend 后声明（Box 内的 frontend 先析构），句柄覆盖整个使用期；C++
  // `configResolver` 缺位由 `None` 显式承载（同 nullptr 语义，本测试不查 getConfig）。
  let mut frontend =
    Frontend::new_boxed(mode, &mut file_resolver, None, FrontendOptions::default());

  // cpp `Conformance.test.cpp:2026-2028`：`registerBuiltinGlobals(frontend,
  // frontend.globals)` → `freeze(frontend.globals.globalTypes)`。上游那两个引用
  // 实参（整体 + 字段）的重叠借用已在 #6 wave2 步④收进 ulua-analysis 的单
  // `&mut Frontend` 门面 `Frontend::register_builtin_globals`，本入口全程普通借用。
  frontend.register_builtin_globals(false);
  freeze(frontend.globals.global_types_mut());

  // Safety: `l` 为本用例存活的 LuaState；建空表作为 RTTI 容器。
  unsafe { lua_newtable(l) };

  let global_scope = frontend.globals.global_scope();
  for (name, binding) in &global_scope.bindings {
    // Safety: `l` 存活且栈顶为上面的 RTTI 表；`binding.type_id` 来自刚完成分析的
    // global scope（`populate_rtti` 自行压入类型描述）。
    unsafe { populate_rtti(l, binding.type_id) };
    // Safety: 栈顶仍为 RTTI 表（-2 取它）；`name.ast_name().value` 是 NUL 结尾指针
    // （Symbol::c_str 已随其他代理重构移除）。批 2 后 value 为 `*const u8`，
    // `lua_setfield` 是 C ABI（`const char*`）收口点，cast 仅符号标记。
    unsafe { lua_setfield(l, -2, name.ast_name().value.cast()) };
  }

  // Safety: `l` 存活；把栈顶 RTTI 表登记为全局，与 cpp 一致。
  unsafe { lua_setglobal(l, cstr(b"RTTI\0")) };
}
