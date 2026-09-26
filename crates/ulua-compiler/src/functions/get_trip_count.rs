/// f64→i32 往返精确的幅值上限（C++ `32767`，超界即视为非常量步长域）
const K_EXACT_INT_LIMIT: f64 = 32767.0;
/// 恰为小整数值的 Number → i32；否则返回 None。
fn to_int(v: f64) -> Option<i32> {
  if (-K_EXACT_INT_LIMIT..=K_EXACT_INT_LIMIT).contains(&v) && (v as i32 as f64) == v {
    Some(v as i32)
  } else {
    None
  }
}

pub(crate) fn get_trip_count(from: f64, to: f64, step: f64) -> i32 {
  // 用整数计算迭代次数，保证循环数学（重复加法）是精确的
  let (Some(fromi), Some(toi), Some(stepi)) = (to_int(from), to_int(to), to_int(step)) else {
    return -1;
  };

  if stepi == 0 {
    return -1;
  }

  if (stepi < 0 && toi > fromi) || (stepi > 0 && toi < fromi) {
    return 0;
  }

  (toi - fromi) / stepi + 1
}
