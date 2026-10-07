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
  repl_main::repl_codegen_enabled,
};

/// DELIBERATE DEVIATION（review.md §2/§9.3）：cpp `setupState(LuaState* L)`
/// （`CLI/src/Repl.cpp`，经 `Repl.h` 暴露给测试）是 VM c-API 初始化边界，逐步调用
/// `luau_codegen_create`/`luaL_openlibs`/`luaL_register`/`luaopen_require`/
/// `luaL_sandbox`。本 port 把句柄收编为借用 `&mut LuaState`：存活/独占前提由类型承载，
/// 故入口降级为安全 `fn`，原先的 `# Safety` 契约上移到各调用方唯一的裸指针物化点
/// （`repl_main`/`run_repl` 的 `LuaStateGuard` 句柄、`ulua-cli-test` 夹具的
/// `lua_l_newstate` 结果），仍是 `unsafe` 导出的 `lua_*` 调用以带 `// Safety:` 的最小
/// 块就地使用。
///
/// 调用序契约（由 `&mut` 承载存活，本处补足初始化次序）：`l` 必须是刚创建、尚未
/// openlibs/sandbox 的 `LuaState`。
pub fn setup_state(l: &mut LuaState) {
  // Safety: `luau_codegen_create` 为 unsafe 导出；l 为调用方（run_repl/repl_main/
  // cli-test 夹具路径）刚创建的有效状态（本 fn 调用序契约）。
  if repl_codegen_enabled() {
    unsafe { luau_codegen_create(l) };
  }

  // `lua_l_openlibs` 为引用形安全面：`l` 直传借用（review.md §3）。
  lua_l_openlibs(l);

  // Note: a CALLGRIND build also registers {"callgrind", lua_callgrind}; the
  // upstream default (non-CALLGRIND) build registers only these two.
  // 注册表是纯数据构造，安全代码内完成
  let funcs: [LuaLReg; 2] = [
    LuaLReg::new(b"loadstring", lua_loadstring),
    LuaLReg::new(b"collectgarbage", lua_collectgarbage),
  ];

  l.push_value(LUA_GLOBALSINDEX);
  // lua_l_register_bytes 已降为安全 fn（r12-w6d，其内部裸 C 函数指针/lua_s_new 转手
  // 由被调自身窄块与契约承担）；`l` 为引用形直传，funcs 是本地数组，
  // lua_l_register_bytes 在调用窗口内读取其 name/func 字段（切片与 Some(fn) 指针均静态存活）。
  // bytes 核心形以 `None` 原生表达「注册到当前栈顶」（cpp luaL_register(L, NULL, l) 的
  // NULL 名参语义），无旧 c_char 形的 FFI 折算需求（该保留 null 论证随改道消亡，r16-v10）。
  lua_l_register_bytes(l, None, &funcs);
  // push/pop 配平收回 LUA_GLOBALSINDEX。
  l.pop(1);

  // luaopen_require 已收形为安全 fn（`l` 以 `&mut LuaState` 直传，免在边界折回裸
  // 指针，review.md §2/§3）；其调用序契约在本处成立：宿主随 require 闭包的 userdata
  // 由 GC 持有（内部 lua_newuserdatadtor 装箱），与状态同生命周期。
  luaopen_require(l, create_cli_require_context());

  // Safety: `lua_l_sandbox` 仍为 unsafe 导出（`lua_setsafeenv` 裸形屏障）；只原地冻结全局表。
  unsafe { lua_l_sandbox(l) };
}
