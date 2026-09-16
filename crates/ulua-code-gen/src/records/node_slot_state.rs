#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct NodeSlotState {
  pub pointer: u32,
  pub known_to_not_be_nil: bool,
}
