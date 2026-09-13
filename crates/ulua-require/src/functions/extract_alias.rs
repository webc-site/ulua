pub(crate) fn extract_alias(path: &str) -> &str {
  let rest = path.strip_prefix('@').unwrap_or(path);
  match rest.find('/') {
    Some(pos) => &rest[..pos],
    None => rest,
  }
}
