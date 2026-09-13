#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Context {
  CovariantContext,
  InvariantContext,
}

impl Context {
  pub const COVARIANT: Context = Context::CovariantContext;
  pub const INVARIANT: Context = Context::InvariantContext;
}
