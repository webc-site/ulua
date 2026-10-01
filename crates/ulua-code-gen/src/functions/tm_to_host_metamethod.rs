use ulua_vm::enums::tms::TMS;

use crate::{enums::host_metamethod::HostMetamethod, macros::codegen_assert::CODEGEN_ASSERT};

#[inline]
pub fn tm_to_host_metamethod(tm: i32) -> HostMetamethod {
  // cpp `TMS(tm)` 是前置条件式强转：tm 由编译器写入 IR，必为有效元方法判别值。
  // 这里改用 checked 辅助（仿 LuauBuiltinFunction::from_id），越界输入显式断言，
  // 不再依赖裸 transmute 的未定义行为。
  let Some(tms) = TMS::from_u32(u32::try_from(tm).unwrap_or(u32::MAX)) else {
    CODEGEN_ASSERT!(
      false,
      "tm_to_host_metamethod: tm outside valid TMS discriminant range"
    );
    return HostMetamethod::Add;
  };

  match tms {
    TMS::TmAdd => HostMetamethod::Add,
    TMS::TmSub => HostMetamethod::Sub,
    TMS::TmMul => HostMetamethod::Mul,
    TMS::TmDiv => HostMetamethod::Div,
    TMS::TmIDiv => HostMetamethod::Idiv,
    TMS::TmMod => HostMetamethod::Mod,
    TMS::TmPow => HostMetamethod::Pow,
    TMS::TmUnm => HostMetamethod::Minus,
    TMS::TmEq => HostMetamethod::Equal,
    TMS::TmLt => HostMetamethod::LessThan,
    TMS::TmLe => HostMetamethod::LessEqual,
    TMS::TmLen => HostMetamethod::Length,
    TMS::TmConcat => HostMetamethod::Concat,
    _ => {
      // 合法 TMS 判别值但无对应宿主元方法（如 TmCall/TmIndex）：与 cpp 默认分支一致，
      // 由调用方保证只对算术/比较/长度/拼接类元方法调用本函数
      HostMetamethod::Add
    }
  }
}
