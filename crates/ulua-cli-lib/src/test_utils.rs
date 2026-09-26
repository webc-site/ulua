//! CLI 集成测试共享脚手架（`feature = "test-utils"` 或本 crate 自测时编译）。
//!
//! `ulua-ast-cli` / `ulua-bytecode-cli` / `ulua-compile-cli` / `ulua-reduce-cli`
//! 的输出契约测试都需要「每个用例独占一个临时工作目录 + 子进程跑 bin + Drop 清理」
//! 的同款夹具，此前四处各抄一份；收敛于此，用 `bin`（`CARGO_BIN_EXE_*`）与
//! `prefix`（目录命名空间）参数化。
//!
//! 刻意 feature 门控（默认关闭）：不扩大正常构建的公开 API 面，仅测试侧按需启用。

use std::{
  env::temp_dir,
  fs,
  io::Write,
  path::PathBuf,
  process::{Command, Output, Stdio, id},
  thread,
};

/// 每个用例独占的工作目录；`bin` 为被测可执行文件路径（来自
/// `env!("CARGO_BIN_EXE_...")`），`prefix` 为 crate 名（temp 目录命名空间）。
pub struct Workspace {
  bin: &'static str,
  dir: PathBuf,
}

impl Workspace {
  pub fn new(bin: &'static str, prefix: &str, name: &str) -> Self {
    let dir = temp_dir().join(format!("{prefix}-{pid}-{name}", pid = id()));
    fs::create_dir_all(&dir).expect("create workspace");
    Self { bin, dir }
  }

  pub fn write(&self, name: &str, source: &str) {
    fs::write(self.dir.join(name), source).expect("write fixture");
  }

  pub fn read(&self, name: &str) -> String {
    fs::read_to_string(self.dir.join(name)).expect("read artifact")
  }

  pub fn exists(&self, name: &str) -> bool {
    self.dir.join(name).exists()
  }

  /// 在工作目录里跑一次被测 bin。
  pub fn run(&self, args: &[&str]) -> Output {
    Command::new(self.bin)
      .args(args)
      .current_dir(&self.dir)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .output()
      .expect("run cli binary")
  }

  /// cpp CLI 的 `-` 语义：源码从 stdin 读取。
  pub fn run_with_stdin(&self, args: &[&str], input: &str) -> Output {
    let mut child = Command::new(self.bin)
      .args(args)
      .current_dir(&self.dir)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn()
      .expect("spawn cli binary");

    // 在独立线程写 stdin：被测进程若在读入途中产出超过管道缓冲（64KB）的
    // 输出，同线程先写后读会互相顶死（死锁）——读写并行是标准姿势。
    let mut stdin = child.stdin.take().expect("stdin pipe");
    let writer = {
      let input = input.to_string();
      thread::spawn(move || {
        stdin.write_all(input.as_bytes()).expect("write stdin");
      })
    };

    let output = child.wait_with_output().expect("wait cli binary");
    writer.join().expect("stdin writer thread");
    output
  }
}

impl Drop for Workspace {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.dir);
  }
}

/// 读取子进程 `Output` 的 stdout（UTF-8 宽松解码），各 CLI 契约测试的共用三件套之一
pub fn stdout_of(output: &Output) -> String {
  String::from_utf8_lossy(&output.stdout).into_owned()
}

/// 读取子进程 `Output` 的 stderr（UTF-8 宽松解码）
pub fn stderr_of(output: &Output) -> String {
  String::from_utf8_lossy(&output.stderr).into_owned()
}

/// 读取子进程 `Output` 的退出码（被测 CLI 均以 `exit(code)` 结束，不会因信号退出）
pub fn code(output: &Output) -> i32 {
  output.status.code().expect("exit code")
}

/// 把连续数字段压成 `<n>`：数值（计数、耗时）随运行变化，快照只锁行/缩进/分隔符结构
pub fn normalize_digits(text: &str) -> String {
  let mut out = String::with_capacity(text.len());
  let mut in_digits = false;

  for ch in text.chars() {
    if ch.is_ascii_digit() {
      if !in_digits {
        out.push_str("<n>");
        in_digits = true;
      }
    } else {
      in_digits = false;
      out.push(ch);
    }
  }

  out
}
