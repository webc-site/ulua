use crate::{
  functions::{
    lua_l_optinteger::lua_l_optinteger, lua_o_str_2_l::lua_o_str_2_l,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位为字符串（`check_bytes` 返回本帧存活字节切片），
/// 2 号位可选基数经 `lua_l_optinteger`/`arg_check` 校验落在 2..=36；
/// `lua_pushinteger_64`/`push_nil` 写回可分配/GC。cpp/VM/src/lintlib.cpp:37 int64_fromstring。
pub unsafe fn int64_fromstring(l: *mut LuaState) -> i32 {
  unsafe {
    let s = (*l).check_bytes(1);
    let base = lua_l_optinteger(&mut *l, 2, 10);
    (*l).arg_check((2..=36).contains(&base), 2, "base out of range");

    // SAFETY: check_bytes 返回栈上存活字符串切片
    match lua_o_str_2_l(s, base) {
      Some(result) => lua_pushinteger_64(&mut *l, result),
      None => (*l).push_nil(),
    }

    1
  }
}

lua_lib_fn!(pub fn int64_fromstring, int64_fromstring_arm);
