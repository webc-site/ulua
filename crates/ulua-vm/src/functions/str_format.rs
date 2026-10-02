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

use memchr::memchr;

use crate::{
  functions::{
    addquoted::addquoted_ref,
    format_directive::{
      format_bytes, format_char, format_float, format_int, format_uint, parse_format_spec,
    },
    lua_l_addchar::lua_l_addchar,
    lua_l_addlstring::lua_l_addlstring,
    lua_l_addvalueany::lua_l_addvalueany,
    lua_l_buffinit::lua_l_buffinit,
    lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_pushresult::lua_l_pushresult,
    scanformat::{MAX_FORMAT_SPEC_SCAN, scan_format_spec},
  },
  macros::{l_esc::L_ESC, lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// cpp lstrlib.cpp:1052：无精度限定的长字符串（>=100 字节）直接拼接，
/// 跳过宽度/精度格式化路径。
const DIRECT_APPEND_MIN_LEN: usize = 100;

/// # Safety
/// 格式串由 `lua_l_checklstring_ref` 取得：借用 GC 堆上存活字符串的字节切片；
/// Luau 字符串不会被移动或压缩，循环期间栈增长/参数读取不使其悬垂。
pub(crate) unsafe fn str_format(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 l 存活——checklstring 取 GC 堆字符串字节、buffinit/pushresult 在其栈上操作
  unsafe {
    let top = (*l).get_top();
    let mut arg: i32 = 1;
    // 借用切片形态取格式串：出参 len 由切片长度承接，游标全程按下标推进
    let f = lua_l_checklstring_ref(&mut *l, arg);

    let mut b = LuaLStrbuf::new();
    lua_l_buffinit(&mut *l, &mut b);

    let mut i: usize = 0;
    // 保留下标游走：i 随格式说明符消耗量变步（`%%` 两字节、`%*s` 三字节、
    // `%<spec><conv>` 走 p+1 字节），非等差遍历，无迭代器可套
    while i < f.len() {
      let ch = f[i];
      if ch != L_ESC as u8 {
        lua_l_addchar(&mut b, ch);
        i += 1;
        continue;
      }
      i += 1; // *++strfrmt
      // cpp 在串尾读到 Lua 字符串恒有的 NUL 终止符，此处等价取 0
      let next = f.get(i).copied().unwrap_or(0);
      if next == L_ESC as u8 {
        lua_l_addchar(&mut b, next);
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
        let indicator = f.get(i + p).copied().unwrap_or(0);
        let spec = parse_format_spec(&window[..p]);
        i += p + 1; // 消费 spec 字节与转换指示符（指示符为 0 时下方必报错）
        match indicator {
          b'c' => {
            // DELIBERATE DEVIATION：越出 i32 域的双精度实参，cpp 的
            // `(int)checknumber` 在 x86 上是 cvttsd2si UB（→INT_MIN→'\0'），
            // Rust 侧取饱和语义（→u8 截断）——定义化 C UB 角落
            let out = format_char(&spec, (*l).check_number(arg) as i32 as u8);
            lua_l_addlstring(&mut b, &out);
          }
          b'd' | b'i' => {
            let value: i64 = if (*l).is_integer_64(arg) {
              (*l).check_integer_64(arg)
            } else {
              // 越出 i64 域：cpp double→int64 转换为 UB，Rust 取饱和（同 %c 案）
              (*l).check_number(arg) as i64
            };
            let out = format_int(&spec, value);
            lua_l_addlstring(&mut b, &out);
          }
          b'o' | b'u' | b'x' | b'X' => {
            let v: u64 = if (*l).is_integer_64(arg) {
              (*l).check_integer_64(arg) as u64
            } else {
              let arg_value = (*l).check_number(arg);
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
            let out = format_float(&spec, indicator, (*l).check_number(arg));
            lua_l_addlstring(&mut b, &out);
          }
          b'q' => {
            // 取参序对齐 cpp `addquoted(L, b, arg)`：先 `check_bytes` 检出串实参
            // （非串经 "string expected" 抛出），再喂切片核心转义拼接
            addquoted_ref(&mut b, (*l).check_bytes(arg));
          }
          b's' => {
            let s = lua_l_checklstring_ref(&mut *l, arg);
            // no precision and string too long to format, or no format necessary
            if p == 0 || (spec.precision.is_none() && s.len() >= DIRECT_APPEND_MIN_LEN) {
              lua_l_addlstring(&mut b, s);
            } else {
              let out = format_bytes(&spec, s);
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

lua_lib_fn!(pub(crate) fn str_format, str_format_arm);
