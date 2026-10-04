use crate::{
  functions::{
    buffutfchar::buffutfchar, lua_l_addlstring::lua_l_addlstring, lua_l_buffinit::lua_l_buffinit,
    lua_l_pushresult::lua_l_pushresult, lua_pushlstring::lua_pushlstring_bytes,
  },
  macros::{lua_lib_fn::lua_lib_fn, utf_8_buffsz::UTF8BUFFSZ},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 取顶/校验/编码全经安全门面与已收形的 `buffutfchar`，仅三次 C 缓冲/压串被调仍为不安全调用
/// 而落窄块，故本体降为安全 `fn`）：`l` 须为正在执行的 utf8 库 C 函数帧——栈槽 #1..top 为整数码点
/// （`buffutfchar` 逐个校验、越界抛错发散），串构造经 `lua_l_buffinit`/`lua_l_pushresult` 的栈与 GC
/// 协议，返回值即结果串在栈上的槽数。cpp lutf8lib.cpp:162 `utfchar`。
pub fn utfchar(l: &mut LuaState) -> i32 {
  let mut buff = [0u8; UTF8BUFFSZ];

  let n = l.get_top(); // number of arguments
  if n == 1 {
    // optimize common case of single char
    let charstr = buffutfchar(l, 1, &mut buff);
    // lua_pushlstring_bytes 已降为安全切片核心（r12-w6d）：l 由 &mut 承载存活/独占，
    // charstr 为 buff 自有的界内编码窗，核心界内拷入堆串、不留借出窗
    lua_pushlstring_bytes(l, charstr);
  } else {
    let mut b = LuaLStrbuf::new();
    lua_l_buffinit(l, &mut b);
    for i in 1..=n {
      let charstr = buffutfchar(l, i, &mut buff);
      // SAFETY: b 处于 buffinit 之后、pushresult 之前的有效态；charstr 为 buff 自有界内
      // 切片，不与 b 缓冲重叠（同 cpp memcpy 前置条件）
      unsafe { lua_l_addlstring(&mut b, charstr) };
    }
    // w6e 降级消费点：`lua_l_pushresult` 已降为安全 fn，包裹消亡；b 为 buffinit 接线
    // 存活态、结果槽由 C 帧约定预留；可分配、可 GC
    lua_l_pushresult(&mut b);
  }
  1
}

lua_lib_fn!(pub fn utfchar @ref, utfchar_arm);
