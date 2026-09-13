use ulua_common::records::variant::Variant2;

use crate::{records::dcr_logger::DcrLogger, type_aliases::step_snapshot::StepSnapshot};

impl DcrLogger {
  pub fn commit_step_snapshot(&mut self, snapshot: StepSnapshot) {
    if let Variant2::V1(eg) = &snapshot
      && eg.before == eg.after
    {
      return;
    }

    self.solve_log.step_states.push(snapshot);
  }
}
