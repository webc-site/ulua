use ulua_vm::{functions::lua_ref::lua_ref, records::lua_state::LuaState};

use crate::functions::counters_init::G_COUNTERS;

// Faithful port of:
//     void counters_track(LuaState* l, int funcindex) {
//         int ref = lua_ref(l, funcindex);
//         G_COUNTERS.moduleRefs.push_back(ref);
//     }
pub(crate) fn counters_track(l: *mut LuaState, funcindex: i32) {
  // Safety: `l` 指向存活的 `LuaState` 且 `funcindex` 为有效栈索引
  // （调用方 run_file / load 已把模块表压栈）。
  let ref_id = unsafe { lua_ref(l, funcindex) };
  G_COUNTERS.with(|counters| {
    counters.borrow_mut().module_refs.push(ref_id);
  });
}
