//! 六个发布二进制的端到端 CLI 集成测试。
//!
//! 每个二进制覆盖：`--help` / `-h`、未知 flag、临时 `.luau` 文件的真实调用
//! （断言输出 + 退出码）、坏 / 缺失路径。退出码与消息对齐 C++ 移植（如
//! `ulua-reduce --help` 退出 1，对应上游 `help()` 的 `exit(1)`；
//! `ulua-analyze` / `ulua-bytecode` 以 stderr `Error: Unrecognized option`
//! 拒绝未知 flag，对齐 `CLI/src/Analyze.cpp` / `CLI/src/Bytecode.cpp`）。
//! 同形流水线（`bin(..)…assert().<状态>().<流>(谓词)`）经 `common::cli_case!`
//! 夹具收口，非常规判据（绑定 output 续查、谓词函数）保持显式链式写法。

use std::{fs::read_to_string, str::from_utf8};

use sonic_rs::JsonValueTrait;
#[macro_use]
mod common;
#[path = "common/missing_paths.rs"]
mod missing_paths;

use common::{bin, write_script};
use predicates::prelude::*;

// ---------------------------------------------------------------------------
// ulua  (REPL / script runner)
// ---------------------------------------------------------------------------

#[test]
fn repl_help_long_exits_zero_with_usage() {
  cli_case!(bin("ulua").arg("--help") => success,
    stdout(predicate::str::contains("Usage:").and(predicate::str::contains("interactive REPL"))));
}

#[test]
fn repl_help_short_exits_zero() {
  cli_case!(bin("ulua").arg("-h") => success, stdout(predicate::str::contains("Usage:")));
}

#[test]
fn repl_program_args_reach_script_as_varargs() {
  // cpp Repl.cpp：`-a` / `--program-args` 之后的参数原样推给主 chunk 作 varargs
  let (_dir, path) = write_script(
    "args.luau",
    "print('n=' .. select('#', ...))\nfor i = 1, select('#', ...) do print(i, (select(i, ...))) end\n",
  );
  cli_case!(bin("ulua").arg(&path).arg("-a").arg("alpha").arg("beta") => success,
  // print 的多参数分隔符是 luaB_print 的 '\t'
  stdout(
    predicate::str::contains("n=2")
      .and(predicate::str::contains("1\talpha"))
      .and(predicate::str::contains("2\tbeta")),
  ));
}

#[test]
fn repl_args_after_program_args_are_not_source_files() {
  // getSourceFiles 在 `-a` 处提前返回：其后的名字不应当作源文件被打开
  let (_dir, path) = write_script("ok.luau", "print('ran')\n");
  cli_case!(bin("ulua").arg(&path).arg("--program-args").arg("ghost.luau") => success,
    stdout(predicate::str::contains("ran")),
    stderr(predicate::str::contains("Error opening").not()));
}

#[test]
fn repl_interactive_flag_runs_script_then_repl_on_same_state() {
  // cpp Repl.cpp:676 `-i, --interactive: Run an interactive REPL after executing
  // the last script specified`（:717 解析、:863 `runFile(.., interactive && isLastFile)`）。
  // REPL 与脚本共享同一 state：脚本写入的全局 `x` 在随后的交互行可见。
  let (_dir, path) = write_script("sess.luau", "x = 42\nprint('script-ran')\n");
  cli_case!(bin("ulua").arg(&path).arg("-i").write_stdin("return x\n") => success,
  stdout(
    predicate::str::contains("script-ran")
      .and(predicate::function(|out: &str| out.lines().any(|line| line.trim() == "42"))),
  ));
}

#[test]
fn repl_unknown_flag_errors_nonzero() {
  cli_case!(bin("ulua").arg("--definitely-not-a-flag") => failure,
    stderr(predicate::str::contains("Unrecognized option")));
}

#[test]
fn repl_runs_script_file_and_prints_stdout() {
  let (_dir, path) = write_script("hello.luau", "print('hello-from-script')\n");
  cli_case!(bin("ulua").arg(&path) => success, stdout(predicate::str::contains("hello-from-script")));
}

