use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring_ref, lua_l_checkstack::lua_l_checkstack,
    lua_l_optinteger::lua_l_optinteger, posrelat::posrelat,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, uchar::uchar},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v38 收形后
/// 取参/校验/扩容全经安全门面/已收形被调）：`l` 须处于可抛错受保护帧；栈槽 #1 为串实参
/// （非串经 `lua_l_checklstring_ref` 抛 "string expected" 发散）、#2/#3 为可选下标
/// （`lua_l_optinteger` 要求可转整数、越界实参由 `posrelat` 钳位）；`lua_l_checkstack`
/// 需为 n 个结果预留槽位，`push_integer` 逐字节压栈。
///
/// 唯一的裸操作面是「源串窗口跨逐字节压栈存活」（p28 锚定形 × `&mut` 接收者不可共存），
/// 按 r16-v29 桥接判例收在末块的一次就地转手内，借用窗止于该块，故本函数据 r16-v21 判例
/// 降为安全 `fn`。
pub fn str_byte(l: &mut LuaState) -> i32 {
  // 借用切片形态取源串（锚定形）：首次派窗止于取长，兼作 cpp `luaL_checklstring` 的先位
  // 抛错件（"string expected" 必先于 #2/#3 的 optinteger），其后 `l` 复原可借
  let len = lua_l_checklstring_ref(l, 1).len();
  let mut posi = posrelat(lua_l_optinteger(l, 2, 1), len);
  let mut pose = posrelat(lua_l_optinteger(l, 3, posi), len);

  if posi <= 0 {
    posi = 1;
  }
  if (pose as usize) > len {
    pose = len as i32;
  }

  if posi > pose {
    return 0; // empty interval; return no values
  }

  let n = pose - posi + 1;
  // oracle 同位死守卫（lstrlib.cpp:144 `if (posi + n <= pose) // overflow?`）：posi/pose 已钳位后
  // 恒假——忠实保留，防 sync-cpp 时漂移
  if posi + n <= pose {
    // overflow?
    luaL_error!(l, "string slice too long");
  }

  lua_l_checkstack(l, n, "string slice too long");

  // SAFETY: 二次派窗直达逐字节压栈——窗口须活过整段 `push_integer`（cpp 的 `c[posi]`
  // 游标同形），与压栈的 `&mut l` 借用不可共存（p28 锚定形），故按 r16-v29 桥接判例在块内
  // 一次就地转手裸句柄，借用窗止于本块；同槽同值故与首次派窗观测等价。
  // 钳位保证 1 <= posi <= pose <= len，[posi-1, pose) 恒界内。
  unsafe {
    let lp = l.as_mut_ptr();
    let s = lua_l_checklstring_ref(&mut *lp, 1);
    debug_assert_eq!(s.len(), len);

    for &b in &s[(posi - 1) as usize..pose as usize] {
      (*lp).push_integer(uchar(b as i32) as i32);
    }
  }

  n
}

lua_lib_fn!(pub fn str_byte @ref, str_byte_arm);
