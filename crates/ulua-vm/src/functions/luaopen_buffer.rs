use ulua_common::fflag;

use crate::{
  functions::{
    buffer_copy::buffer_copy_arm, buffer_create::buffer_create_arm, buffer_fill::buffer_fill_arm,
    buffer_fromstring::buffer_fromstring_arm, buffer_len::buffer_len_arm,
    buffer_readbits::buffer_readbits_arm, buffer_readfp::buffer_readfp,
    buffer_readinteger::buffer_readinteger, buffer_readlong::buffer_readlong_arm,
    buffer_readstring::buffer_readstring_arm, buffer_tostring::buffer_tostring_arm,
    buffer_writebits::buffer_writebits_arm, buffer_writefp::buffer_writefp,
    buffer_writeinteger::buffer_writeinteger, buffer_writelong::buffer_writelong_arm,
    buffer_writestring::buffer_writestring_arm, lua_l_register::lua_l_register,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 `CStr`/`c"…"`）。
const LIB_BUFFER: &[u8] = b"buffer\0";

/// 泛型函数不能声明为 `extern "C"`，为每个宽度生成具体包装（FFI 注册表需要 C ABI）。
macro_rules! integer_wrappers {
  ($(($name:ident, $fn:ident, $ty:ty)),+ $(,)?) => {
    $(
      /// # Safety
      /// `l` 须为存活 LuaState 且已处于本 C 函数的受保护调用帧：栈上按 buffer 库约定备好参数
      /// （1 号 buffer、2 号偏移等，`$fn::<$ty>` 会经 checkbuffer/checkinteger 读取并按需抛错/触发 GC）。
      /// cpp/VM/src/lbuflib.cpp:67 buffer_readinteger、:88 buffer_writeinteger。
      unsafe extern "C-unwind" fn $name(l: *mut LuaState) -> i32 {
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
      /// `l` 须为存活 LuaState 且处于本 C 函数的受保护调用帧，栈上备好 buffer 库约定参数
      /// （`$fn::<$ty, $raw>` 读取 buffer/偏移并按需抛错/GC，`$ty`/`$raw` 为编译期选定的浮点宽度对）。
      /// cpp/VM/src/lbuflib.cpp:147 buffer_readfp、:174 buffer_writefp。
      unsafe extern "C-unwind" fn $name(l: *mut LuaState) -> i32 {
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

// 共享基表（integer 开关不影响的前 26 个条目，编译期拼接出两个注册表）
const BUFFER_BASE: [LuaLReg; 26] = [
  LuaLReg::new(b"create", buffer_create_arm),
  LuaLReg::new(b"fromstring", buffer_fromstring_arm),
  LuaLReg::new(b"tostring", buffer_tostring_arm),
  LuaLReg::new(b"readi8", buffer_readinteger_i8),
  LuaLReg::new(b"readu8", buffer_readinteger_u8),
  LuaLReg::new(b"readi16", buffer_readinteger_i16),
  LuaLReg::new(b"readu16", buffer_readinteger_u16),
  LuaLReg::new(b"readi32", buffer_readinteger_i32),
  LuaLReg::new(b"readu32", buffer_readinteger_u32),
  LuaLReg::new(b"readf32", buffer_readfp_f32),
  LuaLReg::new(b"readf64", buffer_readfp_f64),
  LuaLReg::new(b"writei8", buffer_writeinteger_i8),
  LuaLReg::new(b"writeu8", buffer_writeinteger_u8),
  LuaLReg::new(b"writei16", buffer_writeinteger_i16),
  LuaLReg::new(b"writeu16", buffer_writeinteger_u16),
  LuaLReg::new(b"writei32", buffer_writeinteger_i32),
  LuaLReg::new(b"writeu32", buffer_writeinteger_u32),
  LuaLReg::new(b"writef32", buffer_writefp_f32),
  LuaLReg::new(b"writef64", buffer_writefp_f64),
  LuaLReg::new(b"readstring", buffer_readstring_arm),
  LuaLReg::new(b"writestring", buffer_writestring_arm),
  LuaLReg::new(b"len", buffer_len_arm),
  LuaLReg::new(b"copy", buffer_copy_arm),
  LuaLReg::new(b"fill", buffer_fill_arm),
  LuaLReg::new(b"readbits", buffer_readbits_arm),
  LuaLReg::new(b"writebits", buffer_writebits_arm),
];

// integer 开启时追加 readinteger/writeinteger（转调 buffer_readlong/writelong）
const INTEGER_TAIL: [LuaLReg; 2] = [
  LuaLReg::new(b"readinteger", buffer_readlong_arm),
  LuaLReg::new(b"writeinteger", buffer_writelong_arm),
];

// 编译期拼接注册表
const fn join<const N: usize>(a: &[LuaLReg], b: &[LuaLReg]) -> [LuaLReg; N] {
  let mut out = [a[0]; N];
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

static BUFFER_LIB: [LuaLReg; 28] = join::<28>(&BUFFER_BASE, &INTEGER_TAIL);

/// # Safety
/// `l` 须为存活 LuaState 且栈顶之上至少留 1 个空槽（`lua_l_register` 会 push 库表并作为返回值）；
/// 须在可分配/GC 的受保护帧内调用。
/// cpp/VM/src/lbuflib.cpp:433 luaopen_buffer。
pub unsafe extern "C-unwind" fn luaopen_buffer(l: *mut LuaState) -> i32 {
  unsafe {
    let buffer_lib: &[LuaLReg] = if fflag::LuauIntegerLibrary.get() {
      &BUFFER_LIB
    } else {
      &BUFFER_BASE
    };

    lua_l_register(l, LIB_BUFFER.as_ptr().cast(), buffer_lib);
    1
  }
}
