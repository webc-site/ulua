use std::{
  io::{BufRead, BufReader},
  process::{Command, Stdio},
};

use memchr::memmem;

use crate::{enums::test_result::TestResult, records::reducer::Reducer};

impl Reducer {
  pub fn run(&mut self) -> TestResult {
    self.write_temp_script(false);

    let escaped_script_name = self.escape(&self.script_name);
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
    // spawn 失败（命令不存在等）时不 panic：cpp `popen` 失败后读不到任何
    // 输出，等价按 NoBug 处理，随后的主流程会给出 "Could not find failure
    // string" 提示。
    let mut child = match command.stdout(Stdio::piped()).spawn() {
      Ok(child) => child,
      Err(_) => return TestResult::NoBug,
    };

    if let Some(stdout) = child.stdout.take() {
      let mut reader = BufReader::new(stdout);
      let mut line = Vec::new();

      loop {
        line.clear();
        // cpp `fgets` 按字节读行；用 read_until 而非 read_line，避免命令输出
        // 含非法 UTF-8 时校验失败被误判为 EOF 而提前停止
        if reader.read_until(b'\n', &mut line).unwrap_or(0) == 0 {
          break;
        }

        if memmem::find(&line, self.search_text.as_bytes()).is_some() {
          result = TestResult::BugFound;
          break;
        }
      }
    }

    let _ = child.wait();

    result
  }
}
