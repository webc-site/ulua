use core::str::from_utf8;

use ulua_common::functions::c_str::cstr_bytes;
use ulua_vm::{enums::lua_type::LuaType, macros::getstr::getstr, records::proto::Proto};

/// `Proto::k` 字符串常量名读取收口（安全边界门面）：`k[index]` 为 UTF-8 可解码的
/// Lua string 常量时，把其 `&str` 名交给 `with` 消费并返回结果；否则返回 `None`。
/// `proto`/`k` 为空同样返回 `None`（调用侧保留 cpp 的判空短路语义）。
/// 裸指针解引用统一收口在本函数内：`proto` 源自 codegen 入参、生命周期覆盖整次
/// 编译，`index` 为 IR VmConst 操作数携带的合法常量下标（codegen 依 `sizek` 生成），
/// 单线程串行、只读、借用不出块，`&str` 仅在 `with` 调用期内存活。
/// 纯字节视图（宿主 hook 的 C-ABI 物化）走 `proto_views::string_constant`。
pub(crate) fn with_constant_string_name<T>(
  proto: *mut Proto,
  index: u32,
  with: impl FnOnce(&str) -> T,
) -> Option<T> {
  if proto.is_null() {
    return None;
  }

  // Safety: 见函数注释——proto 活、k 界内、tt 已判为 String，tsvalue!/getstr/cstr_bytes
  // 只读存活 TString 的 NUL 结尾字节。
  unsafe {
    let constants = (*proto).k;
    if constants.is_null() {
      return None;
    }

    let value = constants.add(index as usize);
    if (*value).tt != LuaType::String as i32 {
      return None;
    }

    from_utf8(cstr_bytes(getstr((*value).as_string())))
      .ok()
      .map(with)
  }
}

/// `Proto::k` vector 常量分量读取收口（安全边界门面）：`k[index]` 为 vector 常量时
/// 返回第 `component`（<4 由调用方断言）个分量；proto/k 为空或非 vector 返回 `None`。
/// 收口契约与 `with_constant_string_name` 同源：`index` 为合法 VmConst 下标，
/// `(*tv).is_vector()` 判型后 `as_vector()` 读 union vector 臂为位有效，单线程只读。
pub(crate) fn proto_constant_vector_component(
  proto: *mut Proto,
  index: u32,
  component: usize,
) -> Option<f64> {
  if proto.is_null() {
    return None;
  }

  // Safety: 见函数注释。
  unsafe {
    let constants = (*proto).k;
    if constants.is_null() {
      return None;
    }

    let tv = constants.add(index as usize);
    if (*tv).is_vector() {
      Some((*tv).as_vector()[component] as f64)
    } else {
      None
    }
  }
}
