//! r12-w5s T9 切片化：真实逻辑全部落在切片核心 [`codepoint_ref`]——参串经
//! `check_bytes` 取 payload 切片喂入（旧 `from_raw_parts(s, len + 1)` 手工裸窗退役：
//! 解码环只走到 `pose ≤ len`，串尾 NUL 仅被 `utf_8_decode` 的续字节探测读到，
//! payload 切片越界 `get` 归一为 0 与 cpp 读终止 NUL 逐点位同构，见
//! `utf_8_decode.rs` 入约模型）。C-ABI 面 [`codepoint`] 保一行委托垫片
//! （`codepoint_arm` 注册臂与 capi `ulua_codepoint` 导出壳均接本面）。

use crate::{
  functions::{
    lua_l_checkstack::lua_l_checkstack, lua_l_optinteger::lua_l_optinteger, u_posrelat::u_posrelat,
    utf_8_decode::utf_8_decode,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// UTF-8 码点扫描推栈（切片核心，真实逻辑）：cpp `utf8codes`（lutf8lib.cpp:109）的
/// 取参（先 #1 串、后 #2/#3 位置）、`out of range`/`string slice too long`/
/// `invalid UTF-8 code` 抛出序与空区间返回 0 逐点对齐 oracle。
///
/// `bytes` 为参串 payload 切片（借用自栈槽串体：Lua 串不可变、不搬移，本次调用
/// 内存活——同 `lua_l_checklstring_ref` 切片契约）；循环下标恒 `< pose ≤ len`，
/// `&bytes[i..]` 永在界内（空串时 `posi > pose` 先行返回 0）。
fn codepoint_ref(l: &mut LuaState, bytes: &[u8]) -> i32 {
  let len = bytes.len();

  let posi = u_posrelat(lua_l_optinteger(l, 2, 1), len);
  let pose = u_posrelat(lua_l_optinteger(l, 3, posi), len);

  l.arg_check(posi >= 1, 2, "out of range");
  l.arg_check(pose <= len as i32, 3, "out of range");

  if posi > pose {
    return 0; // empty interval; return no values
  }

  // 纯界判定留在 unsafe 外；仅抛错调用收进窄块（抛出文本与点位同 cpp :119）
  if (pose as i64 - posi as i64) >= i32::MAX as i64 {
    // SAFETY: 调用方垫片契约保证 `l` 为可捕获错误的存活帧，lua_l_error_l
    // 抛出不返回（longjmp 等价发散）
    unsafe { luaL_error!(l, "string slice too long") };
  }

  let n = (pose - posi) + 1;
  lua_l_checkstack(l, n, "string slice too long");

  let mut pushed = 0;
  // cpp `while (s < se)`：se = s + pose，指针比较等价于下标比较 i < pose；
  // 步长随 utf_8_decode 结果可变（1..=4 字节），非等差遍历，保留下标游走
  let mut i = (posi - 1) as usize;
  while i < pose as usize {
    let (step, code) = utf_8_decode(&bytes[i..]);
    // 解码失败经 luaL_error(!) 抛出不返回：let-else 收敛判定，消除哨兵回退值
    let Some(code) = code else {
      // SAFETY: 同上，`l` 存活帧内 "invalid UTF-8 code" 抛出发散
      unsafe { luaL_error!(l, "invalid UTF-8 code") };
    };
    l.push_integer(code as i32);
    pushed += 1;
    // cpp `s = next`：解码成功 step >= 1，循环必前进
    i += step;
  }

  pushed
}

/// C-ABI 镜像垫片（一行委托 [`codepoint_ref`]）：`lua_lib_fn!` 注册臂与 capi
/// 导出壳 `ulua_codepoint` 均接本面，故保留（契约单源在本签名）。
///
/// # Safety
/// `l` 须为存活 `LuaState` 且处于可抛错受保护帧，栈槽 #1 为串实参、#2/#3 按库函数
/// 约定取（取参序、抛出与压栈义务见 [`codepoint_ref`] 文档）；`&mut *l` 的引用
/// 重建窗口即本次调用。cpp `lutf8lib.cpp:109`。
pub unsafe fn codepoint(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 存活且独占驱动；payload 切片借用自栈槽 #1 串体
  // （不可变、不搬移），本次调用内有效
  unsafe { codepoint_ref(&mut *l, (*l).check_bytes(1)) }
}

lua_lib_fn!(pub fn codepoint, codepoint_arm);
