use std::{
  io::{BufRead, BufReader},
  process::{Command, Stdio},
};

use crate::{
  enums::test_result::TestResult, methods::reducer_escape::reducer_escape,
  records::reducer::Reducer,
};

impl Reducer {
  pub fn run(&mut self) -> TestResult {
    self.write_temp_script(false);

    let escaped_script_name = reducer_escape(self, &self.script_name);
    let mut cmd = self.command.clone();
    while let Some(pos) = cmd.find("{}") {
      cmd.replace_range(pos..pos + 2, &escaped_script_name);
    }

    let mut result = TestResult::NoBug;

    self.step += 1;
    println!("Step {:4}...", self.step);

    // Run the user command through the platform shell, mirroring C++
    // `popen`, which delegates to `cmd.exe /C` on Windows and `/bin/sh -c`
    // elsewhere. The previous port hardcoded `sh`, which does not exist on a
    // stock Windows install and made the reducer abort there.
    let mut command = {
      #[cfg(windows)]
      {
        use std::os::windows::process::CommandExt;
        // cmd.exe does not understand the MSVC `\"`-escaping that Rust's
        // normal argument quoting applies; pass the (already shell-quoted)
        // command line verbatim with `raw_arg` so cmd parses it itself.
        let mut c = Command::new("cmd");
        c.arg("/C");
        c.raw_arg(&cmd);
        c
      }
      #[cfg(not(windows))]
      {
        let mut c = Command::new("sh");
        c.arg("-c").arg(&cmd);
        c
      }
    };
    let mut child = command
      .stdout(Stdio::piped())
      .spawn()
      .expect("failed to run reducer command");

    if let Some(stdout) = child.stdout.take() {
      let mut reader = BufReader::new(stdout);
      let mut line = String::new();

      loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).unwrap_or(0);
        if bytes_read == 0 {
          break;
        }

        if line.contains(self.search_text.as_str()) {
          result = TestResult::BugFound;
          break;
        }
      }
    }

    let _ = child.wait();

    result
  }
}
