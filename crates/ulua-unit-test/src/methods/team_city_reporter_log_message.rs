use crate::records::team_city_reporter::TeamCityReporter;
impl TeamCityReporter {
  pub fn log_message(&mut self, md_severity: i32, md_file: &str, md_line: i32, md_string: &str) {
    let is_warn = (md_severity & 1) != 0;
    let severity = if is_warn { "WARNING" } else { "ERROR" };

    let is_error = (md_severity & (2 | 4)) != 0;

    if is_error {
      eprintln!("{}({}): {}: {}", md_file, md_line, severity, md_string);
    } else {
      println!("{}({}): {}: {}", md_file, md_line, severity, md_string);
    }
  }
}
