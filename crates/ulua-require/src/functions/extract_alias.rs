use crate::functions::{get_path_type::ALIAS_PREFIX, split_path::PATH_SEPARATOR};

/// 取 `@` 之后的别名段（到首个 `/` 或串尾）。对应 cpp `extractAlias`：
/// 纯字节切分，字节数与原串一致，不做任何 UTF-8 校验。
pub(crate) fn extract_alias(path: &[u8]) -> &[u8] {
  let rest = if path.first() == Some(&ALIAS_PREFIX) {
    &path[1..]
  } else {
    path
  };

  match rest.iter().position(|&b| b == PATH_SEPARATOR) {
    Some(pos) => &rest[..pos],
    None => rest,
  }
}
