use alloc::string::String;

use crate::records::alias_cycle_tracker::AliasCycleTracker;

impl AliasCycleTracker {
  pub(crate) fn get_stringified_cycle(&self, repeated: &str) -> String {
    let mut result = String::new();
    let mut in_cycle = false;
    for item in &self.ordered {
      if in_cycle {
        result.push_str(" -> ");
        result.push('@');
        result.push_str(item);
      }
      if item == repeated {
        in_cycle = true;
        result.push('@');
        result.push_str(item);
      }
    }
    result.push_str(" -> @");
    result.push_str(repeated);
    result
  }
}
