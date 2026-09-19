//! ulua-vm 行为契约测试：字符串 intern、GC 状态机。
//! 对照 cpp/VM 的 lstring.cpp(luaS_newlstr) / lapi.cpp(lua_gc)。

use core::{ffi::c_void, slice::from_raw_parts};
use std::{
  ffi::c_char,
  ptr::{eq, null},
  str::from_utf8,
};

use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{
    lua_close::lua_close, lua_gc::lua_gc, lua_getfield::lua_getfield, lua_gettop::lua_gettop,
    lua_l_newstate::lua_l_newstate, lua_pcall::lua_pcall, lua_pushinteger::lua_pushinteger,
    lua_pushlstring::lua_pushlstring, lua_s_newlstr::luaS_newlstr, lua_settop::lua_settop,
    lua_tolstring::lua_tolstring, lua_tonumberx::lua_tonumberx, luaopen_string::luaopen_string,
  },
  type_aliases::lua_state::lua_State,
};

/// 独立 VM 状态的 RAII 守卫，测试退出即释放
struct State {
  l: *mut lua_State,
}

impl State {
  fn new() -> Self {
    let l = lua_l_newstate();
    assert!(!l.is_null(), "lua_l_newstate 失败");
    Self { l }
  }

  /// 以 Rust 字节切片 intern 字符串（内嵌 NUL 由长度驱动，非 C 截断）
  unsafe fn intern(&self, bytes: &[u8]) -> *mut c_void {
    unsafe { luaS_newlstr(self.l, bytes.as_ptr() as *const c_char, bytes.len()).cast() }
  }
}

impl Drop for State {
  fn drop(&mut self) {
    unsafe { lua_close(self.l) };
  }
}

/// 相同字节序列必须 intern 到同一 TString 指针（cpp luaS_newlstr 语义）
#[test]
fn intern_returns_identical_pointer_for_equal_bytes() {
  let s = State::new();
  unsafe {
    let a = b"hello luau intern test";
    let ts1 = s.intern(a);
    let ts2 = s.intern(a);
    assert!(!ts1.is_null());
    assert!(eq(ts1, ts2), "等值字符串必须复用同一 intern 对象");

    // 前缀相同但长度不同 → 不同对象（长度参与判定）
    let ts3 = s.intern(b"hello luau intern tes");
    assert!(!eq(ts1, ts3));

    // 内嵌 NUL：长度语义，不能被当作 C 字符串在 NUL 处截断
    let ts4 = s.intern(b"ab\0cd");
    let ts5 = s.intern(b"ab\0cd");
    assert!(eq(ts4, ts5));
    let ts6 = s.intern(b"ab");
    assert!(!eq(ts4, ts6), "len 语义必须区分内嵌 NUL");
  }
}

/// GC 状态机：Stop/Restart 切换 isrunning；未知操作码返回 -1（cpp lua_gc default 分支）
#[test]
fn gc_stop_restart_isrunning_contract() {
  let s = State::new();
  unsafe {
    assert_eq!(
      lua_gc(s.l, LuaGcOp::Isrunning as i32, 0),
      1,
      "新状态默认在跑"
    );
    assert_eq!(lua_gc(s.l, LuaGcOp::Stop as i32, 0), 0);
    assert_eq!(lua_gc(s.l, LuaGcOp::Isrunning as i32, 0), 0);
    assert_eq!(lua_gc(s.l, LuaGcOp::Restart as i32, 0), 0);
    assert_eq!(lua_gc(s.l, LuaGcOp::Isrunning as i32, 0), 1);

    // Count 以 KB 计（totalbytes >> 10），基础状态已 > 1KB
    assert!(lua_gc(s.l, LuaGcOp::Count as i32, 0) >= 1);
    // 越界操作码 → -1
    assert_eq!(lua_gc(s.l, 42, 0), -1);
  }
}

/// 全量 GC 后被引用的字符串仍 intern：栈持有引用（cpp 标记阶段 root），fullgc 存活
#[test]
fn intern_survives_full_gc_when_referenced() {
  let s = State::new();
  unsafe {
    let k = b"persistent-key";
    let ts1 = s.intern(k);
    // 栈上持有引用，标记阶段可达 → 存活
    lua_pushlstring(s.l, k.as_ptr() as *const c_char, k.len());
    // 制造临时字符串再 fullgc
    for i in 0..64 {
      let buf = format!("garbage-{i}-{}", "x".repeat(64));
      s.intern(buf.as_bytes());
    }
    assert_eq!(lua_gc(s.l, LuaGcOp::Collect as i32, 0), 0);
    let ts2 = s.intern(k);
    assert!(eq(ts1, ts2), "被引用字符串 fullgc 后 intern 必须一致");
    lua_settop(s.l, 0);
  }
}

