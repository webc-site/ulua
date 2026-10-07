use crate::{
  functions::lua_pushthread::lua_pushthread, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票把首参收形为
/// 引用形后，取线程/压栈全经 `lua_pushthread`/`push_nil` 安全门面，体内已无裸指针解引用，故本体降为
/// 安全 `fn`）：`l` 须处于可分配/GC 的受保护帧，栈顶之上至少留 1 个空槽（`lua_pushthread` 主线程时先
/// 占一槽、随后 `push_nil` 补一槽作返回值）。cpp/VM/src/lcorolib.cpp:354 corunning。
pub(crate) fn corunning(l: &mut LuaState) -> i32 {
  if lua_pushthread(l) != 0 {
    l.push_nil(); // main thread is not a coroutine
  }
  1
}

lua_lib_fn!(pub(crate) fn corunning @ref, corunning_arm);