#[test]
fn repl_missing_file_errors_nonzero() {
  let (_dir, missing) = missing_paths::not_exist("nope.luau");
  cli_case!(bin("ulua").arg(&missing) => failure, stderr(predicate::str::contains("Error opening")));
}

// ---------------------------------------------------------------------------
// ulua-analyze  (type-checker)
// ---------------------------------------------------------------------------

#[test]
fn analyze_help_exits_zero_with_usage() {
  cli_case!(bin("ulua-analyze").arg("--help") => success,
    stdout(predicate::str::contains("Usage:").and(predicate::str::contains("typecheck"))));
}

#[test]
fn analyze_reports_type_error_on_bad_strict_file() {
  let (_dir, path) = write_script(
    "bad.luau",
    "--!strict\nlocal x: number = \"not a number\"\nreturn x\n",
  );
  // CLI/src/Analyze.cpp's reportError writes diagnostics to stderr (fprintf to
  // stderr); the process exits non-zero when there are errors.
  cli_case!(bin("ulua-analyze").arg(&path) => failure,
    stderr(predicate::str::contains("TypeError").and(predicate::str::contains("number"))));
}

#[test]
fn analyze_clean_on_good_strict_file() {
  let (_dir, path) = write_script("good.luau", "--!strict\nlocal x: number = 42\nreturn x\n");
  // cpp reportModuleResult：无错误无 lint 时不写任何输出（stdout/stderr 双流锁空）
  cli_case!(bin("ulua-analyze").arg(&path) => success,
    stdout(predicate::str::is_empty()),
    stderr(predicate::str::is_empty()));
}

#[test]
fn analyze_unknown_flag_errors_nonzero() {
  // 对齐 CLI/src/Analyze.cpp：未知选项打印 `Error: Unrecognized option` 到
  // stderr 并 exit 1
  cli_case!(bin("ulua-analyze").arg("--no-such-flag") => failure,
    stderr(predicate::str::contains("Unrecognized option '--no-such-flag'.")));
}

#[test]
fn analyze_missing_file_is_defined_outcome() {
  // cpp oracle（CLI/src/Analyze.cpp）：缺失文件 readSource 失败，不产生
  // checked-module 结果，走 `Error opening %s` 分支（main 尾部对未出现在
  // checkedNames 的 path 打 stderr 并 failed++），退出码 1——是定义良好的
  // 结果而非崩溃；Rust 侧 main.rs 同形实现。
  let (_dir, missing) = missing_paths::not_exist("nope.luau");
  cli_case!(bin("ulua-analyze").arg(&missing) => failure,
    stdout(predicate::str::is_empty()),
    stderr(predicate::str::contains("Error opening ")),
    stderr(predicate::str::contains("nope.luau")),
    stderr(predicate::str::contains("panicked").not()));
}

// ---------------------------------------------------------------------------
// ulua-ast  (AST dump as JSON)
// ---------------------------------------------------------------------------

#[test]
fn ast_help_exits_zero() {
  cli_case!(bin("ulua-ast").arg("--help") => success, stdout(predicate::str::contains("Usage")));
}

#[test]
fn ast_emits_valid_json_for_real_file() {
  let (_dir, path) = write_script("prog.luau", "return 1 + 2\n");
  let out = cli_case!(bin("ulua-ast").arg(&path) => success);
  let stdout = from_utf8(&out.get_output().stdout).expect("utf8 stdout");
  let parsed: sonic_rs::Value = sonic_rs::from_str(stdout).expect("AST output must be valid JSON");
  // The root node is an AstStatBlock.
  assert_eq!(
    parsed["root"]["type"], "AstStatBlock",
    "unexpected AST root: {parsed}"
  );
}

#[test]
fn ast_no_args_prints_help_and_exits_one() {
  // CLI/src/Ast.cpp: argc < 2 -> displayHelp + return 1.
  cli_case!(bin("ulua-ast") => failure, stdout(predicate::str::contains("Usage")));
}

#[test]
fn ast_missing_file_errors_nonzero() {
  let (_dir, missing) = missing_paths::not_exist("nope.luau");
  cli_case!(bin("ulua-ast").arg(&missing) => failure,
    stderr(predicate::str::contains("Couldn't read source")));
}

