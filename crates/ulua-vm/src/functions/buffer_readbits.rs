use crate::{
  functions::{
    buffer_window::{buffer_bit_bounds, buffer_data_ref, BITS_PER_BYTE},
    load_bits_u64::load_bits_u64,
    lua_pushunsigned::lua_pushunsigned,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe fn buffer_readbits(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 为存活调用帧；buffer_bit_bounds 的界校验保证读取区间
  // [startbyte, endbyte) 落在 buffer 数据界内，越界路径走报错
  unsafe {
    let buf = buffer_data_ref(l, 1);
    let bitoffset = (*l).check_number(2) as i64;
    let bitcount = (*l).check_integer(3);
    let (startbyte, endbyte) = buffer_bit_bounds(l, buf.len(), bitoffset, bitcount);

    // 字节区间装入 u64；buffer_bit_bounds 的越界检查保证区间落在数据界内且 ≤ 8 字节
    let data = load_bits_u64(&buf[startbyte..endbyte]);

    let subbyteoffset = (bitoffset & (BITS_PER_BYTE as i64 - 1)) as u64;
    let mask = (1u64 << bitcount as u64) - 1;

    let result = ((data >> subbyteoffset) & mask) as u32;
    lua_pushunsigned(l, result);

    1
  }
}

lua_lib_fn!(pub(crate) fn buffer_readbits, buffer_readbits_arm);
