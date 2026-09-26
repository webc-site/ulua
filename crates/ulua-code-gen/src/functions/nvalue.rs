use core::ptr::from_mut;

use ulua_vm::{macros::setnvalue::setnvalue, type_aliases::t_value::TValue};

/// 就地构造一个 number `TValue`（cpp 慢路径 `TValue n; setnvalue(&n, v);` 的定值形）。
///
/// 根因在 `ulua_vm::TValue` 的 union 载荷写入是 `unsafe`（`set_nvalue`，§11 的
/// `enum Value` 路线落地前不可再收），此处把"写局部联合值"这一最小需求收口为
/// 唯一封装点：返回按值所有，调用点无裸指针、无 unsafe、无悬垂窗口。
#[inline]
pub(crate) fn nvalue(n: f64) -> TValue {
  let mut v = TValue::default();
  // Safety: `v` 为活局部；`set_nvalue` 只写 `value.n` union 域与 `tt` 标签，
  // 该值其后仅按 number 读取。
  unsafe { setnvalue!(from_mut(&mut v), n) };
  v
}
