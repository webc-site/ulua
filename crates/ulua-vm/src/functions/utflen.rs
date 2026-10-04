use crate::{
  functions::{
    lua_l_optinteger::lua_l_optinteger, u_posrelat::u_posrelat, utf_8_decode::utf_8_decode,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 取参/校验/解码/压栈全经 `check_bytes`/`arg_check`/`push_*` 安全门面，旧 `from_raw_parts`
/// 手工裸窗退役，体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错的受保护帧——栈槽 #1
/// 为串实参（非串经 `check_bytes` 抛错发散，严格先于 #2/#3 取数，兼作 cpp
/// `luaL_checklstring` 的先位抛错件），#2/#3 为可选整数；结果压栈需 `top` 后 ≥1 空槽。
///
/// 扫描窗取 payload 切片（`codepoint.rs` 同形判例）：解码环只走到 `posi ≤ posj ≤ len - 1`，
/// 串尾 NUL 终止符在旧形里仅被 `utf_8_decode` 的续字节探测读到，payload 切片越界 `get`
/// 归一为非法续字节与读终止 NUL 逐点位同构（见 `utf_8_decode.rs` 入约模型）。
/// cpp lutf8lib.cpp `utf8len`。
pub fn utflen(l: &mut LuaState) -> i32 {
  let mut n: i32 = 0;
  // 首次派窗止于取长（借用窗随语句结束），其后 `l` 复原可借
  let len = l.check_bytes(1).len();

  let posi = u_posrelat(lua_l_optinteger(l, 2, 1), len);
  let mut posj = u_posrelat(lua_l_optinteger(l, 3, -1), len);

  l.arg_check(
    1 <= posi && posi <= len as i32 + 1,
    2,
    "initial position out of string",
  );
  posj -= 1;
  l.arg_check(posj < len as i32, 3, "final position out of string");

  let mut posi = posi - 1;

  // 二次派窗取同一栈槽串体（同槽同值，观测等价）：钳位保证 0 <= posi <= posj <= len - 1，
  // `&bytes[posi..]` 恒起始界内
  let bytes = l.check_bytes(1);
  let mut invalid_at: Option<i32> = None;
  while posi <= posj {
    // 保留下标游走：步长随 utf_8_decode 结果可变（1..=4 字节），非等差遍历
    let (step, code) = utf_8_decode(&bytes[posi as usize..]);
    if code.is_none() {
      invalid_at = Some(posi + 1);
      break;
    };
    // cpp `posi = s1 - s`：解码成功时 step >= 1，循环必前进
    posi += step as i32;
    n += 1;
  }
  // 扫描窗随块尾 `bytes` 末次使用收口；失败臂的 nil/posi 连压与 cpp「解码失败即推」之间
  // 无其他栈操作穿插，观测序列不变

  if let Some(pos) = invalid_at {
    l.push_nil();
    l.push_integer(pos);
    return 2;
  }

  l.push_integer(n);
  1
}

lua_lib_fn!(pub fn utflen @ref, utflen_arm);
