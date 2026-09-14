use std::io::stdout;

use crate::{
  methods::team_city_reporter_test_case_end::{
    doctest as doctest_crate, records::test_case_data as test_case_data_crate,
  },
  records::team_city_reporter::TeamCityReporter,
};
extern crate ulua_common;

use core::ffi::CStr;
use std::io::Write;

impl TeamCityReporter {
  pub fn test_case_end(&mut self, in_stats: &doctest_crate::CurrentTestCaseStats) {
    // Mirror the C++ behavior: print 3 TeamCity attributes and optionally a failed event.
    // The TestCaseData record is defined locally in this module to satisfy the pointer cast.
    let tc = self.current_test as *const test_case_data_crate::TestCaseData;
    unsafe {
      let suite = CStr::from_ptr((*tc).m_test_suite).to_string_lossy();
      let name = CStr::from_ptr((*tc).m_name).to_string_lossy();

      println!(
        "##teamcity[testMetadata testName='{}: {}' name='total_asserts' type='number' value='{}']",
        suite, name, in_stats.num_asserts_current_test
      );
      println!(
        "##teamcity[testMetadata testName='{}: {}' name='failed_asserts' type='number' value='{}']",
        suite, name, in_stats.num_asserts_failed_current_test
      );
      println!(
        "##teamcity[testMetadata testName='{}: {}' name='runtime' type='number' value='{}']",
        suite, name, in_stats.seconds
      );

      if !in_stats.test_case_success {
        println!("##teamcity[testFailed name='{}: {}']", suite, name);
      }

      println!("##teamcity[testFinished name='{}: {}']", suite, name);
      let _ = stdout().flush();
    }
  }
}

pub mod doctest {
  #[repr(C)]
  pub struct CurrentTestCaseStats {
    pub num_asserts_current_test: i32,
    pub num_asserts_failed_current_test: i32,
    pub seconds: f64,
    pub test_case_success: bool,
  }
}

pub mod records {
  pub mod test_case_data {
    use core::ffi::c_char;

    #[repr(C)]
    pub struct TestCaseData {
      pub m_test_suite: *const c_char,
      pub m_name: *const c_char,
    }
  }
}
