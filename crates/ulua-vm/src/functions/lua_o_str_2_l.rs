use ulua_common::strtoull_shim::{parse_c_ll, parse_c_ull};

use crate::functions::lua_o_str_2_d::num_parse_tail;

/// cpp `luaO_str2l`：按 `base` 把 C 字符串解析为 64 位整数。入参为 NUL 前原始
/// 字节切片（读取经 `ulua-common` 的 `cstr_bytes` 门面收口）。
///
/// cpp 用 `lua_Integer64* result` 出参 + `bool` 返回值，Rust 版折叠为 `Option<i64>`。
/// 解析语义与 C `strtoll`/`strtoull` 的 endptr 逐一对齐：0 消费（完全失败）或
/// 尾随非空白字符返回 `None`；base 10 下 `0x` 前缀走 16 进制重解析。
pub(crate) fn lua_o_str_2_l(bytes: &[u8], base: i32) -> Option<i64> {
  let (mut result, mut end);

  if base == 10 {
    let (v, consumed) = parse_c_ll(bytes, base as u32);
    result = v;
    end = consumed;
    if end == 0 {
      return None; // conversion failed
    }
    if matches!(bytes.get(end), Some(b'x') | Some(b'X')) {
      // maybe an hexadecimal constant?
      let (hex, hexend) = parse_c_ull(bytes, 16);
      result = hex as i64;
      end = hexend;
    }
  } else {
    // unsigned parse in other bases
    let (v, consumed) = parse_c_ull(bytes, base as u32);
    result = v as i64;
    end = consumed;
    if end == 0 {
      return None;
    }
  }

  num_parse_tail(bytes, result, end)
}
