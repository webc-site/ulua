use ulua_vm::{functions::lua_ref::lua_ref, records::lua_state::LuaState};

use crate::functions::coverage_init::G_COVERAGE;

// Faithful port of:
//     void coverage_track(LuaState* l, int funcindex) {
//         int ref = lua_ref(l, funcindex);
//         gCoverage.functions.push_back(ref);
//     }
pub(crate) fn coverage_track(l: *mut LuaState, funcindex: i32) {
  // Safety: `l` 指向存活的 `LuaState` 且 `funcindex` 为有效栈索引
  // （调用方 run_file / load 已把模块表压栈）。
  let ref_id = unsafe { lua_ref(l, funcindex) };
  G_COVERAGE.with(|coverage| {
    coverage.borrow_mut().functions.push(ref_id);
  });
}
