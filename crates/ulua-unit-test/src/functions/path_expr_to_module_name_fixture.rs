use alloc::{string::String, vec::Vec};

pub fn path_expr_to_module_name_module_name_vector_string_view(
  current_module_name: &str,
  segments: &Vec<&str>,
) -> Option<String> {
  if segments.is_empty() {
    return None;
  }

  let mut result: Vec<String> = Vec::new();
  let mut it = segments.iter();

  if let Some(&first) = it.next() {
    if first == "script" && !current_module_name.is_empty() {
      for segment in current_module_name.split('/') {
        result.push(segment.to_string());
      }
    } else {
      result.push(first.to_string());
    }
  }

  for &segment in it {
    if result.len() > 1 && segment == "Parent" {
      result.pop();
    } else {
      result.push(segment.to_string());
    }
  }

  let mut joined = String::new();
  for (i, segment) in result.iter().enumerate() {
    if i > 0 {
      joined.push('/');
    }
    joined.push_str(segment);
  }

  Some(joined)
}
