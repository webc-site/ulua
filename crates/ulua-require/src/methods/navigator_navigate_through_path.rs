use crate::{
  functions::split_path::split_path,
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub fn navigate_through_path(&mut self, path: &str) -> Error {
    // 别名路径跳过首段别名：到别名的导航由调用方负责
    let (mut first, mut rest) = split_path(path);
    if path.starts_with('@') {
      (first, rest) = split_path(rest);
    }

    let mut previous_component: Option<&str> = None;
    while !(first.is_empty() && rest.is_empty()) {
      match first {
        // '.' 与空组件直接跳过
        "." | "" => {
          (first, rest) = split_path(rest);
          continue;
        }
        ".." => {
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
