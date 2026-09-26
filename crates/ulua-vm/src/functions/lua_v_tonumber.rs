use core::ptr::from_ref;

use crate::{
  enums::value_view::ValueView,
  functions::{cstr_bytes, lua_o_str_2_d::lua_o_str_2_d},
  macros::{getstr::getstr, setnvalue::setnvalue},
  records::t_string::tstring,
  type_aliases::t_value::TValue,
};

/// `luaV_tonumber`：`obj` 可作数值时返回指向数值 TValue 的指针，否则 `None`。
///
/// `obj` 已是数值时直接返回其指针；字符串转出的数值写入 `scratch`
/// （对应 C++ 出参 `n`）后返回 `scratch`。
///
/// # Safety
/// `obj` 必须指向可读且对齐的 `TValue`；其为字符串时 payload 完整并以 NUL 结尾
/// （`CStr::from_ptr` 按 strlen 定读取界，失配即越读）。`scratch` 必须可写对齐，
/// 字符串转出成功后数值写入其中。返回指针别名 `obj` 或 `scratch` 之一，存活期由
/// 调用方对两者的借用管辖（对应 cpp 出参 `n`）。cpp lvmutils.cpp:25。
pub(crate) unsafe fn lua_v_tonumber(
  obj: *const TValue,
  scratch: &mut TValue,
) -> Option<*const TValue> {
  // Safety: 契约保证 `obj` 指向存活 TValue 且其串 payload 完整；数值解析只读取串数据界内字节，endptr 落在缓冲内
  unsafe {
    match ValueView::from_tvalue(&*obj) {
      ValueView::Number(_) => Some(obj),
      ValueView::String(ts) => {
        if let Some(num) = lua_o_str_2_d(cstr_bytes(getstr(ts as *const tstring))) {
          setnvalue!(&mut *scratch, num);
          return Some(from_ref(scratch));
        }
        None
      }
      _ => None,
    }
  }
}
