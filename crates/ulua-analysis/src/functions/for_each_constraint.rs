use crate::records::{
  checkpoint::Checkpoint, constraint::Constraint, constraint_generator::ConstraintGenerator,
};
pub fn for_each_constraint<F>(
  start: Checkpoint,
  end: Checkpoint,
  cg: &ConstraintGenerator,
  mut f: F,
) where
  F: FnMut(*mut Constraint),
{
  for &c in &cg.constraints[start.offset..end.offset] {
    f(c);
  }
}
