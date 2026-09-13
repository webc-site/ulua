use crate::records::boost_like_reporter::BoostLikeReporter;
impl BoostLikeReporter {
  pub fn log_message(&mut self, md_severity: i32, md_file: &str, md_line: i32, md_string: &str) {
    let severity = if (md_severity & 1) != 0 {
      "WARNING"
    } else {
      "ERROR"
    };
    // Matches doctest's behavior: "<file>(<line>): <severity>: <message>\n"
    println!("{}({}): {}: {}", md_file, md_line, severity, md_string);
  }
}
