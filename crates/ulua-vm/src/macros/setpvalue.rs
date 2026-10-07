//! §11 pass C1：宏体收口为 `TValue::set_pvalue` 方法转发（语义与旧宏体逐位一致：
//! 先 `value.p`、再 `extra[0]`、最后 `set_tt(LUA_TLIGHTUSERDATA)`；`$obj`/`$x`/`$tag`
//! 各求值一次）。宏体不自带 `unsafe` 块，unsafe 上下文由调用点提供。
//! cpp `lobject.h:153`。
#[macro_export]
macro_rules! setpvalue {
  ($obj:expr, $x:expr, $tag:expr) => {{ (*$obj).set_pvalue($x, $tag) }};
}

pub use setpvalue;
