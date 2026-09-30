//! `Proto` 边界只读访问门面。
//!
//! Source: `CodeGen/src/IrTranslation.cpp` 各 `proto->k[...]`/`proto->p[...]` 直读点
//! （行为 oracle），实现按 review §2 收口：`Proto` 裸指针、`TValue` 联合体臂的散点
//! `unsafe` 解引用统一进本文件，业务侧只见安全闭包与类型化读数。

use ulua_vm::{records::proto::Proto, type_aliases::t_value::TValue};

/// 以只读共享视图消费 `Proto`：`proto` 为空短路为 `None`。
///
/// # Safety 契约（调用点通用）
/// `build.function.proto` 由 IrBuilder 构造期接线，非空时指向存活 `Proto`
/// （生命周期覆盖整次 codegen）；单线程串行、闭包只读、借用不出语句。
/// 原散点点形解引用在断言保证非空后直接 `(*proto).field`，本门面对 null 改为
/// 显式 `None` 短路（生产中不可达，仅把原 UB 分支转为可判读的空值）。
pub(crate) fn with_proto<T>(proto: *mut Proto, with: impl FnOnce(&Proto) -> T) -> Option<T> {
  if proto.is_null() {
    return None;
  }

  // Safety: 见契约——proto 非空即活 Proto，共享只读借用无别名冲突。
  Some(unsafe { with(&*proto) })
}

/// 以只读 TValue 视图消费 `k[index]` 常量：`with` 闭包读取 `tt` 与联合体臂，
/// 判定顺序由调用点保留，与原散点读取逐项一致。界内前提同 `with_proto`：
/// `index` 为 codegen 依 `sizek` 写定的合法常量下标；`k` 基址空值短路为 `None`。
pub(crate) fn with_constant_value<T>(
  proto: *mut Proto,
  index: u32,
  with: impl FnOnce(&TValue) -> T,
) -> Option<T> {
  with_proto(proto, |proto| {
    let constants = proto.k;
    if constants.is_null() {
      return None;
    }

    // Safety: index < sizek 由调用侧契约保证，`constants.add(index)` 落在数组界内、
    // TValue 对齐；`&*` 只读借用不出本语句，闭包对 Copy 字段读取与原散点逐字节一致。
    Some(unsafe { with(&*constants.add(index as usize)) })
  })?
}

/// 读出 `p[index]` 子原型指针：`proto` 为空短路为 `None`。
/// 界内前提同 `with_proto`：`index < sizep` 由调用侧 `CODEGEN_ASSERT` 契约保证。
pub(crate) fn child_proto(proto: *mut Proto, index: u32) -> Option<*mut Proto> {
  with_proto(proto, |proto| {
    // Safety: index < sizep 由调用侧契约保证，`p.add(index)` 落在子原型数组界内、
    // 指针字对齐，只读拷贝出一个 `*mut Proto` 值、不产生引用。
    unsafe { *proto.p.add(index as usize) }
  })
}

/// 读 `Number`（f64）臂：不复核 tt，与原散点读取一致——活跃臂由调用方所在
/// 分发路径的 opcode 选择/`CODEGEN_ASSERT` 契约保证。unsafe 收口在本函数一行内。
#[inline]
pub(crate) fn constant_number(tv: &TValue) -> f64 {
  // Safety: 契约见 `with_constant_value` 调用侧；`tt == Number` 时 `.n` 为存活臂。
  unsafe { tv.value.n }
}

/// 读 `Integer`（i64）臂，契约同 `constant_number`。
#[inline]
pub(crate) fn constant_integer(tv: &TValue) -> i64 {
  // Safety: 同 `constant_number`——`tt == Integer` 时 `.l` 为存活臂。
  unsafe { tv.value.l }
}

/// 读 `Boolean`（i32）臂，契约同 `constant_number`。
#[inline]
pub(crate) fn constant_boolean(tv: &TValue) -> i32 {
  // Safety: 同 `constant_number`——`tt == Boolean` 时 `.b` 为存活臂。
  unsafe { tv.value.b }
}
