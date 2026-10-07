use crate::{
  functions::{lua_l_tolstring::lua_l_tolstring_ref, writestring::writestring},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// base 库 `print` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （受保护帧内执行）：逐槽 `lua_l_tolstring_ref` 串化可回跑 `__tostring`、可分配、
/// 可抛错，转换结果压栈后随即 `lua_pop` 回收；stdout 写入本身不感知 VM。
/// cpp `lbaselib.cpp:23` print。
pub fn lua_b_print(l: &mut LuaState) -> i32 {
  let n = l.get_top();
  for i in 1..=n {
    // `None`（`__tostring` 发散前的不可达形态）与旧 null 指针同为空串输出。
    // SAFETY: `l` 存活（引用形保证）；`lua_l_tolstring_ref` 的 `# Safety` 其余前提
    // （i 为 1..=n 的合法正索引、受保护帧）由循环界与库函数约定成立。
    let s = unsafe { lua_l_tolstring_ref(l, i) }.unwrap_or_default();
    if i > 1 {
      writestring(b"\t");
    }
    writestring(s);
    l.pop(1);
  }
  writestring(b"\n");
  0
}

lua_lib_fn!(pub fn lua_b_print @ref, lua_b_print_arm);
