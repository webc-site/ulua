use crate::{
  macros::{lua_l_error::luaL_error, mc::MC, nb::NB, szint::SZINT},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `bytes` 长度须为本次读取的 `size` 字节（`str_unpack` 已用 `luaL_argcheck` 校验数据串
/// 剩余量，DEBUG 下 `debug_assert` 兜底；失配 Rust 下退化为 panic 而非 UB）。`l` 仅作
/// 报错用：高位字节与符号扩展不符时抛 "N-byte integer does not fit into Lua Integer"、
/// 不返回。cpp lstrlib.cpp:1532 `unpackint`。
pub(crate) unsafe fn unpackint(
  l: *mut LuaState,
  bytes: &[u8],
  islittle: i32,
  size: i32,
  issigned: i32,
) -> i64 {
  debug_assert!(bytes.len() >= size as usize);

  let mut res: u64 = 0;
  let limit = if size <= SZINT { size } else { SZINT };

  for i in (0..limit).rev() {
    res <<= NB as u32;
    let idx = if islittle != 0 { i } else { size - 1 - i };
    res |= bytes[idx as usize] as u64;
  }

  if size < SZINT {
    if issigned != 0 {
      let mask = 1u64 << ((size * NB) - 1);
      // C does `(res ^ mask) - mask` in unsigned (wrapping) arithmetic for sign extension.
      res = (res ^ mask).wrapping_sub(mask);
    }
  } else if size > SZINT {
    let mask = if issigned == 0 || res as i64 >= 0 {
      0
    } else {
      MC
    };
    for i in limit..size {
      let idx = if islittle != 0 { i } else { size - 1 - i };
      let byte = bytes[idx as usize] as i32;
      if byte != mask {
        // Safety: 契约保证 l 为存活调用帧，超界即经它抛错、不返回
        unsafe {
          luaL_error!(l, "{}-byte integer does not fit into Lua Integer", size);
        }
      }
    }
  }

  res as i64
}
