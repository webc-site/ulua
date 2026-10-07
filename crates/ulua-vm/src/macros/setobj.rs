//! §11 pass C2：宏体收口为 `TValue::set_obj` 方法转发（C 族唯一核心壳；
//! `setobj_2_s`/`setobj2t`/`setobj2n`/`setobjt2t`/`setobj2class` 薄壳继续转发本
//! 宏，形成「宏壳→TValue 方法」两级，仿 C1 既有薄壳链）。语义与旧宏体逐位一致：
//! `*o1 = *o2` 整槽复制收口为 `ptr::copy`（memmove，容许同址/重叠，cpp 同形），
//! 随后 `checkliveness!((*$l).global, o1)`；`$obj1`/`$obj2`/`$l` 求值次数与原宏
//! 一致（各一次，o2 先于 o1 绑定次序不变）。`(*$l).global` 为纯指针字段加载，
//! 提前于复制求值不改变可观察行为。屏障不在宏内触发（cpp 中 SETOBJ 与
//! luaC_barriert/back 分步、调用点自持时序）。宏体不自带 `unsafe` 块，unsafe
//! 上下文由调用点提供。cpp `lobject.h:232`。
#[macro_export]
macro_rules! setobj {
  ($l:expr, $obj1:expr, $obj2:expr) => {{
    let o2: *const $crate::type_aliases::t_value::TValue = $obj2;
    let o1: *mut $crate::type_aliases::t_value::TValue = $obj1;
    (*o1).set_obj(o2, (*$l).global);
  }};
}

pub use setobj;
