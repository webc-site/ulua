//! r12-w5s T9 切片化：真实逻辑全部落在切片核心 [`byteoffset_ref`]——参串经
//! `check_bytes` 取 payload 切片喂入（旧 `from_raw_parts(s, len + 1)` 手工裸窗退役：
//! cpp 的续字节探测在串尾 `posi == len` 点位读终止 NUL（恒非续字节），经
//! [`is_cont_at`] 越界归一为 0 逐点同构，入约模型见 `utf_8_decode.rs`）。
//! C-ABI 面 [`byteoffset`] 保一行委托垫片（`byteoffset_arm` 注册臂与 capi
//! `ulua_byteoffset` 导出壳均接本面）。cpp `lutf8lib.cpp:191`。

use crate::{
  functions::{
    lua_l_optinteger::lua_l_optinteger, u_posrelat::u_posrelat, utf_8_decode::is_cont_byte,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// 越界归一的续字节判定（切片核心唯一读点）：cpp `iscont(s + posi)` 在串尾
/// `posi == len` 读到恒终止 NUL（必判非续字节）；payload 切片同点位越界经 `get`
/// 归一为 0，判定结果逐点同构。`posi` 恒 ≥ 0——入口 `1 <= posi && posi <= len + 1`
/// 钳位后 `posi ∈ [0, len]`，各游走环均以 `posi > 0` / `posi < len` 卫兵先行，
/// 故 `as usize` 不回绕。
#[inline]
fn is_cont_at(bytes: &[u8], posi: i32) -> bool {
  bytes.get(posi as usize).copied().is_some_and(is_cont_byte)
}

/// 字符偏移回算（切片核心，真实逻辑）：cpp `byteoffset`（lutf8lib.cpp:191）的
/// 取参序（#1 串 → #2 n → #3 posi 默认位）、`position out of range`/
/// `initial position is a continuation byte` 抛出与双向游走逐点对齐 oracle，
/// 命中推 `posi + 1`、脱靶推 nil 并返回 1。
///
/// `bytes` 为参串 payload 切片（借用自栈槽串体，本次调用内存活——同
/// `lua_l_checklstring_ref` 切片契约）；全部下标读经 [`is_cont_at`] 界归一。
fn byteoffset_ref(l: &mut LuaState, bytes: &[u8]) -> i32 {
  let len = bytes.len();
  let mut n = l.check_integer(2);
  let mut posi = if n >= 0 { 1 } else { len as i32 + 1 };
  posi = u_posrelat(lua_l_optinteger(l, 3, posi), len);
  l.arg_check(
    1 <= posi && posi <= len as i32 + 1,
    3,
    "position out of range",
  );
  posi -= 1;

  if n == 0 {
    // find beginning of current byte sequence
    while posi > 0 && is_cont_at(bytes, posi) {
      posi -= 1;
    }
  } else {
    if is_cont_at(bytes, posi) {
      luaL_error!(l, "initial position is a continuation byte");
    }
    if n < 0 {
      while n < 0 && posi > 0 {
        // find beginning of previous character：先退一格，续字节连退至序列首
        //（原 `loop { posi -= 1; if !(…) break; }` 的 do-while 形态展开）
        posi -= 1;
        while posi > 0 && is_cont_at(bytes, posi) {
          posi -= 1;
        }
        n += 1;
      }
    } else {
      n -= 1; // do not move for 1st character
      while n > 0 && posi < len as i32 {
        // find beginning of next character：先进一格，续字节连进至下一序列首
        //（cpp `(cannot pass final '\0')`——串尾 NUL 经 `is_cont_at` 归一恒止行）
        posi += 1;
        while is_cont_at(bytes, posi) {
          posi += 1;
        }
        n -= 1;
      }
    }
  }

  if n == 0 {
    // did it find given character?
    l.push_integer(posi + 1);
  } else {
    // no such character
    l.push_nil();
  }
  1
}

/// C-ABI 镜像垫片（一行委托 [`byteoffset_ref`]）：`lua_lib_fn!` 注册臂与 capi
/// 导出壳 `ulua_byteoffset` 均接本面，故保留（契约单源在本签名）。
///
/// # Safety
/// `l` 须为存活 `LuaState` 且处于可抛错受保护帧：栈槽 #1 为串实参（非串经
/// `check_bytes` 抛 "string expected"），#2/#3 为整数或缺省，越界与续字节初位经
/// `arg_check`/`luaL_error` 抛出（义务见 [`byteoffset_ref`] 文档）；`&mut *l`
/// 的引用重建窗口即本次调用。cpp `lutf8lib.cpp:191`。
pub unsafe fn byteoffset(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 存活且独占驱动；payload 切片借用自栈槽 #1 串体
  // （不可变、不搬移），本次调用内有效
  unsafe { byteoffset_ref(&mut *l, (*l).check_bytes(1)) }
}

lua_lib_fn!(pub fn byteoffset, byteoffset_arm);
