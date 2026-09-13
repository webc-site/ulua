use crate::records::team_city_reporter::TeamCityReporter;

impl TeamCityReporter {
  pub fn test_case_skipped(&mut self, in_test_suite: &str, in_name: &str) {
    println!(
      "##teamcity[testIgnored name='{}: {}' captureStandardOutput='false']",
      in_test_suite, in_name
    );
  }
}
