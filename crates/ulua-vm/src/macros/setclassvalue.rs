//! §11 pass C2：宏体收口为 `TValue::set_classvalue` 方法转发（语义与旧宏体逐位
//! 一致：先 `value.gc`、后 `tt = LUA_TCLASS`、再 `checkliveness!((*$l).global,
//! i_o)`；`$obj`/`$x`/`$l` 各求值一次，`as *mut TValue` 收敛与 `as *mut GCObject` 抹除
//! 保留在宏体）。`(*$l).global` 为纯指针字段加载，先于载荷写入求值不改变可观察
//! 行为。屏障不在宏内触发。宏体不自带 `unsafe` 块，unsafe 上下文由调用点提供。
//! cpp `lobject.h:240`。
#[macro_export]
macro_rules! setclassvalue {
  ($l:expr, $obj:expr, $x:expr) => {{
    let i_o = $obj as *mut $crate::type_aliases::t_value::TValue;
    (*i_o).set_classvalue(
      ($x) as *mut $crate::records::gc_object::GCObject,
      (*$l).global,
    );
  }};
}

pub use setclassvalue;
