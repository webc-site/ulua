//! ulua-vm 行为契约测试：字符串 intern、GC 状态机。
//! 对照 cpp/VM 的 lstring.cpp(luaS_newlstr) / lapi.cpp(lua_gc)。

use core::{ffi::c_void, ptr::addr_of_mut};
use std::{ffi::c_char, ptr::eq};

use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{
    lua_gc::lua_gc,
    lua_getfield::lua_getfield,
    lua_gettop::lua_gettop,
    lua_o_rawequal_obj::lua_o_rawequal_obj,
    lua_pcall::lua_pcall,
    lua_pushinteger::lua_pushinteger,
    lua_pushlstring::lua_pushlstring,
    lua_s_newlstr::lua_s_newlstr,
    lua_settop::lua_settop,
    lua_tolstring::{lua_tolstring, lua_tolstring_ref},
    lua_tonumberx::lua_tonumberx,
    lua_v_equalval::lua_v_equalval_export,
    luaopen_string::luaopen_string,
  },
  macros::{setlvalue::setlvalue, setpvalue::setpvalue},
  type_aliases::t_value::TValue,
};

#[path = "common/state.rs"]
mod state;

use state::State;

/// 本文件用例的专属辅助，挂在共享的 `state::State` 上
impl State {
  /// 以 Rust 字节切片 intern 字符串（内嵌 NUL 由长度驱动，非 C 截断）。
  /// 入参已是切片，调用方无须再持有任何指针契约，故为安全签名。
  fn intern(&self, bytes: &[u8]) -> *mut c_void {
    // Safety: 单线程测试内 `self.l` 为存活 VM 状态；`bytes` 借用覆盖整个调用。
    unsafe { lua_s_newlstr(self.l, bytes).cast() }
  }
}

/// 相同字节序列必须 intern 到同一 TString 指针（cpp luaS_newlstr 语义）
#[test]
fn intern_returns_identical_pointer_for_equal_bytes() {
  let s = State::new();
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

/// GC 状态机：Stop/Restart 切换 isrunning；未知操作码返回 -1（cpp lua_gc default 分支）
#[test]
fn gc_stop_restart_isrunning_contract() {
  let s = State::new();
  // Safety: `s.l` 为 State 独占持有的存活 VM 状态，lua_gc 的 C ABI 仅需非空 L。
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
    // 越界操作码 → -1（IsPaused=10 补齐后不再是未知操作码）
    assert_eq!(lua_gc(s.l, 42, 0), -1);
    assert_ne!(lua_gc(s.l, LuaGcOp::IsPaused as i32, 0), -1);
  }
}

/// IsPaused（cpp LUA_GCISPAUSED，lapi.cpp `res = g->gcstate == GCSpause`）：
/// 新状态停在 GCSpause → 1；堆上造垃圾后一小步把状态机推进出 pause → 0；
/// 巨量步进跑完整周期回到 pause → 1。
#[test]
fn gc_ispaused_follows_gcstate_contract() {
  let s = State::new();
  // Safety: `s.l` 为存活 VM 状态；lua_gc/lua_pushlstring 仅要求非空 L 与可读字节区。
  unsafe {
    assert_eq!(
      lua_gc(s.l, LuaGcOp::IsPaused as i32, 0),
      1,
      "新状态必在 GCSpause"
    );

    // 制造足够多的存活对象，保证一小步走不完整个周期
    for i in 0..512 {
      let buf = format!("ispaused-garbage-{i}-{}", "y".repeat(256));
      lua_pushlstring(s.l, buf.as_ptr() as *const c_char, buf.len());
    }
    assert_eq!(lua_gc(s.l, LuaGcOp::Step as i32, 1), 0, "小步不应完成周期");
    assert_eq!(
      lua_gc(s.l, LuaGcOp::IsPaused as i32, 0),
      0,
      "步进后应离开 GCSpause"
    );

    lua_gc(s.l, LuaGcOp::Step as i32, 1024 * 1024);
    assert_eq!(
      lua_gc(s.l, LuaGcOp::IsPaused as i32, 0),
      1,
      "周期跑完必须回到 GCSpause"
    );
    lua_settop(s.l, 0);
  }
}

