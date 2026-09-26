use ulua_common::functions::read_var_int_64::try_read_var_int_64;

/// 对齐 cpp `readVarInt = static_cast<unsigned int>(readVarInt64(...))`
/// （`lvmload.cpp:135`）：截断 64 位读取结果；流截断同样以 `None` 上抛，
/// 由 `loadsafe` 转成「损坏字节码」错误。
pub(crate) fn read_var_int(data: &[u8], offset: &mut usize) -> Option<u32> {
  try_read_var_int_64(data, offset).map(|value| value as u32)
}
