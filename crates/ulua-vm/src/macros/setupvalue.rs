//! §11 pass C2：宏体收口为 `TValue::set_upvalue` 方法转发（语义与旧宏体逐位一致：
//! 先 `value.gc`、后 `tt = LUA_TUPVAL`、再 `checkliveness!((*$l).global, i_o)`；
//! `$obj`/`$x`/`$l` 各求值一次，`as *mut GCObject` 抹除保留在宏体）。`(*$l).global` 为纯
//! 指针字段加载，先于载荷写入求值不改变可观察行为。屏障不在宏内触发（cpp 屏障是
//! 调用点分步动作）。r17-c：方法壳转 safe（纯字段写），debug-only 的 `checkliveness` 存活断言由方法内下沉至本宏壳、紧跟写入，次序与旧宏体逐位一致；`(*$l).global` 由断言路径解引用，与宏体其余裸解引用同处调用点 unsafe 语境。 宏体不自带 `unsafe` 块，unsafe 上下文由调用点提供。
//! cpp `lobject.h:224`。
#[macro_export]
macro_rules! setupvalue {
  ($l:expr, $obj:expr, $x:expr) => {{
    let i_o: *mut $crate::type_aliases::t_value::TValue = $obj;
    (*i_o).set_upvalue(($x) as *mut $crate::records::gc_object::GCObject);
    $crate::checkliveness!((*$l).global, i_o);
  }};
}

pub use setupvalue;
