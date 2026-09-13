#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IdfVisitMarks {
  pub seen_in_queue: bool,
  pub seen_in_worklist: bool,
}
