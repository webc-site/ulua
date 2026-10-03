use crate::{
  functions::{
    buffer_window::{BITS_PER_BYTE, buffer_bit_bounds, buffer_data_ref},
    load_bits_u64::load_bits_u64,
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；索引 1 为
/// buffer、2 为位偏移、3 为位宽、4 为待写入值，[`buffer_bit_bounds`] 的界校验保证写入
/// 区间落在数据界内、越界走报错。
pub(crate) fn buffer_writebits(l: &mut LuaState) -> i32 {
  let buf = buffer_data_ref(l, 1);
  let bitoffset = l.check_number(2) as i64;
  let bitcount = l.check_integer(3);
  let value = lua_l_checkunsigned(l, 4);
  let (startbyte, endbyte) = buffer_bit_bounds(l, buf.len(), bitoffset, bitcount);

  // 字节区间装入 u64（与 buffer_readbits 共享同一装载逻辑）
  let mut data = load_bits_u64(&buf[startbyte..endbyte]);

  let subbyteoffset = (bitoffset & (BITS_PER_BYTE as i64 - 1)) as u64;
  let mask = ((1u64 << bitcount) - 1) << subbyteoffset;

  data = (data & !mask) | ((value as u64) << subbyteoffset & mask);

  // cpp 大端逐字节写 / 小端整块拷贝两条分支的字节序等价（见 load_bits_u64 证明），
  // 统一为按小端字节序回写区间
  let region = &mut buf[startbyte..endbyte];
  region.copy_from_slice(&data.to_le_bytes()[..region.len()]);

  0
}

lua_lib_fn!(pub(crate) fn buffer_writebits @ref, buffer_writebits_arm);
