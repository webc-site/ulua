#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReportFormat {
  #[default]
  Default,
  Luacheck,
  Gnu,
}
