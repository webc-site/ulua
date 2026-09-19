use core::{ffi::c_int, ptr::null};

use ulua_common::fflag;

use crate::{
  functions::{
    buffer_copy::buffer_copy, buffer_create::buffer_create, buffer_fill::buffer_fill,
    buffer_fromstring::buffer_fromstring, buffer_len::buffer_len, buffer_readbits::buffer_readbits,
    buffer_readfp::buffer_readfp, buffer_readinteger::buffer_readinteger,
    buffer_readlong::buffer_readlong, buffer_readstring::buffer_readstring,
    buffer_tostring::buffer_tostring, buffer_writebits::buffer_writebits,
    buffer_writefp::buffer_writefp, buffer_writeinteger::buffer_writeinteger,
    buffer_writelong::buffer_writelong, buffer_writestring::buffer_writestring,
    lua_l_register::lua_l_register,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

/// 泛型函数不能声明为 `extern "C"`，为每个宽度生成具体包装（FFI 注册表需要 C ABI）。
macro_rules! integer_wrappers {
  ($(($name:ident, $fn:ident, $ty:ty)),+ $(,)?) => {
    $(
      /// # Safety
      /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
      unsafe extern "C-unwind" fn $name(l: *mut lua_State) -> c_int {
        unsafe { $fn::<$ty>(l) }
      }
    )+
  };
}

integer_wrappers! {
  (buffer_readinteger_i8, buffer_readinteger, i8),
  (buffer_readinteger_u8, buffer_readinteger, u8),
  (buffer_readinteger_i16, buffer_readinteger, i16),
  (buffer_readinteger_u16, buffer_readinteger, u16),
  (buffer_readinteger_i32, buffer_readinteger, i32),
  (buffer_readinteger_u32, buffer_readinteger, u32),
  (buffer_writeinteger_i8, buffer_writeinteger, i8),
  (buffer_writeinteger_u8, buffer_writeinteger, u8),
  (buffer_writeinteger_i16, buffer_writeinteger, i16),
  (buffer_writeinteger_u16, buffer_writeinteger, u16),
  (buffer_writeinteger_i32, buffer_writeinteger, i32),
  (buffer_writeinteger_u32, buffer_writeinteger, u32),
}

macro_rules! fp_wrappers {
  ($(($name:ident, $fn:ident, $ty:ty, $raw:ty)),+ $(,)?) => {
    $(
      /// # Safety
      /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
      unsafe extern "C-unwind" fn $name(l: *mut lua_State) -> c_int {
        unsafe { $fn::<$ty, $raw>(l) }
      }
    )+
  };
}

fp_wrappers! {
  (buffer_readfp_f32, buffer_readfp, f32, u32),
  (buffer_readfp_f64, buffer_readfp, f64, u64),
  (buffer_writefp_f32, buffer_writefp, f32, u32),
  (buffer_writefp_f64, buffer_writefp, f64, u64),
}

/// 注册表条目：`(lua 名, 包装函数)`。
macro_rules! reg {
  ($name:expr, $func:expr) => {
    LuaLReg {
      name: $name.as_ptr(),
      func: Some($func),
    }
  };
}

// 共享基表（integer 开关不影响的前 26 个条目，编译期拼接出两个注册表）
const BUFFER_BASE: [LuaLReg; 26] = [
  reg!(c"create", buffer_create),
  reg!(c"fromstring", buffer_fromstring),
  reg!(c"tostring", buffer_tostring),
  reg!(c"readi8", buffer_readinteger_i8),
  reg!(c"readu8", buffer_readinteger_u8),
  reg!(c"readi16", buffer_readinteger_i16),
  reg!(c"readu16", buffer_readinteger_u16),
  reg!(c"readi32", buffer_readinteger_i32),
  reg!(c"readu32", buffer_readinteger_u32),
  reg!(c"readf32", buffer_readfp_f32),
  reg!(c"readf64", buffer_readfp_f64),
  reg!(c"writei8", buffer_writeinteger_i8),
  reg!(c"writeu8", buffer_writeinteger_u8),
  reg!(c"writei16", buffer_writeinteger_i16),
  reg!(c"writeu16", buffer_writeinteger_u16),
  reg!(c"writei32", buffer_writeinteger_i32),
  reg!(c"writeu32", buffer_writeinteger_u32),
  reg!(c"writef32", buffer_writefp_f32),
  reg!(c"writef64", buffer_writefp_f64),
  reg!(c"readstring", buffer_readstring),
  reg!(c"writestring", buffer_writestring),
  reg!(c"len", buffer_len),
  reg!(c"copy", buffer_copy),
  reg!(c"fill", buffer_fill),
  reg!(c"readbits", buffer_readbits),
  reg!(c"writebits", buffer_writebits),
];

// integer 开启时追加 readinteger/writeinteger（转调 buffer_readlong/writelong）
const INTEGER_TAIL: [LuaLReg; 3] = [
  reg!(c"readinteger", buffer_readlong),
  reg!(c"writeinteger", buffer_writelong),
  LuaLReg {
    name: null(),
    func: None,
  },
];

const NULL_TAIL: [LuaLReg; 1] = [LuaLReg {
  name: null(),
  func: None,
}];

// 编译期拼接注册表，剩余槽位保持 null 终止
const fn join<const N: usize>(a: &[LuaLReg], b: &[LuaLReg]) -> [LuaLReg; N] {
  let mut out = [LuaLReg {
    name: null(),
    func: None,
  }; N];
  let mut i = 0;
  while i < a.len() {
    out[i] = a[i];
    i += 1;
  }
  let mut j = 0;
  while j < b.len() {
    out[i + j] = b[j];
    j += 1;
  }
  out
}

static BUFFER_LIB: SyncLuaLReg<29> = SyncLuaLReg(join::<29>(&BUFFER_BASE, &INTEGER_TAIL));

static BUFFER_LIB_NO_INTEGER: SyncLuaLReg<27> = SyncLuaLReg(join::<27>(&BUFFER_BASE, &NULL_TAIL));

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_buffer(l: *mut lua_State) -> c_int {
  unsafe {
    let buffer_lib = if fflag::LuauIntegerLibrary.get() {
      BUFFER_LIB.0.as_ptr()
    } else {
      BUFFER_LIB_NO_INTEGER.0.as_ptr()
    };

    lua_l_register(l, c"buffer".as_ptr(), buffer_lib);
    1
  }
}
