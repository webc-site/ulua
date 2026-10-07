#[macro_export]
macro_rules! equalobj {
  ($l:expr, $o1:expr, $o2:expr) => {
    ($crate::macros::ttype::ttype!($o1) == $crate::macros::ttype::ttype!($o2)
      // B2-1：`lua_v_equalval` 读侧收口为共享引用，本宏在边界处由操作数指针重建
      // 只读借用（解引用窗口止于本调用，unsafe 上下文与前提由展开点提供）
      && $crate::functions::lua_v_equalval::lua_v_equalval($l, &*$o1, &*$o2) != 0)
  };
}

pub use equalobj;
