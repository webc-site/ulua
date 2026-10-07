use ulua_vm::{functions::lua_ref::lua_ref, records::lua_state::LuaState};

use crate::functions::{counters_init::G_COUNTERS, state_ref::state};

// Faithful port of:
//     void counters_track(LuaState* l, int funcindex) {
//         int ref = lua_ref(&mut *l, funcindex);
//         G_COUNTERS.moduleRefs.push_back(ref);
//     }
pub(crate) fn counters_track(l: *mut LuaState, funcindex: i32) {
  // `l` 为 run_file / load 路径交出的存活句柄（`state` 门面契约，funcindex 为
  // 有效栈索引）；物化后 `lua_ref` 系 ulua-vm 安全封装，原 `unsafe { &mut *l }`
  // 裸指针解引用随之消失。
  let ref_id = lua_ref(state(l), funcindex);
  G_COUNTERS.with(|counters| {
    counters.borrow_mut().module_refs.push(ref_id);
  });
}
