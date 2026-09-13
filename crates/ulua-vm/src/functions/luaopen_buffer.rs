use core::{ffi::c_int, ptr::null};

use ulua_common::FFlag;

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

// 泛型函数不能声明为 `extern "C"`,为每个整数宽度提供具体包装(FFI 注册表需要 C ABI)
unsafe extern "C-unwind" fn buffer_readinteger_i8(l: *mut lua_State) -> c_int {
  unsafe { buffer_readinteger::<i8>(l) }
}
unsafe extern "C-unwind" fn buffer_readinteger_u8(l: *mut lua_State) -> c_int {
  unsafe { buffer_readinteger::<u8>(l) }
}
unsafe extern "C-unwind" fn buffer_readinteger_i16(l: *mut lua_State) -> c_int {
  unsafe { buffer_readinteger::<i16>(l) }
}
unsafe extern "C-unwind" fn buffer_readinteger_u16(l: *mut lua_State) -> c_int {
  unsafe { buffer_readinteger::<u16>(l) }
}
unsafe extern "C-unwind" fn buffer_readinteger_i32(l: *mut lua_State) -> c_int {
  unsafe { buffer_readinteger::<i32>(l) }
}
unsafe extern "C-unwind" fn buffer_readinteger_u32(l: *mut lua_State) -> c_int {
  unsafe { buffer_readinteger::<u32>(l) }
}

// 泛型函数不能声明为 `extern "C"`,为每个宽度提供具体包装(FFI 注册表需要 C ABI)
unsafe extern "C-unwind" fn buffer_writeinteger_i8(l: *mut lua_State) -> c_int {
  unsafe { buffer_writeinteger::<i8>(l) }
}
unsafe extern "C-unwind" fn buffer_writeinteger_u8(l: *mut lua_State) -> c_int {
  unsafe { buffer_writeinteger::<u8>(l) }
}
unsafe extern "C-unwind" fn buffer_writeinteger_i16(l: *mut lua_State) -> c_int {
  unsafe { buffer_writeinteger::<i16>(l) }
}
unsafe extern "C-unwind" fn buffer_writeinteger_u16(l: *mut lua_State) -> c_int {
  unsafe { buffer_writeinteger::<u16>(l) }
}
unsafe extern "C-unwind" fn buffer_writeinteger_i32(l: *mut lua_State) -> c_int {
  unsafe { buffer_writeinteger::<i32>(l) }
}
unsafe extern "C-unwind" fn buffer_writeinteger_u32(l: *mut lua_State) -> c_int {
  unsafe { buffer_writeinteger::<u32>(l) }
}
unsafe extern "C-unwind" fn buffer_readfp_f32(l: *mut lua_State) -> c_int {
  unsafe { buffer_readfp::<f32, u32>(l) }
}
unsafe extern "C-unwind" fn buffer_readfp_f64(l: *mut lua_State) -> c_int {
  unsafe { buffer_readfp::<f64, u64>(l) }
}
unsafe extern "C-unwind" fn buffer_writefp_f32(l: *mut lua_State) -> c_int {
  unsafe { buffer_writefp::<f32, u32>(l) }
}
unsafe extern "C-unwind" fn buffer_writefp_f64(l: *mut lua_State) -> c_int {
  unsafe { buffer_writefp::<f64, u64>(l) }
}

struct SyncLuaLReg<const N: usize>([LuaLReg; N]);
unsafe impl<const N: usize> Sync for SyncLuaLReg<N> {}

// 共享基表（integer 开关不影响的前 26 个条目，编译期拼接出两个注册表）
const BUFFER_BASE: [LuaLReg; 26] = [
  LuaLReg {
    name: c"create".as_ptr(),
    func: Some(buffer_create),
  },
  LuaLReg {
    name: c"fromstring".as_ptr(),
    func: Some(buffer_fromstring),
  },
  LuaLReg {
    name: c"tostring".as_ptr(),
    func: Some(buffer_tostring),
  },
  LuaLReg {
    name: c"readi8".as_ptr(),
    func: Some(buffer_readinteger_i8),
  },
  LuaLReg {
    name: c"readu8".as_ptr(),
    func: Some(buffer_readinteger_u8),
  },
  LuaLReg {
    name: c"readi16".as_ptr(),
    func: Some(buffer_readinteger_i16),
  },
  LuaLReg {
    name: c"readu16".as_ptr(),
    func: Some(buffer_readinteger_u16),
  },
  LuaLReg {
    name: c"readi32".as_ptr(),
    func: Some(buffer_readinteger_i32),
  },
  LuaLReg {
    name: c"readu32".as_ptr(),
    func: Some(buffer_readinteger_u32),
  },
  LuaLReg {
    name: c"readf32".as_ptr(),
    func: Some(buffer_readfp_f32),
  },
  LuaLReg {
    name: c"readf64".as_ptr(),
    func: Some(buffer_readfp_f64),
  },
  LuaLReg {
    name: c"writei8".as_ptr(),
    func: Some(buffer_writeinteger_i8),
  },
  LuaLReg {
    name: c"writeu8".as_ptr(),
    func: Some(buffer_writeinteger_u8),
  },
  LuaLReg {
    name: c"writei16".as_ptr(),
    func: Some(buffer_writeinteger_i16),
  },
  LuaLReg {
    name: c"writeu16".as_ptr(),
    func: Some(buffer_writeinteger_u16),
  },
  LuaLReg {
    name: c"writei32".as_ptr(),
    func: Some(buffer_writeinteger_i32),
  },
  LuaLReg {
    name: c"writeu32".as_ptr(),
    func: Some(buffer_writeinteger_u32),
  },
  LuaLReg {
    name: c"writef32".as_ptr(),
    func: Some(buffer_writefp_f32),
  },
  LuaLReg {
    name: c"writef64".as_ptr(),
    func: Some(buffer_writefp_f64),
  },
  LuaLReg {
    name: c"readstring".as_ptr(),
    func: Some(buffer_readstring),
  },
  LuaLReg {
    name: c"writestring".as_ptr(),
    func: Some(buffer_writestring),
  },
  LuaLReg {
    name: c"len".as_ptr(),
    func: Some(buffer_len),
  },
  LuaLReg {
    name: c"copy".as_ptr(),
    func: Some(buffer_copy),
  },
  LuaLReg {
    name: c"fill".as_ptr(),
    func: Some(buffer_fill),
  },
  LuaLReg {
    name: c"readbits".as_ptr(),
    func: Some(buffer_readbits),
  },
  LuaLReg {
    name: c"writebits".as_ptr(),
    func: Some(buffer_writebits),
  },
];

// integer 开启时追加 readinteger/writeinteger（转调 buffer_readlong/writelong）
const INTEGER_TAIL: [LuaLReg; 3] = [
  LuaLReg {
    name: c"readinteger".as_ptr(),
    func: Some(buffer_readlong),
  },
  LuaLReg {
    name: c"writeinteger".as_ptr(),
    func: Some(buffer_writelong),
  },
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
    let buffer_lib = if FFlag::LuauIntegerLibrary.get() {
      BUFFER_LIB.0.as_ptr()
    } else {
      BUFFER_LIB_NO_INTEGER.0.as_ptr()
    };

    lua_l_register(l, c"buffer".as_ptr(), buffer_lib);
    1
  }
}
