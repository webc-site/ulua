use core::mem::replace;
use std::{
  io::{BufRead, BufReader},
  process::{self, Command, Stdio},
};

use memchr::memmem;
use ulua_ast::records::parser::Parser;

use crate::{
  enums::test_result::TestResult,
  functions::escape::escape,
  records::{
    node::Block,
    reducer::{Reducer, empty_cst_node_map},
  },
};

impl Reducer {
  /// cpp `Reducer::run()`（探针半）：写出临时脚本、跑用户命令，逐行在 stdout
  /// 里找特征串。返回是否复现了 bug。
  pub fn run(&mut self) -> TestResult {
    // 命令里代表"待归约脚本路径"的占位符（cpp 用字面量 "{}"）。
    const SCRIPT_PLACEHOLDER: &str = "{}";

    self.write_temp_script(false);

    let escaped_script_name = escape(&self.script_name);
    let mut cmd = self.command.clone();
    // 从上次插入结束处继续找（cpp Reduce.cpp:149 每次 find 从头重扫）：一来免掉
    // O(n^2) 重扫，二来修复脚本名本身含 "{}"（escape 后仍在）时无限自替换到 OOM
    // 的上游病——插入文本不再参与后续匹配。
    let mut start = 0;
    while let Some(pos) = cmd[start..].find(SCRIPT_PLACEHOLDER) {
      let pos = start + pos;
      cmd.replace_range(pos..pos + SCRIPT_PLACEHOLDER.len(), &escaped_script_name);
      start = pos + escaped_script_name.len();
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

impl Reducer {
  /// cpp `Reducer::run(scriptName, command, source, searchText)`（主流程半）：
  /// 解析 → 初测 → 逐块归约 → 输出。机器味的重载编码名并入本函数。
  pub fn run_from_source(
    &mut self,
    script_name: String,
    command: String,
    source: &str,
    search_text: &str,
  ) {
    self.script_name = script_name;

    println!("Script: {}", self.script_name);

    self.command = command;
    self.search_text = search_text.to_string();

    // `allocator` 字段是 Box 钉堆（见 Reducer 定义）：解析产出的节点存活于
    // arena 页，句柄在 `self` 借用期内有效。
    self.parse_result = Parser::parse(
      source,
      &mut self.name_table,
      &mut self.allocator,
      self.parse_options.clone(),
    );
    if !self.parse_result.errors.is_empty() {
      println!("Parse errors");
      process::exit(1);
    }

    // cpp `root = parseResult.root; cstNodeMap = std::move(...)`：root 以
    // `Option` 落位（解析成功即 `Some`），CST 表用空表复位。
    self.root = Block::new(self.parse_result.root);
    self.cst_node_map = replace(&mut self.parse_result.cst_node_map, empty_cst_node_map());

    let initial_result = self.run();
    if initial_result == TestResult::NoBug {
      println!(
        "Could not find failure string in the unmodified script!  Check your commandline arguments"
      );
      process::exit(2);
    }

    // 初测通过即证明 root 已落位（`Block::new` 非空），此处取回句柄。
    let root = self.root.expect("初测通过则解析必成功，root 已落位");
    self.walk(root);

    self.write_temp_script(true);

    println!("Done!  Check {}", self.script_name);
  }
}
