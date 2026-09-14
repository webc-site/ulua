use ulua_ast::records::location::Location;

pub fn report(name: &str, location: &Location, r#type: &str, message: &str) {
  eprintln!(
    "{}({},{}): {}: {}",
    name,
    location.begin.line + 1,
    location.begin.column + 1,
    r#type,
    message
  );
}
