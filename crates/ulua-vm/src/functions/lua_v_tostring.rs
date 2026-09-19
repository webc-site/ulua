use core::ffi::c_char;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_s_newlstr::luaS_newlstr, luai_num_2_str::luai_num2str},
  macros::{
    luai_maxnum_2_str::LUAI_MAXNUM2STR, nvalue::nvalue, setsvalue::setsvalue,
    ttisnumber::ttisnumber,
  },
  type_aliases::{lua_state::LuaState, stk_id::StkId},
};

/// 2^53：|n| ≤ 2^53 的整数值 double，最短十进制表示与精确整数字面值一致，
/// 转 `i64` 也无损，可安全走 itoa 快路径
const TWO_POW_53_F64: f64 = 9007199254740992.0;

/// 整数值 double 的 itoa 快路径判定：可无损转为 `i64` 时返回该整数。
///
/// 界取 |n| ≤ 2^53：更大的整数值 double（如 2^63）经 schubfach 输出的是
/// 最短往返表示（如 `-9223372036854776000`），与精确整数字面值不同，必须走
/// `luai_num2str`。
///
/// `-0.0` 返回 None：tostring 契约要求输出 `-0`。NaN / ±∞ 不满足判定，
/// 同样落回慢路径。
pub(crate) fn lua_v_int_fast(n: f64) -> Option<i64> {
  if n.abs() <= TWO_POW_53_F64 && n == n.trunc() && !(n == 0.0 && n.is_sign_negative()) {
    Some(n as i64)
  } else {
    None
  }
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_v_tostring(l: *mut LuaState, obj: StkId) -> i32 {
  unsafe {
    if !ttisnumber!(obj) {
      return 0;
    }
    let n = nvalue!(obj);

    // 整数值 double 快路径：itoa 直出十进制，跳过 schubfach 全流程
    if let Some(v) = lua_v_int_fast(n) {
      let mut b = itoa::Buffer::new();
      let s = b.format(v);
      setsvalue!(
        l,
        obj,
        luaS_newlstr(l, s.as_ptr().cast::<c_char>(), s.len())
      );
      return 1;
    }

    let mut s = [0 as c_char; LUAI_MAXNUM2STR as usize];
    let e = luai_num2str(s.as_mut_ptr(), n);
    LUAU_ASSERT!((e as usize) < (s.as_ptr() as usize + s.len()));
    setsvalue!(
      l,
      obj,
      luaS_newlstr(l, s.as_ptr(), e.offset_from(s.as_ptr()) as usize)
    );
    1
  }
}

#[cfg(test)]
mod tests {
  use core::{ffi, slice::from_raw_parts};

  use super::{TWO_POW_53_F64, lua_v_int_fast};
  use crate::functions::luai_num_2_str::luai_num_2_str;

  fn luau_str(n: f64) -> String {
    let mut buf = [0 as ffi::c_char; 48];
    let end = unsafe { luai_num_2_str(buf.as_mut_ptr(), n) };
    let len = unsafe { end.offset_from(buf.as_ptr()) } as usize;
    String::from_utf8_lossy(
      // SAFETY: luai_num2str 只写 ASCII
      unsafe { from_raw_parts(buf.as_ptr().cast::<u8>(), len) },
    )
    .into_owned()
  }

  #[test]
  fn int_fast_path_matches_luai_num2str() {
    let mut vals = vec![
      0.0,
      -0.0,
      1.0,
      -1.0,
      0.5,
      -0.5,
      1.0 / 3.0,
      f64::INFINITY,
      f64::NEG_INFINITY,
      f64::NAN,
      f64::MAX,
      f64::MIN_POSITIVE,
      5e-324,
      2f64.powi(52),
      2f64.powi(53),
      2f64.powi(53) + 2.0,
      1e17,
      -1e17,
      1e21,
      i64::MIN as f64,
      i64::MAX as f64,
      9.2e18,
      9.3e18,
    ];
    for i in -2000..2000 {
      vals.push(i as f64);
      vals.push(i as f64 + 0.5);
    }
    for n in vals {
      // 快路径命中时，itoa 输出必须与 luai_num2str 逐字节一致
      if let Some(v) = lua_v_int_fast(n) {
        assert_eq!(itoa::Buffer::new().format(v), luau_str(n), "n={n:?}");
      }
    }
    // 边界契约：-0.0 / 超出 2^53 / 非整数值不进快路径
    assert_eq!(lua_v_int_fast(-0.0), None);
    assert_eq!(lua_v_int_fast(TWO_POW_53_F64), Some(1 << 53));
    assert_eq!(lua_v_int_fast(-TWO_POW_53_F64), Some(-(1 << 53)));
    assert_eq!(lua_v_int_fast(9.3e18), None);
    assert_eq!(lua_v_int_fast(f64::INFINITY), None);
    assert_eq!(lua_v_int_fast(f64::NAN), None);
    assert_eq!(lua_v_int_fast(0.5), None);
    assert_eq!(lua_v_int_fast(0.0), Some(0));
  }
}
