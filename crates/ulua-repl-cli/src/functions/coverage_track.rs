use ulua_vm::{functions::lua_ref::lua_ref, records::lua_state::LuaState};

use crate::functions::coverage_init::G_COVERAGE;

// Faithful port of:
//     void coverage_track(LuaState* l, int funcindex) {
//         int ref = lua_ref(&mut *l, funcindex);
//         gCoverage.functions.push_back(ref);
//     }
// review.md §2/§3 收形：`l` 由裸 `*mut LuaState` 收编为借用 `&mut LuaState`（真实物化点
// 只在 run_file / load 交出 `lua_newthread` 句柄的边界处一次），`lua_ref` 系 ulua-vm
// 安全引用形，本函数体内不再有 `unsafe` 面。
pub(crate) fn coverage_track(l: &mut LuaState, funcindex: i32) {
  let ref_id = lua_ref(l, funcindex);
  G_COVERAGE.with(|coverage| {
    coverage.borrow_mut().functions.push(ref_id);
  });
}
