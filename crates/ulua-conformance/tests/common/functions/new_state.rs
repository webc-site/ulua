use ulua_vm::functions::lua_l_newstate::lua_l_newstate;

use crate::common::records::state_ref::StateRef;

/// cpp `Conformance.test.cpp` 各用例的统一前置：`StateRef globalState(luaL_newstate(),
/// lua_close)` —— 建一个带全部标准库的 `lua_State`，并交给 `StateRef` 在作用域结束时
/// `lua_close`。分配失败即中止（上游此时会在首次解引用空指针处崩溃，这里给出显式诊断）。
///
/// 与 [`run_conformance`](crate::common::functions::run_conformance::run_conformance) 内部
/// 的空指针兜底等价，是「直接调 C API 的用例」的唯一 state 构造出口。
pub fn new_state() -> StateRef {
  StateRef::new(lua_l_newstate()).expect("lua state allocation failed")
}