#[test]
fn ast_reports_parse_errors_nonzero() {
  let (_dir, path) = write_script("syntax.luau", "local = = =\n");
  cli_case!(bin("ulua-ast").arg(&path) => failure, stderr(predicate::str::contains("Parse errors")));
}

// ---------------------------------------------------------------------------
// ulua-compile  (bytecode / disassembly)
// ---------------------------------------------------------------------------

#[test]
fn compile_help_exits_zero() {
  cli_case!(bin("ulua-compile").arg("--help") => success,
    stdout(predicate::str::contains("Available modes")));
}

#[test]
fn compile_short_help_exits_zero() {
  cli_case!(bin("ulua-compile").arg("-h") => success, stdout(predicate::str::contains("Usage")));
}

#[test]
fn compile_disasm_contains_return_opcode() {
  let (_dir, path) = write_script("ret.luau", "return 1 + 2\n");
  cli_case!(bin("ulua-compile").arg(&path) => success, stdout(predicate::str::contains("RETURN")));
}

#[test]
fn compile_unknown_flag_errors_nonzero() {
  cli_case!(bin("ulua-compile").arg("--definitely-not-a-flag") => failure,
    stderr(predicate::str::contains("Unrecognized option")));
}

#[test]
fn compile_bad_optimization_level_errors() {
  let (_dir, path) = write_script("ret.luau", "return 1\n");
  cli_case!(bin("ulua-compile").arg("-O9").arg(&path) => failure,
    stderr(predicate::str::contains("Optimization level")));
}

#[test]
fn compile_bad_debug_level_errors() {
  // cpp CLI/src/Compile.cpp:517-525：`-g<n>` 越界打印
  // "Error: Debug level must be between 0 and 2 inclusive."（单换行）并 return 1，
  // stdout 无输出。原 `contains("Optimization level")` 只覆盖 -O，-g/-t 同族分支
  // 此前零覆盖。
  cli_case!(bin("ulua-compile").arg("-g9") => failure,
    stdout(predicate::str::is_empty()),
    stderr(predicate::str::diff("Error: Debug level must be between 0 and 2 inclusive.\n")));
}

#[test]
fn compile_bad_type_info_level_errors() {
  // cpp Compile.cpp:527-535：`-t<n>` 界为 [0..1]（与 -O/-g 的 [0..2] 不同）
  cli_case!(bin("ulua-compile").arg("-t2") => failure,
    stdout(predicate::str::is_empty()),
    stderr(predicate::str::diff("Error: Type info level must be between 0 and 1 inclusive.\n")));
}

#[test]
fn compile_unknown_target_errors() {
  // cpp Compile.cpp:537-553：--target 只认 a64/a64_nf/x64/x64_ms，
  // 其余打印 "Error: unknown target" 并 return 1
  cli_case!(bin("ulua-compile").arg("--target=msp430") => failure,
    stdout(predicate::str::is_empty()),
    stderr(predicate::str::diff("Error: unknown target\n")));
}

#[test]
fn compile_stats_file_missing_filename_errors() {
  // cpp Compile.cpp:587-596：`--stats-file=` 空文件名报
  // "Error: filename missing for '--stats-file'." 后带空行（"\n\n"）并 return 1
  cli_case!(bin("ulua-compile").arg("--stats-file=") => failure,
  stdout(predicate::str::is_empty()),
  stderr(predicate::str::diff(
    "Error: filename missing for '--stats-file'.\n\n"
  )));
}

#[test]
fn compile_bytecode_summary_requires_record_stats() {
  // cpp Compile.cpp:632-637：`--bytecode-summary` 必须配
  // `--record-stats=function`，否则 stderr 逐字（含开头多余单引号的原文案）：
  // "'Error: Required '--record-stats=function' for '--bytecode-summary'."
  let (_dir, path) = write_script("ret.luau", "return 1\n");
  cli_case!(bin("ulua-compile").arg("--bytecode-summary").arg(&path) => failure,
  stdout(predicate::str::is_empty()),
  stderr(predicate::str::diff(
    "'Error: Required '--record-stats=function' for '--bytecode-summary'.\n"
  )));
}

