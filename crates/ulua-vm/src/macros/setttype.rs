//! §11 pass C1：宏体收口为槽自身的 `set_tt` 方法调用（语义与旧宏体逐位一致：
//! 仅 set_tt，`$obj`/`$tt` 各求值一次；宏体不自带 `unsafe` 块，unsafe 上下文由
//! 调用点提供）。收口方法复用 `lua_TValue::set_tt` / `TKey::set_tt`
//! （`setttype!` 唯一消费点作用于 `TKey`，两型各有一份 `set_tt`，宏保持对接收者
//! 类型的泛化）。cpp `lobject.h:272`。
#[macro_export]
macro_rules! setttype {
  ($obj:expr, $tt:expr) => {
    (*$obj).set_tt($tt)
  };
}

pub use setttype;
