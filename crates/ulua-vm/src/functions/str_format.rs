//! Source: `VM/src/lstrlib.cpp:966`
//!
//! `string.format` — walk the format string, copying literals and dispatching
//! each `%` spec to the matching argument: `%c/d/i/o/u/x/X/e/E/f/g/G` (and
//! padded `%s`) go through the pure-Rust directive formatter (the C++ original
//! forwards to `snprintf`, which `wasm32-unknown-unknown` cannot bind — no
//! libc — so the numeric path is implemented natively for every target), `%q`
//! quotes, `%s` fast-paths long strings, `%*` appends any value, and `%%` is a
//! literal percent.
//!
//! cpp 版经 `char form[MAX_FORMAT]` 缓冲 + `scanformat` 指针游标中转；Rust 版
//! 直接在格式串切片上按下标推进，扫描复用 `scanformat::scan_format_spec`。

use core::{ffi::c_char, slice::from_raw_parts};

use memchr::memchr;

use crate::{
  functions::{
    addquoted::addquoted,
    format_directive::{
      format_bytes, format_char, format_float, format_int, format_uint, parse_format_spec,
    },
    lua_gettop::lua_gettop,
    lua_l_addchar::lua_l_addchar,
    lua_l_addlstring::lua_l_addlstring,
    lua_l_addvalueany::lua_l_addvalueany,
    lua_l_buffinit::lua_l_buffinit,
    lua_l_checkinteger_64::lua_l_checkinteger_64,
    lua_l_checklstring::lua_l_checklstring,
    lua_l_checknumber::lua_l_checknumber,
    lua_l_pushresult::lua_l_pushresult,
    scanformat::{MAX_FORMAT_SPEC_SCAN, scan_format_spec},
  },
  macros::{l_esc::L_ESC, lua_isinteger_64::lua_isinteger_64, lua_l_error::luaL_error},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// cpp lstrlib.cpp:1052：无精度限定的长字符串（>=100 字节）直接拼接，
/// 跳过宽度/精度格式化路径。
const DIRECT_APPEND_MIN_LEN: usize = 100;

/// # Safety
/// 格式串由 `lua_l_checklstring` 取得：指向 GC 堆上存活字符串的 `sfl` 字节；
/// Luau 字符串不会被移动或压缩，循环期间栈增长/参数读取不使其悬垂。
pub(crate) unsafe extern "C-unwind" fn str_format(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 l 存活——checklstring 取 GC 堆字符串字节、buffinit/pushresult 在其栈上操作
  unsafe {
    let top = lua_gettop(l);
    let mut arg: i32 = 1;
    let mut sfl: usize = 0;
    let strfrmt = lua_l_checklstring(l, arg, &mut sfl);
    let f = from_raw_parts(strfrmt as *const u8, sfl);

    let mut b = LuaLStrbuf::new();
    lua_l_buffinit(l, &mut b);

    let mut i: usize = 0;
    while i < f.len() {
      let ch = f[i];
      if ch != L_ESC as u8 {
        lua_l_addchar(&mut b, ch as c_char);
        i += 1;
        continue;
      }
      i += 1; // *++strfrmt
      // cpp 在串尾读到 Lua 字符串恒有的 NUL 终止符，此处等价取 0
      let next = if i < f.len() { f[i] } else { 0 };
      if next == L_ESC as u8 {
        lua_l_addchar(&mut b, next as c_char);
        i += 1; // %%
      } else if next == b'*' {
        i += 1;
        arg += 1;
        if arg > top {
          luaL_error!(l, "missing argument #{}", arg);
        }
        lua_l_addvalueany(&mut b, arg);
      } else {
        // format item：扫描 flags/width/prec，得到 `&window[..p]` 即 cpp `form` 内容
        arg += 1;
        if arg > top {
          luaL_error!(l, "missing argument #{}", arg);
        }
        let hi = (i + MAX_FORMAT_SPEC_SCAN).min(f.len());
        let window = &f[i..hi];
        // cpp 的扫描止于首个 NUL（嵌入 '\0' 之后不参与解析）
        let window = &window[..memchr(0, window).unwrap_or(window.len())];
        let p = scan_format_spec(window).unwrap_or_else(|err| luaL_error!(l, "{}", err));
        let indicator = if i + p < f.len() { f[i + p] } else { 0 };
        let spec = parse_format_spec(&window[..p]);
        i += p + 1; // 消费 spec 字节与转换指示符（指示符为 0 时下方必报错）
        match indicator {
          b'c' => {
            // DELIBERATE DEVIATION：越出 i32 域的双精度实参，cpp 的
            // `(int)checknumber` 在 x86 上是 cvttsd2si UB（→INT_MIN→'\0'），
            // Rust 侧取饱和语义（→u8 截断）——定义化 C UB 角落
            let out = format_char(&spec, lua_l_checknumber(l, arg) as i32 as u8);
            lua_l_addlstring(&mut b, &out);
          }
          b'd' | b'i' => {
            let value: i64 = if lua_isinteger_64!(l, arg) {
              lua_l_checkinteger_64(l, arg)
            } else {
              // 越出 i64 域：cpp double→int64 转换为 UB，Rust 取饱和（同 %c 案）
              lua_l_checknumber(l, arg) as i64
            };
            let out = format_int(&spec, value);
            lua_l_addlstring(&mut b, &out);
          }
          b'o' | b'u' | b'x' | b'X' => {
            let v: u64 = if lua_isinteger_64!(l, arg) {
              lua_l_checkinteger_64(l, arg) as u64
            } else {
              let arg_value = lua_l_checknumber(l, arg);
              if arg_value < 0.0 {
                // 越域饱和化：同 %d/%i 案（cpp 侧 UB 角落）
                (arg_value as i64) as u64
              } else {
                arg_value as u64
              }
            };
            let out = format_uint(&spec, indicator, v);
            lua_l_addlstring(&mut b, &out);
          }
          b'e' | b'E' | b'f' | b'g' | b'G' => {
            let out = format_float(&spec, indicator, lua_l_checknumber(l, arg));
            lua_l_addlstring(&mut b, &out);
          }
          b'q' => {
            addquoted(l, &mut b, arg);
          }
          b's' => {
            let mut slen: usize = 0;
            let s = lua_l_checklstring(l, arg, &mut slen);
            // no precision and string too long to format, or no format necessary
            if p == 0 || (spec.precision.is_none() && slen >= DIRECT_APPEND_MIN_LEN) {
              lua_l_addlstring(&mut b, from_raw_parts(s as *const u8, slen));
            } else {
              let out = format_bytes(&spec, from_raw_parts(s as *const u8, slen));
              lua_l_addlstring(&mut b, &out);
            }
          }
          b'*' => {
            // %* is parsed above, so if we got here we must have %...*
            luaL_error!(l, "'%*' does not take a form");
          }
          _ => {
            // also treat cases 'pnLlh'
            // 指示符字节经 char 格式化：≥0x80 时按 Unicode 码点重编码为多字节，
            // cpp 写原始单字节——仅错误文本字节级差异（DELIBERATE DEVIATION）
            luaL_error!(l, "invalid option '%{}' to 'format'", indicator as char);
          }
        }
      }
    }

    lua_l_pushresult(&mut b);
    1
  }
}
