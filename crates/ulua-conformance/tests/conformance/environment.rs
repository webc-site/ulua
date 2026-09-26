// _G 沙箱与去库环境用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_safe_env() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("safeenv.luau");
}

#[test]
fn conformance_sandbox_without_libs() {
  use ulua_vm::{
    functions::{
      lua_getreadonly::lua_getreadonly, lua_l_sandbox::lua_l_sandbox, luaopen_base::luaopen_base,
    },
    macros::lua_globalsindex::LUA_GLOBALSINDEX,
  };

  use crate::common::functions::new_state::new_state;

  let global_state = new_state();
  let l = global_state.as_ptr();

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`global_state` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    luaopen_base(l);
    lua_l_sandbox(l);

    assert_ne!(lua_getreadonly(l, LUA_GLOBALSINDEX), 0);
  }
}
