use core::ffi::c_uint;

/// LOG_2 改为编译期生成：LOG_2[i] = floor(log2(i)) + 1（即 i 的位宽），LOG_2[0] = 0，
/// 逐位等价于原手抄表（改动时曾全表编译期比对验证），抽样断言兜底防漂移。
const fn build_log_2_table() -> [u8; 256] {
  let (mut t, mut i) = ([0u8; 256], 1usize);
  while i < 256 { t[i] = (i as u32).ilog2() as u8 + 1; i += 1; }
  t
}
const LOG_2: [u8; 256] = build_log_2_table();
const _: () = assert!(LOG_2[0] == 0 && LOG_2[1] == 1 && LOG_2[2] == 2 && LOG_2[3] == 2
  && LOG_2[4] == 3 && LOG_2[127] == 7 && LOG_2[128] == 8 && LOG_2[255] == 8);

pub const fn lua_o_log_2(mut x: c_uint) -> i32 {
  let mut l: i32 = -1;

  while x >= 256 {
    l += 8;
    x >>= 8;
  }

  l + (LOG_2[x as usize] as i32)
}
