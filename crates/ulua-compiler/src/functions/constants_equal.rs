use ulua_common::{fflag::LuauIntegerType2, macros::luau_assert::LUAU_ASSERT};

use crate::records::constant::{Constant, ConstantStr};

/// 字符串按内容比较（同指针直接判定相等，跳过逐字节扫描）
fn str_eq(a: &ConstantStr, b: &ConstantStr) -> bool {
  a.len == b.len && (a.ptr == b.ptr || a.bytes() == b.bytes())
}

pub fn constants_equal(la: &Constant, ra: &Constant) -> bool {
  LUAU_ASSERT!(!la.is_unknown() && !ra.is_unknown());

  match (la, ra) {
    (Constant::Nil, Constant::Nil) => true,
    (Constant::Boolean(a), Constant::Boolean(b)) => a == b,
    (Constant::Number(a), Constant::Number(b)) => a == b,
    (Constant::Vector(a), Constant::Vector(b)) => a == b,
    (Constant::Str(a), Constant::Str(b)) => str_eq(a, b),
    (Constant::Table(a), Constant::Table(b)) => a == b,
    (Constant::Integer(a), Constant::Integer(b)) => {
      if LuauIntegerType2.get() {
        a == b
      } else {
        LUAU_ASSERT!(false);
        false
      }
    }
    // 同类型分支已全部覆盖；此处仅剩跨类型（不等）
    _ => false,
  }
}
