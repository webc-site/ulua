use ulua_ast::records::location::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Statement {
  pub(crate) start: Location,
  pub(crate) last_line: u32,
  pub(crate) flagged: bool,
}

impl Statement {
  pub const fn last_line(&self) -> u32 {
    self.last_line
  }
}
