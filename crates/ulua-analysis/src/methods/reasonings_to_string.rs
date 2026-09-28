use core::fmt;

use crate::records::reasonings::Reasonings;

impl fmt::Display for Reasonings {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.reasons.is_empty() {
      return Ok(());
    }

    let mut reasons = self.reasons.clone();
    reasons.sort();

    if reasons.len() < 2 {
      writeln!(f)?;
    } else {
      write!(f, "\nthis is because ")?;
    }

    for reason in &reasons {
      if reasons.len() > 1 {
        write!(f, "\n\t * ")?;
      }
      write!(f, "{}", reason)?;
    }

    Ok(())
  }
}
