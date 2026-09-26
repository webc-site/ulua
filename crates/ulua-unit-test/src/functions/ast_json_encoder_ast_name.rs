use ulua_ast::records::ast_name::AstName;

pub fn ast_name(value: &'static [u8]) -> AstName {
  let trimmed = match value.strip_suffix(b"\0") {
    Some(s) => s,
    None => value,
  };
  AstName::from_static(trimmed)
}
