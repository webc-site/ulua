use crate::{
  functions::{get_path_type::ALIAS_PREFIX, split_path::split_path},
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub(crate) fn navigate_through_path(&mut self, path: &[u8]) -> Error {
    // 别名路径跳过首段别名：到别名的导航由调用方负责
    let (mut first, mut rest) = split_path(path);
    if path.first() == Some(&ALIAS_PREFIX) {
      (first, rest) = split_path(rest);
    }

    // 组件按字节零拷贝借用原路径，非 UTF-8 字节原样传给导航上下文
    let mut previous_component: Option<&[u8]> = None;
    while !(first.is_empty() && rest.is_empty()) {
      match first {
        // '.' 与空组件直接跳过
        b"." | b"" => {
          (first, rest) = split_path(rest);
          continue;
        }
        b".." => {
          if let Some(error) = self.navigate_to_parent(previous_component) {
            return Some(error);
          }
        }
        component => {
          if let Some(error) = self.navigate_to_child(component) {
            return Some(error);
          }
        }
      }
      previous_component = Some(first);
      (first, rest) = split_path(rest);
    }

    None
  }
}
