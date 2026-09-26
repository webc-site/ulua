//! `lua_tolstring` C-ABI 垫片的转换契约（对照 cpp lapi.cpp:497-510）：
//! 非串数值槽经 `luaV_tostring` **就地**改写为字符串（分配新串、`lua_c_check_gc`
//! 挪栈后重取槽位），失败路径（不可转换值）返回 NULL 且 `*len = 0`、槽位类型
//! 不变。数值转串的字节由 lnumprint.cpp 的 `luai_num2str`（schubfach 最短表示）
//! 规定；整数值 itoa 快路径与 `-0.0` 慢路径的分流见 lua_v_tostring 的实现。

use core::slice::from_raw_parts;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_close::lua_close, lua_createtable::lua_createtable, lua_l_newstate::lua_l_newstate,
    lua_pushnil::lua_pushnil, lua_pushnumber::lua_pushnumber, lua_settop::lua_settop,
    lua_tolstring::lua_tolstring, lua_type::lua_type,
  },
  records::lua_state::LuaState,
};

/// 以 lua.h 的 `(const char*, size_t* len)` 出参形态读取 `idx` 槽：NULL 返回
/// 折叠为 None；`len` 预置哨兵，成功/失败两条路径都必须覆写它。
unsafe fn tobytes(l: *mut LuaState, idx: i32) -> Option<Vec<u8>> {
  let mut len = usize::MAX;
  // Safety: `l` 为本测试独占持有的存活 VM；idx 指向本测试刚压入的槽位，
  // `len` 为本地可写 usize；返回指针指向 Lua 串缓冲（恒有终止 NUL）。
  let p = unsafe { lua_tolstring(l, idx, &mut len) };
  if p.is_null() {
    assert_eq!(len, 0, "失败路径必须把出参 len 归零");
    return None;
  }
  // Safety: 指针与 len 来自同一次成功调用，指向槽内串的 payload 字节
  Some(unsafe { from_raw_parts(p.cast::<u8>(), len) }.to_vec())
}

/// 数值槽就地转串：字节符合 lnumprint 最短表示；槽位类型翻转为 string；
/// 已是串后再读不得二次转换（字节稳定）。整数值命中 itoa 快路径，
/// 3.5 走 schubfach，-0.0 被快路径显式放行到慢路径以保留 `-0`。
#[test]
fn number_slot_converts_in_place_to_string() {
  let l = lua_l_newstate();
  assert!(!l.is_null(), "lua_l_newstate 失败");
  for (n, want) in [
    (42.0, b"42".as_slice()),
    (-17.0, b"-17"),
    (0.0, b"0"),
    (3.5, b"3.5"),
    (-0.0, b"-0"),
  ] {
    // Safety: `l` 存活且独占；本块仅做压栈/读槽/弹栈的常规 C-API 操作。
    unsafe {
      lua_pushnumber(l, n);
      assert_eq!(
        lua_type(l, -1),
        LuaType::Number as i32,
        "前置：压入的是数值"
      );
      let first = tobytes(l, -1);
      let second = tobytes(l, -1);
      assert_eq!(first.as_deref(), Some(want), "tostring({n}) 字节不符");
      assert_eq!(second, first, "已是串后重复读取必须稳定");
      assert_eq!(
        lua_type(l, -1),
        LuaType::String as i32,
        "槽位必须就地翻转为 string（cpp luaV_tostring 改写原槽）"
      );
      lua_settop(l, 0);
    }
  }
  // Safety: `l` 由上方断言非空后独占持有，测试末尾关闭。
  unsafe { lua_close(l) };
}

/// 不可转换的槽位（nil / 表）：返回 NULL、len 归零、类型保持不变
/// （cpp 失败路径：`luaV_tostring` 返回 0 → `*len = 0` + NULL）。
#[test]
fn non_convertible_slot_returns_null_and_keeps_type() {
  let l = lua_l_newstate();
  assert!(!l.is_null(), "lua_l_newstate 失败");
  // Safety: `l` 存活且独占；本块仅做压栈/读槽/弹栈的常规 C-API 操作。
  unsafe {
    lua_pushnil(l);
    assert_eq!(tobytes(l, -1), None, "nil 不可转串");
    assert_eq!(lua_type(l, -1), LuaType::Nil as i32, "nil 槽不得被改写");

    lua_createtable(l, 0, 0);
    assert_eq!(tobytes(l, -1), None, "表不可转串");
    assert_eq!(lua_type(l, -1), LuaType::Table as i32, "表槽不得被改写");

    lua_settop(l, 0);
    lua_close(l);
  }
}
