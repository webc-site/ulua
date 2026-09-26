use core::slice::from_raw_parts;

use crate::{
  functions::{
    buffer_window::{buffer_bit_bounds, buffer_data},
    load_bits_u64::load_bits_u64,
    lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checknumber::lua_l_checknumber,
    lua_pushunsigned::lua_pushunsigned,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe extern "C-unwind" fn buffer_readbits(l: *mut LuaState) -> i32 {
  // Safety: 契约保证读取区间 [byte, byte+nbits) 落在已检查的 buffer 数据界内，越界路径走报错
  unsafe {
    let (buf, len) = buffer_data(l, 1);
    let bitoffset = lua_l_checknumber(l, 2) as i64;
    let bitcount = lua_l_checkinteger(l, 3);
    let (startbyte, endbyte) = buffer_bit_bounds(l, len, bitoffset, bitcount);

    // 字节区间装入 u64；buffer_bit_bounds 的越界检查保证区间落在数据界内且 ≤ 8 字节
    let region = from_raw_parts(buf.add(startbyte), endbyte - startbyte);
    let data = load_bits_u64(region);

    let subbyteoffset = (bitoffset & 0x7) as u64;
    let mask = (1u64 << bitcount as u64) - 1;

    let result = ((data >> subbyteoffset) & mask) as u32;
    lua_pushunsigned(l, result);

    1
  }
}
