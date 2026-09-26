//! Source: `VM/src/loslib.cpp:112`
//!
//! `os.date` — format a timestamp. An optional `!` prefix selects UTC; the format
//! `*t` builds a table of broken-down fields; otherwise each `%` conversion spec
//! is rendered through the pure-Rust directive renderer (the C++ original
//! forwards to `strftime`, which `wasm32-unknown-unknown` cannot bind — no libc
//! — so the rendering is implemented natively for every target; see
//! `strftime_directive` for the C-locale / timezone policy). The broken-down
//! time still comes from `time`/`gmtime_r`/`localtime_r`, which the wasm build
//! shims in `ulua-common::wasm_libc` (fixed clock, local == UTC).

use core::{ffi::c_char, ptr::null_mut};

use crate::{
  functions::{
    cstr_bytes,
    localtime_r::{TimeT, Tm, localtime_r},
    lua_createtable::lua_createtable,
    lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit,
    lua_l_checknumber::lua_l_checknumber,
    lua_l_pushresult::lua_l_pushresult,
    lua_pushnil::lua_pushnil,
    setboolfield::setboolfield,
    setfield::setfield,
    strftime_directive::strftime_directive,
  },
  macros::{
    lua_isnoneornil::lua_isnoneornil, lua_l_addchar::lua_l_addchar, lua_l_argerror::luaL_argerror,
    lua_l_optstring::luaL_optstring, lua_strftimeoptions::LUA_STRFTIMEOPTIONS,
  },
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 `CStr`/`c"…"`）。
const FMT_DEFAULT: &[u8] = b"%c\0";

unsafe extern "C" {
  fn time(t: *mut TimeT) -> TimeT;
}

/// `gmtime_r` wrapper：把 `timep` 按 UTC 分解写入 `result`；平台失败返回 `None`。
///
/// 图依赖的 `gmtime_r` 会无条件调用 Windows 的 `gmtime_s`，故在此直接声明
/// 平台正确的符号，把 unsafe 关进最小 FFI 边界。
fn os_gmtime_r<'a>(timep: &TimeT, result: &'a mut Tm) -> Option<&'a mut Tm> {
  #[cfg(target_os = "windows")]
  let ok = {
    unsafe extern "C" {
      // `gmtime_s` is inline in MSVC's <time.h>; link the real UCRT export
      // `_gmtime64_s` (__time64_t = i64) instead.
      fn _gmtime64_s(result: *mut Tm, timep: *const TimeT) -> i32;
    }
    // Safety: `timep`/`result` 为存活可读/可写 Rust 引用，按 UCRT C 签名传其地址
    unsafe { _gmtime64_s(result, timep) == 0 }
  };
  #[cfg(not(target_os = "windows"))]
  let ok = {
    unsafe extern "C" {
      fn gmtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm;
    }
    // Safety: `timep`/`result` 为存活可读/可写 Rust 引用，按 glibc C 签名传其地址；
    // 成功时原地写回并返回 `result`，失败返回 null
    unsafe { !gmtime_r(timep, result).is_null() }
  };

  if ok { Some(result) } else { None }
}

/// # Safety
/// `l` 须为存活 LuaState 并处于 os.date 的受保护帧：栈 1 号位为可选格式串（`luaL_optstring!` 返回本帧存活的 NUL 结尾指针，
/// 经 `cstr_bytes` 折成 `&[u8]`、首字节判 UTC 前缀），2 号位可选数字时间（`lua_isnoneornil`/`lua_l_checknumber`）；
/// `time`/`localtime_r`/`os_gmtime_r` 为 libc FFI（写栈上 `tmv`，失败返回 None → pushnil），
/// `lua_createtable`/`lua_l_buffinit`/`lua_l_pushresult` 可分配/GC/抛错。cpp/VM/src/loslib.cpp:112 os_date。
pub(crate) unsafe extern "C-unwind" fn os_date(l: *mut LuaState) -> i32 {
  unsafe {
    let s: *const c_char = luaL_optstring!(l, 1, FMT_DEFAULT.as_ptr().cast());
    let t: TimeT = if lua_isnoneornil!(l, 2) {
      // Safety: C `time` 接受 NULL 出参（仅取返回值），不写任何内存
      time(null_mut())
    } else {
      lua_l_checknumber(l, 2) as TimeT
    };

    // Safety: 契约保证 luaL_optstring 返回本帧存活的 NUL 结尾串，cstr_bytes 扫至 NUL 终止
    let mut fmt: &[u8] = cstr_bytes(s);
    let mut tmv: Tm = Tm::default();
    let stm: Option<&mut Tm>;
    if fmt.first() == Some(&b'!') {
      // UTC?
      fmt = &fmt[1..]; // skip '!'
      stm = os_gmtime_r(&t, &mut tmv);
    } else {
      // localtime fails for dates before the epoch on some platforms, so disallow that
      stm = if t < 0 {
        None
      } else {
        localtime_r(&t, &mut tmv)
      };
    }

    match stm {
      // invalid date?
      None => lua_pushnil(l),
      Some(stm) if fmt == b"*t" => {
        lua_createtable(l, 0, 9); // 9 = number of fields
        setfield(l, b"sec", stm.tm_sec);
        setfield(l, b"min", stm.tm_min);
        setfield(l, b"hour", stm.tm_hour);
        setfield(l, b"day", stm.tm_mday);
        setfield(l, b"month", stm.tm_mon + 1);
        setfield(l, b"year", stm.tm_year + 1900);
        setfield(l, b"wday", stm.tm_wday + 1);
        setfield(l, b"yday", stm.tm_yday + 1);
        setboolfield(l, b"isdst", stm.tm_isdst);
      }
      Some(stm) => {
        let mut b = LuaLStrbuf::new();
        lua_l_buffinit(l, &mut b);

        // 零拷贝迭代剩余格式串；peek 前瞻实现 C++ 的 *(s + 1) 判定
        let mut fmt = fmt.iter().copied().peekable();
        while let Some(c) = fmt.next() {
          match (c, fmt.peek().copied()) {
            // 转换指示符：'%' 后跟合法字符（非末尾），集合即 LUA_STRFTIMEOPTIONS。
            (b'%', Some(next)) => {
              if !LUA_STRFTIMEOPTIONS.as_bytes().contains(&next) {
                luaL_argerror!(l, 1, "invalid conversion specifier");
              }
              let rendered = strftime_directive(stm, next);
              lua_l_addlstring(&mut b, rendered.as_bytes());
              fmt.next(); // 消费指示符字节
            }
            // 无转换指示符（非 '%' 或 '%' 位于末尾）：原样输出
            _ => lua_l_addchar!(&mut b, c),
          }
        }
        lua_l_pushresult(&mut b);
      }
    }
    1
  }
}
