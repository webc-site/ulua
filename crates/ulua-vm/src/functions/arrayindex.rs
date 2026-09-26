use crate::macros::luai_numeq::luai_numeq;

pub fn arrayindex(key: f64) -> i32 {
  let i = key as i32;

  if luai_numeq(i as f64, key) { i } else { -1 }
}
