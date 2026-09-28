// _G 沙箱与去库环境用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_safe_env() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("safeenv.luau");
}

#[test]
fn conformance_sandbox_without_libs() {
  use ulua_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;

  use crate::common::functions::{new_state::new_state, safe_api::*};

  let global_state = new_state();
  let l = global_state.as_ptr();

  open_base(l);
  sandbox(l);

  assert_ne!(getreadonly(l, LUA_GLOBALSINDEX), 0);
}
