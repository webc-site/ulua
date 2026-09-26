//! Source: `VM/src/lobject.h:141-150`（inline float 分支）
//!
//! 第 4 分量只在 `LUA_VECTOR_SIZE == 4` 时存在。上游用
//! `condvector4(i_v[3] = float(w), (void)(w))`（`VM/src/lobject.h:148`，宏体见
//! `VM/src/lobject.h:53-57`）：4-lane 写第 4 槽，3-lane 只把 `w` 丢弃、不写
//! `i_v[3]`。
//!
//! 这里把 `$w` 整个表达式放进 `if LUA_VECTOR_SIZE == 4` 分支内展开，比 cpp 更严：
//! 连 `w` 都不求值。因为调用点形如
//! `setvvalue!(ra, .., *vb.add(3) + *vc.add(3))`，而 `vb = vvalue!(..).as_ptr()`
//! 的 provenance 只有 `&[f32; LUA_VECTOR_SIZE]`；3-lane 下 `.add(3)` 会越过该切片
//! 尾部读取，在 Rust 里是 UB（cpp 读 padding 无此问题，故只 `(void)(w)`）。
//! `LUA_VECTOR_SIZE` 是编译期常量，false 分支连同其中的第 4 分量读取一起被消除，
//! 零开销。
//!
//! §11 pass C1：宏体收口为 `TValue::set_vvalue` 方法转发，上述 3-lane UB 防御
//! 原样保留——`$w` 以闭包 `|| $w` 惰性传入，只在方法内 `LUA_VECTOR_SIZE == 4`
//! 门内被调用，3-lane 下依旧连 `w` 都不求值；其余写入次序（lane0/1/2 顺次、
//! 门内 lane3、最后 tt）与旧宏体逐位一致，`$obj`/`$x`/`$y`/`$z` 各求值一次。
//! 宏体不自带 `unsafe` 块，unsafe 上下文由调用点提供。
#[macro_export]
macro_rules! setvvalue {
  ($obj:expr, $x:expr, $y:expr, $z:expr, $w:expr) => {{
    let i_o: *mut $crate::type_aliases::t_value::TValue = $obj;
    (*i_o).set_vvalue($x, $y, $z, || $w)
  }};
}

pub use setvvalue;
