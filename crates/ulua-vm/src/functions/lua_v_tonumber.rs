use crate::{
  enums::value_view::ValueView,
  functions::{cstr_bytes, lua_o_str_2_d::lua_o_str_2_d},
  macros::{getstr::getstr, setnvalue::setnvalue},
  records::t_string::tstring,
  type_aliases::t_value::TValue,
};

/// `luaV_tonumber`：`obj` 可作数值时返回指向数值 `TValue` 的共享引用，否则 `None`。
///
/// B2-1 收口（裁决 2：只读侧参数用 `&TValue`）：入参 `obj` 由裸 `*const TValue` 折叠为
/// 共享引用，返回由 `Option<*const TValue>` 折叠为 `Option<&'a TValue>`——命中原槽时
/// 别名 `obj`，字符串转出时别名 `scratch`（对应 C++ 出参 `n`），存活期由借用检查锚定。
/// `obj` 已是数值时直接返回其引用；字符串转出的数值写入 `scratch` 后返回 `scratch`。
///
/// 同址就地改写场景（cpp `luaV_tonumber(p, p)`）不再以别名裸指针表达：调用侧先转本地
/// `scratch`、成功后写回槽（见 `lua_v_prepare_forn`），值语义逐位一致。
///
/// # Safety
/// `obj` 为字符串时其 payload 完整并以 NUL 结尾（按 strlen 定读取界，失配即越读）——
/// 引用本身保证可读与对齐，该 payload 不变量仍由调用方管辖。字符串转出成功后数值写入
/// `scratch`。cpp lvmutils.cpp:25。
pub(crate) unsafe fn lua_v_tonumber<'a>(
  obj: &'a TValue,
  scratch: &'a mut TValue,
) -> Option<&'a TValue> {
  match ValueView::from_tvalue(obj) {
    ValueView::Number(_) => Some(obj),
    ValueView::String(ts) => {
      // SAFETY: 契约保证 `obj` 为字符串时其 payload 完整且以 NUL 结尾；
      // 数值解析只读取串数据界内字节，endptr 落在缓冲内
      if let Some(num) = unsafe { lua_o_str_2_d(cstr_bytes(getstr(ts as *const tstring))) } {
        setnvalue!(scratch, num);
        return Some(&*scratch);
      }
      None
    }
    _ => None,
  }
}
