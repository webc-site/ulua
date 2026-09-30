#[derive(Debug, Clone)]
pub struct TarjanWorklistVertex {
  pub(crate) index: i32,
  pub(crate) curr_edge: i32,
  pub(crate) last_edge: i32,
}
