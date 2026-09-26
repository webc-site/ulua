//! 串表 intern 链行为钉：对照 cpp/VM/src/lstring.cpp 的
//! `lua_s_newlstr`（扫描/复活）、`newlstr` 发布尾（头插 + 到限翻倍 resize）、
//! `lua_s_buffinish`（缓冲发布与去重）、`lua_s_resize`（全量 rehash 保序）。
//! 句柄化重构（`BucketIdx` 句柄 + 切片视图 + 方法收链）后，本文件锁住与
//! oracle 一致的可见行为。
//!
//! 观测面：`lua_tolstring` 返回驻留 TString 的 data 指针（cpp `getstr(ts)`），
//! 内容相同的两个不同 TString 对象地址必不同，「数据指针相等」即「同一
//! intern 对象」的可靠探针；GC 后的地址回收复用不参与判定（只测幂等与内容）。

use core::{ffi::c_char, slice::from_raw_parts};
use std::ptr::eq;

use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{
    lua_concat::lua_concat, lua_gc::lua_gc, lua_pushlstring::lua_pushlstring,
    lua_s_newlstr::lua_s_newlstr, lua_settop::lua_settop, lua_tolstring::lua_tolstring,
  },
  records::{lua_state::lua_State, t_string::tstring},
};

#[path = "common/state.rs"]
mod state;

use state::State;

/// intern 并把栈顶串的 (data 指针, 长度) 快照下来，随后弹栈。
fn intern_and_snapshot(l: *mut lua_State, bytes: &[u8]) -> (*const c_char, usize) {
  // Safety: `l` 为测试守卫独占的存活 VM；bytes 借用覆盖调用；测试中段无 GC，
  // 返回的 data 指针在快照后保持有效。
  unsafe {
    lua_pushlstring(l, bytes.as_ptr() as *const c_char, bytes.len());
    let mut sz: usize = 0;
    let p = lua_tolstring(l, -1, &mut sz);
    lua_settop(l, -2);
    assert!(!p.is_null(), "栈顶应为字符串");
    (p, sz)
  }
}

fn key(i: usize) -> String {
  format!("strtable-intern-key-{i:05}")
}

/// 越过初始表（LUA_MINSTRTABSIZE=32）连插数千串：多次翻倍 resize 之后，
/// 每个串对 `lua_s_newlstr` 仍幂等复用同一 TString，且互不串桶（去重精确）。
#[test]
fn dedup_survives_multiple_growth_resizes() {
  const COUNT: usize = 3000;
  let s = State::new();
  // Safety: `s.l` 存活；各 key 字节常量借用覆盖整段循环。
  let pointers: Vec<*mut tstring> = unsafe {
    (0..COUNT)
      .map(|i| {
        let k = key(i);
        let ts = lua_s_newlstr(s.l, k.as_bytes());
        assert!(!ts.is_null());
        assert!(
          eq(ts, lua_s_newlstr(s.l, k.as_bytes())),
          "第 {i} 串二次 intern 未复用"
        );
        ts
      })
      .collect()
  };
  // 全量 rehash 后回头复查最早插入的串：桶迁移未丢失也未误并
  unsafe {
    for (i, &ts) in pointers.iter().enumerate() {
      let k = key(i);
      assert!(
        eq(ts, lua_s_newlstr(s.l, k.as_bytes())),
        "resize 后键 {i} 指向漂移"
      );
    }
  }
  // 两两不同指针（内容互异，链法去重不得误判前缀相同的键）
  for w in pointers.windows(2) {
    assert!(!eq(w[0], w[1]));
  }
}

/// `lua_s_buffinish` 发布路径（经 `lua_concat` 缓冲拼接）：拼接结果与既有
/// 驻留串等字节时必须复用旧对象——栈面数据指针与整串 intern 的指针相等。
#[test]
fn concat_buffinish_reuses_interned_string() {
  let s = State::new();
  let (whole_p, whole_n) = intern_and_snapshot(s.l, b"hello luau concat buffer");
  let (part_p, _) = intern_and_snapshot(s.l, b"hello luau ");
  // Safety: `s.l` 存活；push 两段各经 intern，concat 走 bufstart/buffinish；
  // 快照指针在无 GC 的测试中段保持有效。
  unsafe {
    lua_pushlstring(s.l, part_p, 11); // 复用已驻留前缀（pushlstring 亦 intern）
    let tail = b"concat buffer";
    lua_pushlstring(s.l, tail.as_ptr() as *const c_char, tail.len());
    assert_eq!(whole_n, 11 + tail.len(), "用例前提：两段拼成整串");
    lua_concat(s.l, 2);
    let mut sz: usize = 0;
    let got = lua_tolstring(s.l, -1, &mut sz);
    assert_eq!(sz, whole_n);
    assert!(
      eq(got, whole_p),
      "buffinish 命中时必须复用驻留串（cpp lstring.cpp:113）"
    );
    lua_settop(s.l, 0);
  }
}

/// 无人引用的垃圾串经 fullgc 被清扫（桶链摘除、nuse 递减）；再 intern 得到
/// 内容一致的对象，且对新对象的重复 intern 依旧幂等——证明 sweep 后表仍自洽。
#[test]
fn gc_sweep_then_reintern_keeps_table_consistent() {
  let s = State::new();
  let transient = b"gc-transient-string-42";
  unsafe {
    let ts1 = lua_s_newlstr(s.l, transient);
    assert_eq!(lua_gc(s.l, LuaGcOp::Collect as i32, 0), 0, "fullgc 失败");
    // 清扫后重 intern：无残留半摘链（若有，桶内指向已释放对象会当场失守）
    let ts2 = lua_s_newlstr(s.l, transient);
    assert!(
      eq(ts2, lua_s_newlstr(s.l, transient)),
      "清扫后重 intern 不幂等"
    );
    let _ = ts1;
  }
  // 内容回读一致（长度由切片携带，内嵌终止 NUL 不参与）
  let (p, n) = intern_and_snapshot(s.l, transient); // 辅助内已自弹栈
  assert_eq!(n, transient.len());
  // Safety: p 为刚快照的 NUL 前 data 区，长度 n 字节可读。
  let bytes = unsafe { from_raw_parts(p as *const u8, n) };
  assert_eq!(bytes, &transient[..]);
}
