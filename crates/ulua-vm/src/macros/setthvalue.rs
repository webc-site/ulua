//! §11 pass C2：宏体收口为 `TValue::set_thvalue` 方法转发（语义与旧宏体逐位一致：
//! 先 `value.gc`、后 `tt = LUA_TTHREAD`、再 `checkliveness!((*$l).global, i_o)`；
//! `$obj`/`$x`/`$l` 各求值一次，`as *mut TValue` 收敛与 `as *mut GCObject` 抹除保留在宏
//! 体）。`(*$l).global` 为纯指针字段加载，先于载荷写入求值不改变可观察行为。
//! 屏障不在宏内触发。宏体不自带 `unsafe` 块，unsafe 上下文由调用点提供。
//! cpp `lobject.h:184`。
#[macro_export]
macro_rules! setthvalue {
  ($l:expr, $obj:expr, $x:expr) => {{
    let i_o = $obj as *mut $crate::type_aliases::t_value::TValue;
    (*i_o).set_thvalue(
      ($x) as *mut $crate::records::gc_object::GCObject,
      (*$l).global,
    );
  }};
}

pub use setthvalue;
