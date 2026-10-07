//! `ulua-compile` 的行为级测试：`--record-stats` 落盘的 JSON 契约、`--stats-file=` 自定义名、
//! `--dump-regspills` 的未生效提示，以及参数校验的退出码。
//!
//! 对照上游 `CLI/src/Compile.cpp`：`main`（487-740）与 `serializeCompileStats`
//! （181-286）。子进程执行（`CARGO_BIN_EXE_*`），因为被测行为含 stdout/stderr 与真实文件写盘。

use ulua_cli_lib::test_utils::{Workspace, code, normalize_digits, stderr_of, stdout_of};

const BIN: &str = env!("CARGO_BIN_EXE_ulua-compile");

/// 每个用例独占的工作目录（stats 产物写在 cwd，需要隔离；共享夹具，
/// temp 目录以本 crate 名前缀隔离）
fn ws(name: &str) -> Workspace {
  Workspace::new(BIN, "ulua-compile-cli", name)
}

/// 单个文件 stats 块的正文（键序、缩进、分隔符与 cpp `serializeCompileStats` 逐字一致）。
/// 数值统一归一为 `<n>`：耗时每次运行都变，字节码长度属于 codegen 单元的契约。
const STATS_BODY: &str = r#"        "lines": <n>,
        "bytecode": <n>,
        "bytecodeInstructionCount": <n>,
        "codegen": <n>,
        "readTime": <n>.<n>,
        "miscTime": <n>.<n>,
        "parseTime": <n>.<n>,
        "compileTime": <n>.<n>,
        "codegenTime": <n>.<n>,
        "lowerStats": {
            "totalFunctions": <n>,
            "skippedFunctions": <n>,
            "spillsToSlot": <n>,
            "spillsToRestore": <n>,
            "maxSpillSlotsUsed": <n>,
            "blocksPreOpt": <n>,
            "blocksPostOpt": <n>,
            "maxBlockInstructions": <n>,
            "regAllocErrors": <n>,
            "loweringErrors": <n>,
            "blockLinearizationStats": {
                "constPropInstructionCount": <n>,
                "timeSeconds": <n>.<n>
            },
            "functions": []
        }"#;

/// cpp `recordStats == Total`: 顶层即单个 stats 对象
#[test]
fn record_stats_total_writes_expected_json() {
  let ws = ws("total");
  ws.write(
    "a.luau",
    "local function add(a, b)\n  return a + b\nend\n\nreturn add(1, 2)\n",
  );

  let output = ws.run(&["--null", "--record-stats=total", "a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));

  let body = ws.read("stats.json");
  let expected = format!("{{\n{STATS_BODY}\n    }}");
  assert_eq!(normalize_digits(&body), expected);
  // 结构之外再钉一个具体值：统计确实来自被编译的那个文件
  assert!(body.contains("\"lines\": 5,"), "stats.json:\n{body}");
}

/// cpp `--stats-file=<filename>`: 产物名由用户决定，默认名不应被创建
#[test]
fn stats_file_overrides_default_name() {
  let ws = ws("custom-name");
  ws.write("a.luau", "return 1\n");

  assert_eq!(
    code(&ws.run(&[
      "--null",
      "--record-stats=total",
      "--stats-file=custom-stats.json"
    ])),
    0
  );
  assert!(ws.exists("custom-stats.json"), "自定义文件名未生效");
  assert!(!ws.exists("stats.json"), "默认 stats.json 不该被写出");
}

/// cpp `recordStats == File`: `{"<file>": <stats>, ...}`，条目分隔符为 `,`，最后一个条目无分隔符
#[test]
fn record_stats_file_maps_every_input_in_order() {
  let ws = ws("file");
  ws.write("a.luau", "return 1\n");
  ws.write("b.luau", "local x = 2\nreturn x\n");

  assert_eq!(
    code(&ws.run(&["--null", "--record-stats=file", "a.luau", "b.luau"])),
    0
  );

  let body = ws.read("stats.json");
  let expected = format!(
    "{{\n    \"./a.luau\": {{\n{STATS_BODY}\n    }},\n    \"./b.luau\": {{\n{STATS_BODY}\n    }}\n}}"
  );
  assert_eq!(normalize_digits(&body), expected);
}

/// cpp `recordStats == Function`: 与 File 同样的按文件外层结构（`functions` 数组由
/// codegen 填充，`--null` 不跑 codegen 故为空，与上游一致）
#[test]
fn record_stats_function_keeps_per_file_structure() {
  let ws = ws("function");
  ws.write("a.luau", "return 1\n");

  assert_eq!(
    code(&ws.run(&[
      "--null",
      "--record-stats=function",
      "--stats-file=fn.json",
      "a.luau",
    ])),
    0
  );

  let body = normalize_digits(&ws.read("fn.json"));
  let expected = format!("{{\n    \"./a.luau\": {{\n{STATS_BODY}\n    }}\n}}");
  assert_eq!(body, expected);
}

/// `--dump-regspills` 在本 build 无消费点（`AssemblyOptions` 缺 `include_reg_spills`），
/// 必须像 analyze-cli 的 `-j` 一样明确告知用户未生效，而不是静默接受
#[test]
fn dump_regspills_reports_that_flag_is_inert() {
  let ws = ws("regspills");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["--null", "--dump-regspills", "a.luau"]);
  assert_eq!(code(&output), 0, "stderr: {}", stderr_of(&output));
  assert!(
    stderr_of(&output)
      .lines()
      .any(|line| line.starts_with("note: --dump-regspills is not effective")),
    "stderr: {}",
    stderr_of(&output)
  );
}

/// 未指定 `--dump-regspills` 时不应有多余提示
#[test]
fn no_regspills_note_without_flag() {
  let ws = ws("regspills-off");
  ws.write("a.luau", "return 1\n");

  assert!(!stderr_of(&ws.run(&["--null", "a.luau"])).contains("--dump-regspills"));
}

/// cpp Compile.cpp:559-573：非法 granularity → 报错退出 1
#[test]
fn unknown_stats_granularity_exits_one() {
  let ws = ws("bad-granularity");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["--record-stats=bogus", "a.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(
    stderr_of(&output),
    "Error: unknown 'granularity' for '--record-stats'.\n"
  );
}

/// cpp Compile.cpp:587-596：`--stats-file=` 缺文件名 → 退出 1（消息以空行结尾）
#[test]
fn stats_file_without_name_exits_one() {
  let ws = ws("no-name");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["--record-stats=total", "--stats-file=", "a.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(
    stderr_of(&output),
    "Error: filename missing for '--stats-file'.\n\n"
  );
}

/// cpp Compile.cpp:633-637：`--bytecode-summary` 必须配 `--record-stats=function`
#[test]
fn bytecode_summary_requires_function_stats() {
  let ws = ws("summary-needs-stats");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["--bytecode-summary", "a.luau"]);
  assert_eq!(code(&output), 1);
  assert_eq!(
    stderr_of(&output),
    "'Error: Required '--record-stats=function' for '--bytecode-summary'.\n"
  );
}

