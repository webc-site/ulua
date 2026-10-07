use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::c_const::{cstring_str, cvar},
  records::constant::Constant,
};

/// C++ `type/typeof` 折叠共享骨架（原 ctype/cttypeof 两函数除 vector 一臂外全同构，
/// 双臂合并为单条路径，仅按 `typeof_like` 旗标分流 vector）：
/// typeof 语义下 vector 的运行时类型名可由宿主自定义，编译期不可折叠（返回
/// Unknown 落 cvar）；type 语义下直载 "vector"。其余分量两语义逐字一致，
/// 两级诊断（前置 `!c.is_unknown()` 与未知类型兜底）文案与触发次序不变。
pub(crate) fn constant_type_name(c: &Constant, typeof_like: bool) -> Constant {
  LUAU_ASSERT!(!c.is_unknown());

  match c {
    Constant::Nil => cstring_str("nil"),
    Constant::Boolean(_) => cstring_str("boolean"),
    Constant::Number(_) => cstring_str("number"),
    Constant::Integer(_) => cstring_str("integer"),
    Constant::Vector(_) => {
      if typeof_like {
        cvar() // vector can have a custom typeof name at runtime
      } else {
        cstring_str("vector")
      }
    }
    Constant::Str(_) => cstring_str("string"),
    // 仅 Unknown / Table 会到达
    _ => {
      LUAU_ASSERT!(false);
      cvar()
    }
  }
}
