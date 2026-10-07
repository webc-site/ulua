use core::ptr::null_mut;

use crate::records::t_string::tstring;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LocVar {
  /// 局部变量名（cpp `lobject.h:420` `TString* varname`）。null = 「该局部无名」，是真正的
  /// 缺席语义而非指针算式基址：cpp `lgc.cpp:413 if (f->locvars[i].varname)` 明确判空后才
  /// `stringmark`，且它由 `readString`（id 0 即 NULL）赋值。§2 判定=规则 1「缺席」，理想形态
  /// `Option<NonNull<tstring>>`（复用 null niche，零尺寸开销）。但本次改动被限定在本定义文件：
  /// 其判空/解引用散落在 `traverseproto.rs:45`、`validateproto.rs:45`、`dumpthread.rs:136`
  /// （均 `.is_null()` 控制流）与 `lua_getlocal.rs:68`/`lua_setlocal.rs:47`（r16-v24 收形后行指；`getstr((*var).varname)`
  /// 解引用），写点在 `loadsafe.rs:800`（`locvar.varname = varname` 收 `read_str!` 的裸指针），全部
  /// 属并行会话文件，就地改型破坏其编译。故本轮保留 `*mut tstring`，null 只在 `Default` 占位，
  /// 并记录这一待协调改点（构造端 `NonNull::new`、消费端 `is_none()`/`.map`）而非机械保留。
  pub varname: *mut tstring,
  pub startpc: i32,
  pub endpc: i32,
  pub reg: u8,
}

impl Default for LocVar {
  fn default() -> Self {
    Self {
      varname: null_mut(),
      startpc: 0,
      endpc: 0,
      reg: 0,
    }
  }
}
