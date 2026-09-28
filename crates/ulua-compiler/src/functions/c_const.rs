use crate::records::constant::Constant;

#[inline]
pub fn cbool(v: bool) -> Constant {
  Constant::Boolean(v)
}

#[inline]
pub fn cnum(v: f64) -> Constant {
  Constant::Number(v)
}

#[inline]
pub fn cvector(x: f64, y: f64, z: f64, w: f64) -> Constant {
  Constant::Vector([x as f32, y as f32, z as f32, w as f32])
}

#[inline]
pub(crate) fn cvar() -> Constant {
  Constant::default()
}

/// 由字节切片构造字符串常量：仅借用源数据，不拷贝（与 C++ `Constant` 语义一致）。
#[inline]
pub(crate) fn cstring_slice(s: &[u8]) -> Constant {
  Constant::string(s.as_ptr(), s.len() as u32)
}

/// 由字符串切片构造字符串常量。
#[inline]
pub fn cstring_str(s: &str) -> Constant {
  cstring_slice(s.as_bytes())
}
