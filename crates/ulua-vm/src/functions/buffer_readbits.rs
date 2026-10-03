use crate::{
  functions::{
    buffer_window::{BITS_PER_BYTE, buffer_bit_bounds, buffer_data_ref},
    load_bits_u64::load_bits_u64,
    lua_pushunsigned::lua_pushunsigned,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；索引 1 为
/// buffer、2 为位偏移、3 为位宽，[`buffer_bit_bounds`] 的界校验保证读取区间落在数据界
/// 内、越界走报错；结果数值压栈需栈顶余量。
pub(crate) fn buffer_readbits(l: &mut LuaState) -> i32 {
  let buf = buffer_data_ref(l, 1);
  let bitoffset = l.check_number(2) as i64;
  let bitcount = l.check_integer(3);
  let (startbyte, endbyte) = buffer_bit_bounds(l, buf.len(), bitoffset, bitcount);

  // 字节区间装入 u64；buffer_bit_bounds 的越界检查保证区间落在数据界内且 ≤ 8 字节
  let data = load_bits_u64(&buf[startbyte..endbyte]);

  let subbyteoffset = (bitoffset & (BITS_PER_BYTE as i64 - 1)) as u64;
  let mask = (1u64 << bitcount as u64) - 1;

  let result = ((data >> subbyteoffset) & mask) as u32;
  lua_pushunsigned(l, result);

  1
}

lua_lib_fn!(pub(crate) fn buffer_readbits @ref, buffer_readbits_arm);