/// cpp Compile.cpp:507-535：级别越界（含 `-t`）逐条报错并退出 1
#[test]
fn out_of_range_levels_exit_one() {
  let ws = ws("levels");
  ws.write("a.luau", "return 1\n");

  for (arg, expected) in [
    (
      "-O5",
      "Error: Optimization level must be between 0 and 2 inclusive.\n",
    ),
    (
      "-g9",
      "Error: Debug level must be between 0 and 2 inclusive.\n",
    ),
    (
      "-t2",
      "Error: Type info level must be between 0 and 1 inclusive.\n",
    ),
  ] {
    let output = ws.run(&[arg, "a.luau"]);
    assert_eq!(code(&output), 1, "{arg} 应退出 1");
    assert_eq!(stderr_of(&output), expected, "{arg} 文案不符");
  }
}

/// cpp Compile.cpp:625-630：未知选项 → 报错 + help + 退出 1
#[test]
fn unrecognized_option_prints_help_and_exits_one() {
  let ws = ws("unknown-option");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&["--definitely-not-an-option", "a.luau"]);
  assert_eq!(code(&output), 1);
  assert!(
    stderr_of(&output).starts_with("Error: Unrecognized option '--definitely-not-an-option'.\n\n"),
    "stderr: {}",
    stderr_of(&output)
  );
  assert!(stdout_of(&output).contains("Usage:"), "应打印 help");
}

/// cpp Compile.cpp:502-506：`--help` 退出 0
#[test]
fn help_exits_zero() {
  let ws = ws("help");
  let output = ws.run(&["--help"]);
  assert_eq!(code(&output), 0);
  assert!(stdout_of(&output).contains("--record-stats=<granularity>"));
}

/// 打不开 stats 文件时报出**真实文件名**与原因（上游硬编码 'stats.json' 且吞掉写盘错误）
#[test]
fn unwritable_stats_file_reports_real_name() {
  let ws = ws("unwritable");
  ws.write("a.luau", "return 1\n");

  let output = ws.run(&[
    "--null",
    "--record-stats=total",
    "--stats-file=missing-dir/stats.json",
  ]);
  assert_eq!(code(&output), 1);
  assert!(
    stderr_of(&output).contains("Unable to open 'missing-dir/stats.json'"),
    "stderr: {}",
    stderr_of(&output)
  );
}

/// cpp Compile.cpp:661-672,739：编译失败数决定退出码，但已收集的 stats 仍要落盘
#[test]
fn compile_failure_still_writes_stats_and_exits_one() {
  let ws = ws("failing");
  ws.write("bad.luau", "local 1 = \n");

  let output = ws.run(&["--null", "--record-stats=file", "bad.luau"]);
  assert_eq!(code(&output), 1);
  assert!(ws.exists("stats.json"), "失败不应阻止 stats 落盘");
}
