//! §11 pass C1：宏体收口为 `TValue::set_bvalue` 方法转发（语义与旧宏体逐位一致：
//! 先 `value.b` 后 `tt = LUA_TBOOLEAN`；`$obj` 求值一次；`($b) as i32` 收敛保留在
//! 宏体，兼容 bool/整型实参——cpp `lobject.h:161` 的 `i_o->value.b = (x)` 同形，
//! Lua 侧 true 恒存 1）。宏体不自带 `unsafe` 块，unsafe 上下文由调用点提供。
#[macro_export]
macro_rules! setbvalue {
  ($obj:expr, $b:expr) => {{ (*$obj).set_bvalue(($b) as i32) }};
}

pub use setbvalue;
