use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_tolstring::lua_l_tolstring_ref, writestring::writestring,
  },
  macros::lua_pop::lua_pop,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`：逐槽 `lua_l_tolstring_ref` 串化可回跑
/// `__tostring`、可分配、可抛错（须在受保护帧内调入），转换结果压栈后随即 `lua_pop` 回收；
/// stdout 写入本身不感知 VM。cpp `lbaselib.cpp:23` print。
pub unsafe extern "C-unwind" fn lua_b_print(l: *mut LuaState) -> i32 {
  unsafe {
    let n = lua_gettop(l);
    for i in 1..=n {
      // `None`（`__tostring` 发散前的不可达形态）与旧 null 指针同为空串输出
      let s = lua_l_tolstring_ref(l, i).unwrap_or_default();
      if i > 1 {
        writestring(b"\t");
      }
      writestring(s);
      // Safety: 弹出本轮 `lua_l_tolstring_ref` 压入的结果串，栈高复原。
      lua_pop(l, 1);
    }
    writestring(b"\n");
  }
  0
}
