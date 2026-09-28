//! §11 pass C1：宏体收口为 `TValue::set_nil` 方法转发（语义与旧宏体逐位一致：
//! 仅写 `tt = LUA_TNIL`，`$obj` 求值一次）。宏体不自带 `unsafe` 块——裸指针解引用
//! 的 unsafe 上下文仍由调用点提供（与旧宏一致，零调用点 churn）。cpp `lobject.h:115`。
#[macro_export]
macro_rules! setnilvalue {
  ($obj:expr) => {
    (*$obj).set_nil()
  };
}

pub use setnilvalue;
