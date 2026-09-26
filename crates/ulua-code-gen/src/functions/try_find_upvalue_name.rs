use ulua_vm::records::proto::Proto;

use crate::{
  functions::proto_views::{name_str, upvalue_names},
  macros::codegen_assert::CODEGEN_ASSERT,
};

/// cpp `tryFindUpvalueName`：按上值下标取调试名。
///
/// 契约：`proto` 指向存活 Proto；返回的 `&str` 寿命随该借用（名字串随 Proto 可达）。
/// 下标越界时断言后按「无名」返回 `None`——cpp 此处是越界读，取安全方向。
pub(crate) fn try_find_upvalue_name(proto: &Proto, upval: i32) -> Option<&str> {
  // cpp 的短路次序：`upvalues != nullptr && upval >= 0` 才进入下标断言，
  // 空表（sizeupvalues == 0 或基址为空）不触发断言。
  if upval < 0 {
    return None;
  }
  let upvalues = upvalue_names(proto);
  if upvalues.is_empty() {
    return None;
  }

  CODEGEN_ASSERT!((upval as usize) < upvalues.len());
  name_str(*upvalues.get(upval as usize)?, proto)
}
