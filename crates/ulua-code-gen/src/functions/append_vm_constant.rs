extern crate alloc;

use alloc::string::String;
use core::slice::from_raw_parts;

use ulua_vm::{
  macros::{gco_2_ts::gco2ts, getstr::getstr, lua_vector_size::LUA_VECTOR_SIZE},
  records::{g_cheader::GCheader, t_string::tstring},
  type_aliases::proto::Proto,
};

use crate::functions::{
  append::append, format_g::format_g, is_printable_string_constant::is_printable_string_constant,
};

// Lua value type tags (lobject.h)
const LUA_TNIL: i32 = 0;
const LUA_TBOOLEAN: i32 = 1;
const LUA_TNUMBER: i32 = 3;
const LUA_TINTEGER: i32 = 4;
const LUA_TVECTOR: i32 = 5;
const LUA_TSTRING: i32 = 6;

const K_MAX_STRING_CONSTANT_PRINT_LENGTH: usize = 16;

// tstring's `len`/`data` are crate-private in luau-vm; mirror the layout so we
// can read the length (the `getstr` macro already exposes the data pointer).
#[repr(C)]
struct tstringHeader {
  hdr: GCheader,
  _padding1: [u8; 1],
  atom: i16,
  _padding2: [u8; 2],
  next: *mut tstring,
  hash: u32,
  len: u32,
  data: [u8; 1],
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn append_vm_constant(result: &mut String, proto: *mut Proto, index: i32) {
  unsafe {
    let constant = *(*proto).k.add(index as usize);

    if constant.tt == LUA_TNIL {
      append(result, format_args!("nil"));
    } else if constant.tt == LUA_TBOOLEAN {
      append(
        result,
        format_args!(
          "{}",
          if constant.value.b != 0 {
            "true"
          } else {
            "false"
          }
        ),
      );
    } else if constant.tt == LUA_TNUMBER {
      let n = constant.value.n;
      if n.is_nan() {
        append(result, format_args!("nan"));
      } else {
        // C++ uses "%.17g"; Rust `{}` prints the shortest round-tripping form.
        result.push_str(&format_g(n, 17));
      }
    } else if constant.tt == LUA_TINTEGER {
      append(result, format_args!("{}i", { constant.value.l }));
    } else if constant.tt == LUA_TSTRING {
      let str_ts = gco2ts!(constant.value.gc) as *const _ as *const tstring;
      let data = getstr(str_ts);
      let len = (*(str_ts as *const tstringHeader)).len;

      let full_slice = from_raw_parts(data as *const u8, len as usize);
      if is_printable_string_constant(full_slice) {
        let n = (len as usize).min(K_MAX_STRING_CONSTANT_PRINT_LENGTH);
        let text = String::from_utf8_lossy(&full_slice[..n]);

        if (len as usize) < K_MAX_STRING_CONSTANT_PRINT_LENGTH {
          append(result, format_args!("'{}'", text));
        } else {
          append(result, format_args!("'{}'...", text));
        }
      }
    } else if constant.tt == LUA_TVECTOR {
      // value.v is float[2] in the union; v[2]/v[3] index into the trailing
      // TValue storage, mirroring the C++ `const float* v = constant.value.v`.
      let v = &constant.value as *const _ as *const f32;

      if LUA_VECTOR_SIZE == 4 {
        if *v.add(3) != 0.0 {
          append(
            result,
            format_args!(
              "{}, {}, {}, {}",
              format_g(*v.add(0) as f64, 9),
              format_g(*v.add(1) as f64, 9),
              format_g(*v.add(2) as f64, 9),
              format_g(*v.add(3) as f64, 9)
            ),
          );
        } else {
          append(
            result,
            format_args!(
              "{}, {}, {}",
              format_g(*v.add(0) as f64, 9),
              format_g(*v.add(1) as f64, 9),
              format_g(*v.add(2) as f64, 9)
            ),
          );
        }
      } else {
        append(
          result,
          format_args!(
            "{}, {}, {}",
            format_g(*v.add(0) as f64, 9),
            format_g(*v.add(1) as f64, 9),
            format_g(*v.add(2) as f64, 9)
          ),
        );
      }
    }
  }
}
