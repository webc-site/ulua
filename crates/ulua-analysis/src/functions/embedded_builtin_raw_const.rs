extern crate alloc;

const EMBEDDED_BUILTINS_CPP: &str = include_str!("embedded_builtin_definitions.cpp");

pub fn embedded_builtin_raw_const(name: &str) -> &'static str {
  let needle = alloc::format!("{name} = R\"");
  let start = EMBEDDED_BUILTINS_CPP
    .find(&needle)
    .unwrap_or_else(|| panic!("missing embedded builtin definition {name}"))
    + needle.len();
  let rest = &EMBEDDED_BUILTINS_CPP[start..];
  let tag_end = rest
    .find('(')
    .unwrap_or_else(|| panic!("malformed embedded builtin definition {name}"));
  let tag = &rest[..tag_end];
  let body_start = start + tag_end + 1;
  let end_marker = alloc::format!("){tag}\"");
  let body_end = EMBEDDED_BUILTINS_CPP[body_start..]
    .find(&end_marker)
    .unwrap_or_else(|| panic!("unterminated embedded builtin definition {name}"));

  &EMBEDDED_BUILTINS_CPP[body_start..body_start + body_end]
}