/// 全量 GC 后被引用的字符串仍 intern：栈持有引用（cpp 标记阶段 root），fullgc 存活
#[test]
fn intern_survives_full_gc_when_referenced() {
  let s = State::new();
  // Safety: `s.l` 存活；`k` 为 'static 字节常量，借用覆盖整段调用。
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
  // Safety: `s.l` 为存活 VM 状态，lua_gc/intern 契约同 intern() 内注释。
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

/// string.rep 契约：字节内容逐字节保留（含内嵌 NUL / 非法 UTF-8）、栈平衡；
/// 零长度巨型重复返回空串
#[test]
fn string_rep_preserves_bytes_and_stack() {
  let s = State::new();
  // Safety: `s.l` 存活；luaopen_string/pushlstring/tolstring 只要求非空 L 与
  // 可读字节区，`bytes` 切片借用覆盖各次调用。
  unsafe {
    luaopen_string(s.l);
    for bytes in [b"".as_slice(), b"a", b"t\0\xff", &[0xff; 1024]] {
      for n in [-1, 0, 1, 2, 3, 31, 32, 33] {
        lua_getfield(s.l, 1, c"rep".as_ptr());
        lua_pushlstring(s.l, bytes.as_ptr().cast(), bytes.len());
        lua_pushinteger(s.l, n);
        assert_eq!(lua_pcall(s.l, 2, 1, 0), 0);
        assert_eq!(lua_gettop(s.l), 2);
        // Safety: `lua_tolstring_ref` 只读栈顶结果串槽，返回全字节切片（含 `\0`/非法
        // UTF-8 字节保真）；不可转换时以空切片对齐旧 (null, 0) 形态。
        let got = lua_tolstring_ref(s.l, -1).unwrap_or(&[]);
        let expected = bytes.repeat(n.max(0) as usize);
        assert_eq!(got, expected);
        lua_settop(s.l, 1);
      }
    }
    lua_getfield(s.l, 1, c"rep".as_ptr());
    lua_pushlstring(s.l, c"".as_ptr(), 0);
    lua_pushinteger(s.l, 1_000_000_000);
    assert_eq!(lua_pcall(s.l, 2, 1, 0), 0);
    // 本处专测 C-ABI 垫片 `lua_tolstring` 的 size_t* 出参语义：空串结果写入 0，
    // 覆盖调用方预置值（Rust 侧长度请用 `lua_tolstring_ref` 的切片长度）。
    let mut len = 1;
    lua_tolstring(s.l, -1, &mut len);
    assert_eq!(len, 0);
  }
}

#[test]
fn intern_empty_string() {
  let s = State::new();
  // Safety: `s.l` 存活。零长度是 luaS_newlstr 的边界输入——cpp 允许 (null, 0) 探针，
  // 切片形态下同一零长度边界以 `&[]` 表达（与 `intern(b"")` 同为空切片）。
  unsafe {
    let a = s.intern(b"");
    assert!(!a.is_null());
    let b = lua_s_newlstr(s.l, &[]).cast::<c_void>();
    assert!(eq(a, b), "零长度 intern 必须命中同一对象");
  }
}

/// 字符串→number 的 API 级契约（tonumber 走 luaO_str2d，经 lua_tonumberx 暴露）
#[test]
fn tonumber_string_conversion_via_api() {
  let s = State::new();
  // Safety: `s.l` 存活；`bytes` 为 'static 常量数组，借用覆盖各次 pushlstring。
  unsafe {
    // 数组直接迭代，免 Vec 堆分配与 drain
    for (bytes, expect) in [
      (b"10" as &[u8], Some(10.0)),
      (b"0x10", Some(16.0)),
      (b" 3.5", Some(3.5)),
      (b"1e2", Some(100.0)),
      (b"abc", None),
      (b"12xyz", None),
    ] {
      lua_pushlstring(s.l, bytes.as_ptr() as *const c_char, bytes.len());
      let v = lua_tonumberx(s.l, -1);
      match expect {
        Some(e) => assert_eq!(v, Some(e), "{bytes:?} 应可转换"),
        None => assert_eq!(v, None, "{bytes:?} 应拒绝转换"),
      }
      lua_settop(s.l, 0);
    }
  }
}

/// int64 相等判定必须精确到 64 位：上游 `luaV_equalval`（lvmutils.cpp:349）与
/// `luaO_rawequalObj`（lobject.cpp）都走 `luai_inteq(lvalue(t1), lvalue(t2))`
/// 即 i64 == i64（lnumutils.h:18）。若中转 f64，2^53 之上的相邻整数会塌缩成误判相等。
#[test]
fn int64_equality_is_exact_beyond_f64_precision() {
  let s = State::new();

  let cases: [(i64, i64); 6] = [
    (1i64 << 53, (1i64 << 53) + 1),
    (i64::MAX, i64::MAX - 1),
    (i64::MIN, i64::MIN + 1),
    (-1, 1),
    (5, 5),
    (0, 0),
  ];

  for (a, b) in cases {
    let mut t1 = TValue::default();
    let mut t2 = TValue::default();
    let want = i32::from(a == b);

    // Safety: 宏写 TValue 的 union 载荷字段；`s.l` 存活，t1/t2 为本作用域值对象。
    unsafe {
      setlvalue!(&mut t1, a);
      setlvalue!(&mut t2, b);

      assert_eq!(
        lua_o_rawequal_obj(&t1, &t2),
        want,
        "rawequal({a}, {b}) 必须按 int64 精确比较"
      );
      assert_eq!(
        lua_v_equalval_export(s.l, &t1, &t2),
        want,
        "equalval({a}, {b}) 必须按 int64 精确比较"
      );
    }
  }
}

/// lightuserdata 相等契约：指针与 tag 都相等才算等
/// （cpp lvmutils.cpp:356 `pvalue(t1) == pvalue(t2) && lightuserdatatag(t1) == lightuserdatatag(t2)`）。
#[test]
fn lightuserdata_equality_requires_pointer_and_tag() {
  let s = State::new();

  let mut storage = [0u8; 2];
  let base = addr_of_mut!(storage).cast::<c_void>();
  // 只比较指针值，不解引用，故取同一数组第二个元素的地址即可区分两个 lightuserdata，
  // 无需指针算术。
  let other = addr_of_mut!(storage[1]).cast::<c_void>();

  let value = |p: *mut c_void, tag: i32| {
    let mut t = TValue::default();
    // `setpvalue!`/`set_pvalue` 已收口为安全实现：宏体对 `&mut t` 的解引用非 unsafe，
    // p 仅作透传载荷存储、不解引用。
    setpvalue!(&mut t, p, tag);
    t
  };

  let cases: [(*mut c_void, i32, *mut c_void, i32, i32); 3] = [
    (base, 1, base, 1, 1),
    (base, 1, base, 2, 0),
    (base, 1, other, 1, 0),
  ];

  for (p1, tag1, p2, tag2, want) in cases {
    let t1 = value(p1, tag1);
    let t2 = value(p2, tag2);

    // Safety: lua_o_rawequal_obj/lua_v_equalval_export 只要求存活 L 与两个
    // 本作用域 TValue；p1/p2 只作指针值比较，从不解引用。
    unsafe {
      assert_eq!(
        lua_o_rawequal_obj(&t1, &t2),
        want,
        "rawequal(tag {tag1}, {tag2})"
      );
      assert_eq!(
        lua_v_equalval_export(s.l, &t1, &t2),
        want,
        "equalval(tag {tag1}, {tag2})"
      );
    }
  }
}
