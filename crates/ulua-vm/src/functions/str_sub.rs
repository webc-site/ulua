use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring_ref, lua_l_optinteger::lua_l_optinteger,
    lua_pushlstring::lua_pushlstring_bytes, posrelat::posrelat,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v38 收形后
/// 取参/钳位全经安全门面/已收形被调）：`l` 须处于可抛错受保护帧；栈槽 #1 为串实参（非串经
/// `lua_l_checklstring_ref` 抛 "string expected" 发散，严格先于 #2 的 `check_integer`）、
/// #2 为必填下标、#3 为可选下标（`lua_l_optinteger`）；结果串压栈需 `top` 后 ≥1 空槽。
///
/// 唯一的裸操作面是「子串窗口跨压栈存活」（p28 锚定形 × `&mut` 接收者不可共存），按 r16-v29
/// 桥接判例收在当句的一次就地转手内，故本函数据 r16-v21 判例降为安全 `fn`。
pub fn str_sub(l: &mut LuaState) -> i32 {
  // 借用切片形态取源串（锚定形）：首次派窗止于取长，兼作 cpp `luaL_checklstring` 的先位
  // 抛错件，其后 `l` 复原可借；后续按下标取子串，免指针游走
  let len = lua_l_checklstring_ref(l, 1).len();
  let mut start = posrelat(l.check_integer(2), len);
  let mut end = posrelat(lua_l_optinteger(l, 3, -1), len);

  if start < 1 {
    start = 1;
  }
  if end > len as i32 {
    end = len as i32;
  }

  if start <= end {
    // SAFETY: `l` 为借用形式的存活调用帧（&mut 保证有效且独占），as_mut_ptr 由该借用重取裸
    // 指针，借用窗止于本块。二次派窗取回同一栈槽串体（同槽同值，观测等价），令子串切片与
    // 同句的压栈（亦经 `&mut l`）共存。上方钳位保证 1 <= start <= end <= len，区间恒界内。
    unsafe {
      let lp = l.as_mut_ptr();
      let s = lua_l_checklstring_ref(&mut *lp, 1);
      debug_assert_eq!(s.len(), len);
      lua_pushlstring_bytes(&mut *lp, &s[(start - 1) as usize..end as usize]);
    }
  } else {
    l.push_bytes(b"");
  }
  1
}

lua_lib_fn!(pub fn str_sub @ref, str_sub_arm);