/// 未被引用的字符串会被 fullgc 回收（cpp sweep 语义）：GC 后内存计数下降
#[test]
fn unreferenced_string_collectable_by_full_gc() {
  let s = State::new();
  unsafe {
    let count_before = lua_gc(s.l, LuaGcOp::Count as i32, 0);
    // 不持有任何引用
    for i in 0..64 {
      let buf = format!("ephemeral-{i}-{}", "x".repeat(64));
      s.intern(buf.as_bytes());
    }
    let count_peak = lua_gc(s.l, LuaGcOp::Count as i32, 0);
    assert!(count_peak > count_before, "intern 应增加内存计数");
    assert_eq!(lua_gc(s.l, LuaGcOp::Collect as i32, 0), 0);
    let count_after = lua_gc(s.l, LuaGcOp::Count as i32, 0);
    assert!(
      count_after < count_peak,
      "不可达字符串必须被 fullgc 回收（{count_peak} → {count_after}）"
    );
  }
}

/// 空串 intern 契约：零长度指向同一对象
#[test]
fn string_rep_preserves_bytes_and_stack() {
  let s = State::new();
  unsafe {
    luaopen_string(s.l);
    for bytes in [b"".as_slice(), b"a", b"t\0\xff", &[0xff; 1024]] {
      for n in [-1, 0, 1, 2, 3, 31, 32, 33] {
        lua_getfield(s.l, 1, c"rep".as_ptr());
        lua_pushlstring(s.l, bytes.as_ptr().cast(), bytes.len());
        lua_pushinteger(s.l, n);
        assert_eq!(lua_pcall(s.l, 2, 1, 0), 0);
        assert_eq!(lua_gettop(s.l), 2);
        let mut len = 0;
        let p = lua_tolstring(s.l, -1, &mut len);
        let expected = bytes.repeat(n.max(0) as usize);
        assert_eq!(from_raw_parts(p.cast::<u8>(), len), expected);
        lua_settop(s.l, 1);
      }
    }
    lua_getfield(s.l, 1, c"rep".as_ptr());
    lua_pushlstring(s.l, c"".as_ptr(), 0);
    lua_pushinteger(s.l, 1_000_000_000);
    assert_eq!(lua_pcall(s.l, 2, 1, 0), 0);
    let mut len = 1;
    lua_tolstring(s.l, -1, &mut len);
    assert_eq!(len, 0);
  }
}

#[test]
fn intern_empty_string() {
  let s = State::new();
  unsafe {
    let a = s.intern(b"");
    assert!(!a.is_null());
    let b = luaS_newlstr(s.l, null(), 0).cast::<c_void>();
    assert!(eq(a, b), "零长度 intern 必须命中同一对象");
  }
}

/// 字符串→number 的 API 级契约（tonumber 走 luaO_str2d，经 lua_tonumberx 暴露）
#[test]
fn tonumber_string_conversion_via_api() {
  let s = State::new();
  unsafe {
    let mut cases: Vec<(&[u8], Option<f64>)> = vec![
      (b"10", Some(10.0)),
      (b"0x10", Some(16.0)),
      (b" 3.5", Some(3.5)),
      (b"1e2", Some(100.0)),
      (b"abc", None),
      (b"12xyz", None),
    ];
    for (bytes, expect) in cases.drain(..) {
      lua_pushlstring(s.l, bytes.as_ptr() as *const c_char, bytes.len());
      let mut isnum: i32 = 0;
      let v = lua_tonumberx(s.l, -1, &mut isnum);
      match expect {
        Some(e) => {
          assert_eq!(isnum, 1, "{:?} 应可转换", from_utf8(bytes));
          assert_eq!(v, e);
        }
        None => assert_eq!(isnum, 0, "{:?} 应拒绝转换", from_utf8(bytes)),
      }
      lua_settop(s.l, 0);
    }
  }
}
