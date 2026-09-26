use ulua_common::{
  functions::is_c_space::is_c_space, strtod_shim::parse_c_double, strtoull_shim::parse_c_ull,
};

/// 数值解析收尾（cpp `luaO_str2d`/`luaO_str2l` 共用的 endptr 后处理）：NUL 消费
/// 完（`bytes.get(end)` 越界即 cpp 侧 NUL 终止符）返回结果；跳过尾随空白；
/// 仍有非空白字符则解析失败。
#[inline]
pub(crate) fn num_parse_tail<T: Copy>(bytes: &[u8], result: T, mut end: usize) -> Option<T> {
  if bytes.get(end).is_none() {
    return Some(result); // most common case（NUL 终止）
  }
  while matches!(bytes.get(end), Some(&c) if is_c_space(c)) {
    end += 1;
  }
  if bytes.get(end).is_some() {
    return None; // invalid trailing characters?
  }
  Some(result)
}

/// cpp `luaO_str2d`（lobject.cpp:86）：把 C 字符串解析为整数/浮点数。
///
/// cpp 用 `lua_Number* result` 出参 + `bool` 返回值，Rust 版折叠为 `Option<f64>`。
/// cpp 侧在 lobject.h 以 `LUAI_FUNC` 导出，Rust 侧对应保持 `pub`。入参为 NUL 前
/// 原始字节切片（读取经 `ulua-common` 的 `cstr_bytes` 门面收口）。
///
/// 解析语义与 C `strtod`/`strtoull` 的 endptr 逐一对齐：完全失败（0 消费）、
/// 尾随非空白字符均返回 `None`；`0x` 前缀走 16 进制重解析。
pub fn lua_o_str_2_d(bytes: &[u8]) -> Option<f64> {
  let (mut result, mut end) = parse_c_double(bytes);
  if end == 0 {
    return None; // conversion failed
  }
  if matches!(bytes.get(end), Some(b'x') | Some(b'X')) {
    // maybe an hexadecimal constant?
    let (hex, hexend) = parse_c_ull(bytes, 16);
    result = hex as f64;
    end = hexend;
  }
  num_parse_tail(bytes, result, end)
}

// §9.3：行为测试已迁至 `tests/str2d_parsing.rs`（本函数即 cpp lobject.h 的
// LUAI_FUNC 导出面），src 内不再保留单元测试属性。
