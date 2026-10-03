use core::{
  ffi::{c_double, c_float},
  mem::size_of,
};

use memchr::memchr;

use crate::{
  enums::k_option::KOption,
  functions::{
    copywithendian::copywithendian, getdetails::getdetails, getnum::FmtCursor,
    initheader::initheader, lua_l_checkstack::lua_l_checkstack, lua_l_optinteger::lua_l_optinteger,
    lua_pushlstring::lua_pushlstring_bytes, posrelat::posrelat, unpackint::unpackint,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{ftypes::Ftypes, header::Header, lua_state::LuaState},
};

/// unpack 读窗切片核（真实逻辑）：cpp `str_unpack`（lstrlib.cpp:1641）的取参序
/// （#1 格式串 → #2 数据串 → #3 初位缺省）、`data string too short`/
/// `initial position out of string`/`unfinished string for format 'z'` 抛出点、
/// 字节序解码与逐项压栈序逐点对齐 oracle。
///
/// 入约模型（r12-w6e，引用先例 `utf_8_decode.rs`/`byteoffset.rs`）：`fmt_bytes`/
/// `data` 均为 payload 切片（**不含**终止 NUL）——`arg_check` 剩余量钳位保证
/// Kint/Kuint/Kfloat/Kchar/Kstring 各读窗恒落 payload `[0, ld)` 内；唯一串尾外
/// 读点 Kzstr 的 `strlen` 以 `memchr` 脱靶归一（脱靶 ⟺ cpp 扫至串尾终止 NUL，
/// len 取尾距 `ld - p`），旧 `ld + 1` 扩窗退役，无静默扩窗。各切片借用自栈槽
/// 串体（Lua 串不可变且不被移动，本次调用内存活——同 `lua_l_checklstring_ref`
/// 切片契约）；结果逐项压栈经 `lua_l_checkstack` 保余量。
///
/// # Safety
/// `l` 须为可抛错受保护帧内存活的 `LuaState`：`initheader` 接线 `h.l` 与传入
/// `l` 的 `unpackint` 报错内核经裸指针解引用抛出并 unwind（不返回）。
unsafe fn str_unpack_ref(l: &mut LuaState, fmt_bytes: &[u8], data: &[u8]) -> i32 {
  unsafe {
    let mut h = Header::default();
    let mut fmt = FmtCursor::from_slice(fmt_bytes);

    let ld = data.len();
    let mut pos = posrelat(lua_l_optinteger(l, 3, 1), ld) - 1;
    if pos < 0 {
      pos = 0;
    }

    let mut n = 0;
    l.arg_check(pos as usize <= ld, 3, "initial position out of string");
    initheader(l, &mut h);

    while fmt.cur() != 0 {
      let (opt, size, ntoalign) = getdetails(&mut h, pos as usize, &mut fmt);
      l.arg_check(
        (ntoalign as usize).wrapping_add(size as usize) <= ld - pos as usize,
        2,
        "data string too short",
      );

      pos += ntoalign;
      // 钳位保证 p ∈ [0, ld]，各读取窗口 data[p..p+size] 恒落在 payload 界内
      let p = pos as usize;
      lua_l_checkstack(l, 2, "too many results");
      n += 1;

      match opt {
        KOption::Kint | KOption::Kuint => {
          // 有/无符号双臂仅差 unpackint 符号扩展位与回推路径（i64 直转 vs 位重解读 u64）
          let signed = opt == KOption::Kint;
          let res = unpackint(
            l,
            &data[p..p + size as usize],
            h.islittle,
            size,
            signed as i32,
          );
          l.push_number(if signed {
            res as f64
          } else {
            res as u64 as f64
          });
        }
        KOption::Kfloat => {
          let mut u = Ftypes { n: 0.0 };
          copywithendian(
            &mut u.buff[..size as usize],
            &data[p..p + size as usize],
            h.islittle,
          );
          let num = if size as usize == size_of::<c_float>() {
            u.f as f64
          } else if size as usize == size_of::<c_double>() {
            u.d
          } else {
            u.n
          };
          l.push_number(num);
        }
        KOption::Kchar => {
          lua_pushlstring_bytes(l, &data[p..p + size as usize]);
        }
        KOption::Kstring => {
          let len = unpackint(l, &data[p..p + size as usize], h.islittle, size, 0) as usize;
          l.arg_check(len <= ld - p - size as usize, 2, "data string too short");
          let q = p + size as usize;
          lua_pushlstring_bytes(l, &data[q..q + len]);
          pos += len as i32;
        }
        KOption::Kzstr => {
          // 从当前偏移单遍扫描首个 NUL 求 strlen（cpp lstrlib.cpp:1705
          // `strlen(data + pos)`）：payload 脱靶 ⟺ cpp 扫至串尾终止 NUL，len 取
          // 尾距 `ld - p`（越界 `get` 归一，入约见文件头），后续尾界判定两端
          // 同失败、同抛出
          let len = memchr(0, &data[p..]).unwrap_or(ld - p);
          l.arg_check(p + len < ld, 2, "unfinished string for format 'z'");
          lua_pushlstring_bytes(l, &data[p..p + len]);
          pos += len as i32 + 1;
        }
        KOption::Kpaddalign | KOption::Kpadding | KOption::Knop => {
          n -= 1;
        }
      }

      pos += size;
    }

    l.push_integer(pos + 1);
    n + 1
  }
}

/// 核心转发垫片（一行委托 [`str_unpack_ref`]）：本面消费点仅 `lua_lib_fn!`
/// 生成的注册臂 `str_unpack_arm`（luaopen_string.rs:8 导入、:34 STRLIB
/// "unpack" 注册），全仓实测无其余消费面 ⇒ 保一行形（r12-w5s `byteoffset`
/// 垫片先例）。#1/#2 串实参在入参位取 payload 切片：`check_bytes` 实参求值序
/// #1→#2 与旧形（先格式串游标、后数据串检出）一致，非串抛出点不变。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v38 收形后
/// 本面唯一点名转手、无裸操作，降为安全 `fn`）：`l` 须处于可抛错受保护帧，栈槽 #1/#2 为串
/// 实参（非串经 `check_bytes` 抛 "string expected"），其余义务单源 [`str_unpack_ref`]。
/// 两个 payload 窗口须与同句的核心调用（该核亦经 `&mut l` 压栈/报错）共存，p28 锚定形与
/// `&mut` 接收者不可共存 ⇒ 按 r16-v29 桥接判例在块内一次就地转手裸句柄，借用窗止于本块。
/// cpp lstrlib.cpp:1641 `str_unpack`。
pub(crate) fn str_unpack(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 由 `&mut` 保证有效且独占，转手后的 `lp` 即同一存活帧；payload 切片借用自
  // 栈槽 #1/#2 串体（不可变、不搬移），本次调用内有效
  unsafe {
    let lp = l.as_mut_ptr();
    str_unpack_ref(&mut *lp, (*lp).check_bytes(1), (*lp).check_bytes(2))
  }
}

lua_lib_fn!(pub(crate) fn str_unpack @ref, str_unpack_arm);
