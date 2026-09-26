#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestResult {
  BugFound, // We encountered the bug we are trying to isolate
  NoBug,    // We did not encounter the bug we are trying to isolate
}
