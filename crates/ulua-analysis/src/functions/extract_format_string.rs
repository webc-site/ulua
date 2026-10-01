use core::str::from_utf8;

use ulua_ast::records::ast_expr_constant_string::AstExprConstantString;

use crate::{
  functions::{follow_type, get_type},
  records::{
    singleton_type::SingletonType, string_singleton::StringSingleton,
    type_pack_iterator::TypePackIterator,
  },
};

/// `string.format` 两处 magic（`magic_format_infer` / `magic_format_type_check`）
/// 逐字孪生的格式串提取段单源：优先取已甄别的常量字符串节点 `fmt`，
/// 否则回落到迭代器首参经 follow 后的字符串单例；两路均不可得时返回 `None`。
/// 返回生命周期随入参 `fmt` 借用（单例路为 `'static`，可协变收窄）。
pub(crate) fn extract_format_string<'a>(
  fmt: Option<&'a AstExprConstantString>,
  iter: &TypePackIterator,
) -> Option<&'a str> {
  let mut format_string: Option<&'a str> = None;

  if let Some(fmt_ref) = fmt {
    // Safety: `fmt` 由调用点 try_as_ptr 同型甄别命中而来，指向存活的
    // AstExprConstantString；`value` 字节数组随节点存活于 arena，
    // `as_bytes()` 只读该区域后转 `&str`。
    format_string = from_utf8(fmt_ref.value.as_bytes()).ok();
  } else {
    let first_arg = *iter.current();
    let followed = follow_type::follow(first_arg);
    if let Some(singleton) = get_type::get::<SingletonType>(followed)
      && let Some(string_singleton) = singleton.variant.get_if::<StringSingleton>()
    {
      format_string = Some(&string_singleton.value);
    }
  }

  format_string
}
