use crate::{
  functions::pcg_32_seed::pcg_32_seed, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v40 收形后
/// 取参与 rng 写点全经安全门面——`check_integer` 与 `gs_mut` 一句一借——体内无裸操作，故降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1 号位经 `check_integer` 取整数种子（非整数抛错发散）；
/// `gs_mut` 门面挂靠 `&mut l` 借出 `global_State` 可变视图（其 `global` 恒定非空由状态不变量保证），
/// `pcg_32_seed` 就地重置种子，不分配/不 GC。cpp/VM/src/lmathlib.cpp:278 math_randomseed。
pub(crate) fn math_randomseed(l: &mut LuaState) -> i32 {
  let seed = l.check_integer(1) as u64;

  pcg_32_seed(&mut l.gs_mut().rngstate, seed);

  0
}

lua_lib_fn!(pub(crate) fn math_randomseed @ref, math_randomseed_arm);
