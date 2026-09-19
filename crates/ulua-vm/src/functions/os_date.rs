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

use core::{
  ffi::{CStr, c_char, c_int},
  mem::zeroed,
  ptr::null_mut,
};

use crate::{
  functions::{
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
    lua_isnoneornil::lua_isnoneornil, lua_l_addchar::luaL_addchar, lua_l_argerror::luaL_argerror,
    lua_l_optstring::luaL_optstring, lua_strftimeoptions::LUA_STRFTIMEOPTIONS,
  },
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

unsafe extern "C" {
  fn time(t: *mut TimeT) -> TimeT;
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `gmtime_r` wrapper. The graph's `gmtime_r` dep unconditionally calls Windows
/// `gmtime_s`, so we declare the platform-correct symbol directly here.
unsafe fn os_gmtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm {
  unsafe {
    #[cfg(target_os = "windows")]
    {
      unsafe extern "C" {
        // `gmtime_s` is inline in MSVC's <time.h>; link the real UCRT export
        // `_gmtime64_s` (__time64_t = i64) instead.
        fn _gmtime64_s(result: *mut Tm, timep: *const TimeT) -> c_int;
      }
      if _gmtime64_s(result, timep) == 0 {
        result
      } else {
        null_mut()
      }
    }
    #[cfg(not(target_os = "windows"))]
    {
      unsafe extern "C" {
        fn gmtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm;
      }
      gmtime_r(timep, result)
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn os_date(l: *mut lua_State) -> c_int {
  unsafe {
    let mut s: *const c_char = luaL_optstring!(l, 1, c"%c".as_ptr() as *const c_char);
    let t: TimeT = if lua_isnoneornil!(l, 2) {
      time(null_mut())
    } else {
      lua_l_checknumber(l, 2) as TimeT
    };

    let mut tmv: Tm = zeroed();
    let stm: *mut Tm;
    if *s == b'!' as c_char {
      // UTC?
      stm = os_gmtime_r(&t, &mut tmv);
      s = s.add(1); // skip '!'
    } else {
      // localtime fails for dates before the epoch on some platforms, so disallow that
      stm = if t < 0 {
        null_mut()
      } else {
        localtime_r(&t, &mut tmv)
      };
    }

    if stm.is_null() {
      // invalid date?
      lua_pushnil(l);
    } else if CStr::from_ptr(s) == c"*t" {
      lua_createtable(l, 0, 9); // 9 = number of fields
      setfield(l, c"sec", (*stm).tm_sec);
      setfield(l, c"min", (*stm).tm_min);
      setfield(l, c"hour", (*stm).tm_hour);
      setfield(l, c"day", (*stm).tm_mday);
      setfield(l, c"month", (*stm).tm_mon + 1);
      setfield(l, c"year", (*stm).tm_year + 1900);
      setfield(l, c"wday", (*stm).tm_wday + 1);
      setfield(l, c"yday", (*stm).tm_yday + 1);
      setboolfield(l, c"isdst", (*stm).tm_isdst);
    } else {
      let mut b: LuaLStrbuf = LuaLStrbuf {
        p: null_mut(),
        end: null_mut(),
        l: null_mut(),
        storage: null_mut(),
        buffer: [0; LUA_BUFFERSIZE],
      };
      lua_l_buffinit(l, &mut b);

      // 零拷贝迭代剩余格式串；peek 前瞻实现 C++ 的 *(s + 1) 判定
      let mut fmt = CStr::from_ptr(s).to_bytes().iter().copied().peekable();
      while let Some(c) = fmt.next() {
        match (c, fmt.peek().copied()) {
          // 转换指示符：'%' 后跟合法字符（非末尾）
          (b'%', Some(next)) => {
            if !LUA_STRFTIMEOPTIONS.as_bytes().contains(&next) {
              luaL_argerror!(l, 1, "invalid conversion specifier");
            }
            let rendered = strftime_directive(&*stm, next);
            lua_l_addlstring(&mut b, rendered.as_ptr() as *const c_char, rendered.len());
            fmt.next(); // 消费指示符字节
          }
          // 无转换指示符（非 '%' 或 '%' 位于末尾）：原样输出
          _ => luaL_addchar!(&mut b, c),
        }
      }
      lua_l_pushresult(&mut b);
    }
    1
  }
}
