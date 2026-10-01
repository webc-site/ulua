use crate::{
  functions::{cstr, cstr_cow, lua_l_optlstring::lua_l_optlstring},
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// base 库 `assert` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （实参自 1 号槽布栈、受保护帧内执行），否则抛错路径的文案槽读取失真。
/// cpp VM/src/lbaselib.cpp luaB_assert。
pub fn lua_b_assert(l: &mut LuaState) -> i32 {
  l.check_any(1);
  if !l.to_boolean(1) {
    let mut len = 0;
    // SAFETY: `l` 存活（引用形保证）；`lua_l_optlstring` 的 `# Safety` 其余前提
    // （2 号槽可读或无值、默认串为 NUL 结尾字面量、len 可写）由库函数约定与实参成立。
    let msg = unsafe { lua_l_optlstring(l, 2, cstr(b"assertion failed!\0"), &mut len) };
    // SAFETY: `msg` 为 `lua_l_optlstring` 返回的 NUL 结尾串指针（默认串或 2 号槽串），
    // 本调用内未被回收；cstr_cow 只读建立字节串视图。
    let msg = unsafe { cstr_cow(msg) };
    // SAFETY: 抛错族契约——`l` 存活且处于受保护帧（库函数调用约定），本调用不返回。
    unsafe { luaL_error!(l.as_mut_ptr(), "{}", msg) };
  }
  l.get_top()
}

lua_lib_fn!(pub fn lua_b_assert @ref, lua_b_assert_arm);