#[test]
fn compile_binary_mode_emits_raw_bytecode() {
  // cpp Compile.cpp:77-79/650-654：`--binary` 把 luau_compile 产物原样写 stdout
  // （Windows 下 _setmode(_O_BINARY)）。首字节为版本号：luau-compile 启动即调
  // setLuauFlagsDefault()（Compile.cpp:494；Flags.cpp:40-45 把非实验性 Luau* 旗标
  // 全置 true），`LuauCompileFastpcall=true` 时 getVersion 返回 14
  //（BytecodeBuilder.cpp:1487-1503，同 Rust bytecode_builder_get_version.rs），
  // 而非无旗标库路径 `compile()` 锁的 9（library_e2e）——两路版本差正是本 CLI
  // 旗标默认的行为特征。0 是错误 blob 专用标记（BytecodeBuilder.cpp:1518）。
  let (_dir, path) = write_script("ret.luau", "return 1 + 2\n");
  let out = cli_case!(bin("ulua-compile").arg("--binary").arg(&path) => success)
    .get_output()
    .clone();
  assert_eq!(
    out.stdout.first(),
    Some(&14u8),
    "binary 产物应以版本字节 14 起始"
  );
}

#[test]
fn compile_null_mode_prints_aggregate_line() {
  // cpp Compile.cpp:673-681：`--null` 不产出字节码，stdout 仅一行汇总
  // "Compiled %d KLOC into %d KB bytecode (read %.2fs, parse %.2fs, compile %.2fs)"
  // ——数值随时序/产物尺寸变化，但字段结构是逐字契约。
  let (_dir, path) = write_script("ret.luau", "return 1 + 2\n");
  cli_case!(bin("ulua-compile").arg("--null").arg(&path) => success,
    stdout(predicate::str::is_match(
      r"Compiled \d+ KLOC into \d+ KB bytecode \(read \d+\.\d{2}s, parse \d+\.\d{2}s, compile \d+\.\d{2}s\)\n"
    ).expect("null 汇总行 regex 合法")));
}

#[test]
fn compile_missing_file_errors_nonzero() {
  let (_dir, missing) = missing_paths::not_exist("nope.luau");
  cli_case!(bin("ulua-compile").arg(&missing) => failure, stderr(predicate::str::contains("Error opening")));
}

// ---------------------------------------------------------------------------
// ulua-bytecode  (bytecode summary JSON)
// ---------------------------------------------------------------------------

#[test]
fn bytecode_help_exits_zero() {
  cli_case!(bin("ulua-bytecode").arg("--help") => success, stdout(predicate::str::contains("Usage")));
}

#[test]
fn bytecode_writes_summary_json_file() {
  let (dir, path) = write_script("prog.luau", "local function f() return 1 end\nreturn f()\n");
  let summary = dir.path().join("summary.json");
  // cpp Bytecode.cpp:296 `fprintf(stdout, "Bytecode summary written to '%s'\n", summaryFile)`
  // ——summaryFile 即 `--summary-file=` 后的原文（Bytecode.cpp:77），提示行
  // 逐字回显，无二次转义；锁完整字符串而非子串
  let note = format!("--summary-file={}", summary.display());
  let expected_note = format!("Bytecode summary written to '{}'\n", summary.display());
  let out = cli_case!(bin("ulua-bytecode").arg(&note).arg(&path) => success)
    .get_output()
    .clone();
  assert_eq!(
    from_utf8(&out.stdout).expect("utf8 stdout"),
    expected_note,
    "提示行应逐字回显 --summary-file= 的原文"
  );
  let contents = read_to_string(&summary).expect("summary file must exist");
  let parsed: sonic_rs::Value = sonic_rs::from_str(&contents).expect("summary must be valid JSON");
  assert!(
    parsed.is_object(),
    "summary JSON should be an object: {parsed}"
  );
}

#[test]
fn bytecode_bad_debug_level_errors() {
  // cpp Bytecode.cpp:65-73：`-g<n>` 越界打印
  // "Error: Debug level must be between 0 and 2 inclusive."（单换行）后
  // parseArgs 返回 false → main return 1；不打印帮助、stdout 为空。
  // （-O 越界与 ulua-compile 同文案，已由 compile_bad_optimization_level_errors
  // 覆盖；本分支此前在 ulua-bytecode 上零覆盖。）
  cli_case!(bin("ulua-bytecode").arg("-g9") => failure,
    stdout(predicate::str::is_empty()),
    stderr(predicate::str::diff("Error: Debug level must be between 0 and 2 inclusive.\n")));
}

