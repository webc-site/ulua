//! §11 pass C1：宏体收口为 `TValue::set_nvalue` 方法转发（语义与旧宏体逐位一致：
//! 先 `value.n` 后 `tt = LUA_TNUMBER`；`$obj`/`$x` 各求值一次）。宏体不自带
//! `unsafe` 块，unsafe 上下文由调用点提供。cpp `lobject.h:117`。
#[macro_export]
macro_rules! setnvalue {
  ($obj:expr, $x:expr) => {{ (*$obj).set_nvalue($x) }};
}

pub use setnvalue;
