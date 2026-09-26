use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_l_addlstring::lua_l_addlstring,
  macros::{maxintsize::MAXINTSIZE, mc::MC, nb::NB, szint::SZINT},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// # Safety
///
/// `buf` 必须指向本次 pack/unpack 调用的可读且可写字节区，`bytes` 在 [1,8] 且落在缓冲界内。
pub(crate) unsafe fn packint(b: *mut LuaLStrbuf, mut n: u64, islittle: i32, size: i32, neg: i32) {
  // Safety: 契约保证 `buf` 可读/可写界覆盖 `bytes` 长度，位组装与回写仅触及该界内字节
  unsafe {
    LUAU_ASSERT!(size <= MAXINTSIZE);
    // 字节序打包本质是 u8 序列：本地缓冲用 u8，仅在出参处转 c_char
    let mut buff = [0u8; MAXINTSIZE as usize];
    buff[if islittle != 0 { 0 } else { size - 1 } as usize] = (n & MC as u64) as u8;

    for i in 1..size {
      n >>= NB as u32;
      buff[if islittle != 0 { i } else { size - 1 - i } as usize] = (n & MC as u64) as u8;
    }

    if neg != 0 && size > SZINT {
      for i in SZINT..size {
        buff[if islittle != 0 { i } else { size - 1 - i } as usize] = MC as u8;
      }
    }

    lua_l_addlstring(&mut *b, &buff[..size as usize]);
  }
}