#[test]
fn bytecode_empty_summary_file_errors() {
  // cpp Bytecode.cpp:75-83：`--summary-file=` 空值打印
  // "Error: filename missing for '--summary-file'." + 空行（"\n\n"）→ return 1
  cli_case!(bin("ulua-bytecode").arg("--summary-file=") => failure,
    stdout(predicate::str::is_empty()),
    stderr(predicate::str::diff("Error: filename missing for '--summary-file'.\n\n")));
}

#[test]
fn bytecode_unknown_flag_reports_error_then_shows_help_and_exits_zero() {
  // 忠实 cpp Bytecode.cpp:89-93：未知选项打印
  // "Error: Unrecognized option '%s'." + 空行到 stderr，随后 displayHelp
  // （该 CLI 的 displayHelp 内部 exit(0)）——进程仍以 0 退出。Rust
  // parse_args.rs:45-49 同构（report_unrecognized_option + display_help）。
  // 与 ulua-analyze 的 `return 1` 分支（Analyze.cpp）刻意不同，此用例即锁
  // 这一 CLI 间差异。
  cli_case!(bin("ulua-bytecode").arg("--no-such-flag") => success,
    stderr(predicate::str::diff("Error: Unrecognized option '--no-such-flag'.\n\n")),
    // Bytecode.cpp:35 displayHelp 首行 `Usage: %s [options] [file list]`
    //（%s 为实调 argv0，含路径故锁固定中段）
    stdout(predicate::str::contains("[options] [file list]")));
}

#[test]
fn bytecode_missing_file_errors_nonzero() {
  let (_dir, missing) = missing_paths::not_exist("nope.luau");
  // 文件不存在，无相对路径解析依赖，cwd 无需设置
  cli_case!(bin("ulua-bytecode").arg(&missing) => failure, stderr(predicate::str::contains("Error opening")));
}

// ---------------------------------------------------------------------------
// ulua-reduce  (test-case reducer)
// ---------------------------------------------------------------------------

#[test]
fn reduce_help_exits_one_faithfully() {
  // CLI/src/Reduce.cpp `help()` calls exit(1) — faithful non-zero exit.
  cli_case!(bin("ulua-reduce").arg("--help") => failure, stdout(predicate::str::contains("Syntax:")));
}

#[test]
fn reduce_wrong_arg_count_prints_syntax_and_exits_one() {
  // Fewer than 3 positional args -> help() -> exit(1).
  cli_case!(bin("ulua-reduce").arg("only-one-arg") => failure, stdout(predicate::str::contains("Syntax:")));
}

#[test]
fn reduce_runs_three_arg_reduction_to_marker() {
  // A real reduction: the "command" greps the (rewritten) script for a marker
  // string that the reducer must preserve. The reducer overwrites the input
  // file in place, so it lives in a private temp dir.
  let (dir, path) = write_script(
    "case.luau",
    "local UNIQUE_MARKER_42 = 1\nlocal unused = 2\nlocal also_unused = 3\nreturn UNIQUE_MARKER_42\n",
  );
  // `command` uses {} as the script-path placeholder; it echoes the (reduced)
  // script to stdout, and the search text is the marker the reducer must keep
  // present for the "bug" to reproduce. The reducer runs the command through
  // the platform shell, so use each shell's file-dumping builtin: `type` under
  // cmd.exe on Windows, `cat` under sh elsewhere.
  let dump_command = if cfg!(windows) { "type {}" } else { "cat {}" };
  cli_case!(bin("ulua-reduce")
    .current_dir(dir.path())
    .arg(&path)
    .arg(dump_command)
    .arg("UNIQUE_MARKER_42") => success)
  .stdout(predicate::str::contains("Done!"));

  // The reduced script (written back to `path`) must still contain the marker.
  let reduced = read_to_string(&path).expect("reduced script must exist");
  assert!(
    reduced.contains("UNIQUE_MARKER_42"),
    "reducer dropped the marker line; reduced output:\n{reduced}"
  );
}
