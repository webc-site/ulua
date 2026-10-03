//! cpp `setupState`（`CLI/src/Repl.cpp`，经 `Repl.h` 暴露给测试使用）。

use ulua_cli_lib::functions::lua_collectgarbage::lua_collectgarbage;
use ulua_code_gen::functions::luau_codegen_create::luau_codegen_create;
use ulua_require::functions::luaopen_require::luaopen_require;
use ulua_vm::{
  functions::{
    lua_l_openlibs::lua_l_openlibs, lua_l_register::lua_l_register_bytes,
    lua_l_sandbox::lua_l_sandbox,
  },
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

use crate::functions::{
  create_cli_require_context::create_cli_require_context, lua_loadstring::lua_loadstring,
  repl_main::repl_codegen_enabled, state_ref::state,
};

/// DELIBERATE DEVIATION（review.md §9.3）：本函数是 VM c-API 初始化边界，逐步调用
/// `luau_codegen_create`/`luaL_openlibs`/`luaL_register`/`luaopen_require`/
/// `luaL_sandbox` 并以裸 `*mut LuaState` 收发，`unsafe` 与裸指针系 ulua-vm 边界固有
/// 形态，非纯 Rust 逻辑；理由即下条 `# Safety` 契约。
/// 保留 `pub unsafe fn` 定性：本入口是跨 crate `pub` 句柄边界（`Repl.h` 的 setupState），
/// 由 `ulua-cli-test`/`repl_main`/`run_repl` 以 `unsafe` 块断言刚创建状态有效；改 safe
/// fn 会撤掉这层调用方契约强制（且 `pub fn` 解引用裸指针形参将命中
/// `clippy::not_unsafe_ptr_arg_deref`），故属「确属 c-API 句柄边界」而非纯逻辑层，签名不动。
///
/// # Safety
///
/// `l` 必须是刚创建、有效的 `LuaState`。
pub unsafe fn setup_state(l: *mut LuaState) {
  // Safety: `# Safety` 契约保证 `l` 非空、活跃，经 `state` 门面物化后全走安全方法
  // （仍是 unsafe fn 的 `lua_*` 导出在各块内论证）。
  let l = state(l);
  // Safety: `luau_codegen_create` 为 unsafe 导出；l 为调用方（run_repl/repl_main 路径）
  // 刚创建的有效状态（fn /// # Safety）。
  if repl_codegen_enabled() {
    unsafe { luau_codegen_create(l) };
  }

  // r16-v3：`lua_l_openlibs` 已前移引用形；本 fn 为 unsafe fn 体（edition 2021 隐式
  // unsafe 语境），`&mut *l` 即时建引用无需块包裹。仅初始化标准库表。
  lua_l_openlibs(&mut *l);

  // Note: a CALLGRIND build also registers {"callgrind", lua_callgrind}; the
  // upstream default (non-CALLGRIND) build registers only these two.
  // 注册表是纯数据构造，安全代码内完成
  let funcs: [LuaLReg; 2] = [
    LuaLReg::new(b"loadstring", lua_loadstring),
    LuaLReg::new(b"collectgarbage", lua_collectgarbage),
  ];

  l.push_value(LUA_GLOBALSINDEX);
  // Safety: `lua_l_register_bytes` 仍为 unsafe fn（`lr` 裸 C 函数指针与 `lua_s_new`
  // 裸形转手的屏障），但 `l` 侧已收成引用形，本处传 `l` 是一次性隐式重借用、借用窗止于当句；
  // funcs 是本地数组，
  // lua_l_register_bytes 在调用窗口内读取其 name/func 字段（切片与 Some(fn) 指针均静态存活）。
  // bytes 核心形以 `None` 原生表达「注册到当前栈顶」（cpp luaL_register(L, NULL, l) 的
  // NULL 名参语义），无旧 c_char 形的 FFI 折算需求（该保留 null 论证随改道消亡，r16-v10）。
  unsafe { lua_l_register_bytes(l, None, &funcs) };
  // push/pop 配平收回 LUA_GLOBALSINDEX。
  l.pop(1);

  // Safety: `luaopen_require` 为 unsafe 导出；宿主随 require 闭包的 userdata 由 GC
  // 持有（luaopen_require 内部 lua_newuserdatadtor 装箱），与状态同生命周期。
  unsafe { luaopen_require(l, create_cli_require_context()) };

  // Safety: `lua_l_sandbox` 仍为 unsafe 导出（`lua_setsafeenv` 裸形屏障）；只原地冻结全局表。
  unsafe { lua_l_sandbox(l) };
}
