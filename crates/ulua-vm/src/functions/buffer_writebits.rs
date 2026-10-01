use core::slice::{from_raw_parts, from_raw_parts_mut};

use crate::{
  functions::{
    buffer_window::{BITS_PER_BYTE, buffer_bit_bounds, buffer_data},
    load_bits_u64::load_bits_u64,
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe fn buffer_writebits(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证写入区间落在 buffer 数据界内且先经清零，越界路径走报错
  unsafe {
    let (buf, len) = buffer_data(l, 1);
    let bitoffset = (*l).check_number(2) as i64;
    let bitcount = (*l).check_integer(3);
    let value = lua_l_checkunsigned(&mut *l, 4);
    let (startbyte, endbyte) = buffer_bit_bounds(l, len, bitoffset, bitcount);

    // 字节区间装入 u64（与 buffer_readbits 共享同一装载逻辑）
    let region = from_raw_parts(buf.add(startbyte), endbyte - startbyte);
    let mut data = load_bits_u64(region);

    let subbyteoffset = (bitoffset & (BITS_PER_BYTE as i64 - 1)) as u64;
    let mask = ((1u64 << bitcount) - 1) << subbyteoffset;

    data = (data & !mask) | ((value as u64) << subbyteoffset & mask);

    // cpp 大端逐字节写 / 小端整块拷贝两条分支的字节序等价（见 load_bits_u64 证明），
    // 统一为按小端字节序回写区间
    let n = endbyte - startbyte;
    let region = from_raw_parts_mut(buf.add(startbyte), n);
    region.copy_from_slice(&data.to_le_bytes()[..n]);

    0
  }
}

lua_lib_fn!(pub(crate) fn buffer_writebits, buffer_writebits_arm);
