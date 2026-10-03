use core::slice;

use crate::{
  functions::{lua_l_buffinitsize::lua_l_buffinitsize, lua_l_pushresultsize::lua_l_pushresultsize},
  macros::{lua_lib_fn::lua_lib_fn, uchar::uchar},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：`l` 须处于
/// 可抛错受保护帧，`get_top` 取实参数 n；对索引 1..=n `check_integer`+`arg_check` 校验落在
/// 0..=255（越界抛错回退）；`pushresultsize` 提交 n 字节需 `top` 后 ≥1 空槽；分配可触发 GC。
/// cpp VM/src/lstrlib.cpp:152
///
/// # Safety
/// `unsafe fn` 屏障按 r16-v21 判例保留：目标区仍由 `lua_l_buffinitsize`（引用形被调，经
/// `&mut *l` 再借，借用窗止于当句）返回的裸 `ptr` 经 `slice::from_raw_parts_mut` 成窗，
/// 逐槽写入受该切片边界约束。
pub unsafe fn str_char(l: &mut LuaState) -> i32 {
  unsafe {
    let n = l.get_top(); // number of arguments

    let mut b = LuaLStrbuf::new();
    let ptr = lua_l_buffinitsize(&mut *l, &mut b, n as usize);

    // 输出切片 zip 参数序号：写入受切片边界约束，消除手写 offset 算术
    // （切片源于裸 `ptr`，不借 `l`，故循环内的 `&mut l` 取参不与任何窗口冲突）
    for (i, slot) in slice::from_raw_parts_mut(ptr, n as usize)
      .iter_mut()
      .enumerate()
    {
      let c = l.check_integer(i as i32 + 1);
      l.arg_check(i32::from(uchar(c)) == c, i as i32 + 1, "invalid value");

      *slot = uchar(c);
    }
    lua_l_pushresultsize(&mut b, n as usize);
    1
  }
}

lua_lib_fn!(pub fn str_char @ref, str_char_arm);
