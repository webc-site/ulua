use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use std::{
  fs::write,
  panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{freeze::freeze, to_string_error::to_string_type_error_type_error_to_string_options},
  records::{
    frontend::Frontend, frontend_options::FrontendOptions,
    internal_compiler_error::InternalCompilerError, internal_error::InternalError,
    time_limit_error::TimeLimitError, type_error::TypeError,
    type_error_to_string_options::TypeErrorToStringOptions, user_cancel_error::UserCancelError,
  },
  type_aliases::{
    collections::HashSet, frontend_callbacks::TaskQueue, module_name_type::ModuleName,
  },
};
use ulua_ast::enums::mode::Mode;
use ulua_cli_lib::functions::{
  atoi::atoi, cli_preamble::cli_preamble, get_source_files::get_source_files_from_slice,
  join_paths_file_utils::join_paths, report_open_error::report_open_error,
  report_unrecognized_option::report_unrecognized_option, set_luau_flags_flags::set_luau_flags,
  time_trace_unsupported::time_trace_unsupported,
};
use ulua_common::fflag;

use crate::{
  enums::report_format::ReportFormat,
  functions::{
    display_help::display_help, report::report, report_module_result::report_module_result,
  },
  records::{cli_config_resolver::CliConfigResolver, cli_file_resolver::CliFileResolver},
};

/// cpp `int main(int argc, char** argv)` (CLI/src/Analyze.cpp)
pub fn run(args: &[String]) -> i32 {
  // cpp 的 `char** argv` 按字节使用；`env::args()` 遇非 UTF-8 会 panic，统一由调用方走 lossy argv()。
  // argv0 → 断言处理器 → 默认旗标 → 顶层 `--help` 检测收敛于共享前奏 `cli_preamble`。
  let preamble = cli_preamble(args, "luau-analyze");
  let argv0 = preamble.argv0;

  // if (argc >= 2 && strcmp(argv[1], "--help") == 0) { displayHelp(argv[0]); return 0; }
  if preamble.help_requested {
    display_help(argv0);
    return 0;
  }

  let mut format = ReportFormat::Default;
  let mut mode = Mode::Nonstrict;
  let mut annotate = false;
  let mut thread_count: i32 = 0;
  let mut base_path = String::new();
  let mut solver_mode = SolverMode::New;

  // for (int i = 1; i < argc; ++i)
  for arg in args.iter().skip(1).map(String::as_str) {
    if !arg.starts_with('-') {
      continue;
    }

    // 编译期可定的精确选项名收敛为 match 最小形式（rustc 对字符串 match 走
    // 长度+比较的分派，免逐条 ==）；带值的前缀选项（--fflags=/-j/--logbase=）
    // 与未识别回退留在 `_` 臂，顺序与原 if-else 一致。
    match arg {
      "--formatter=plain" => format = ReportFormat::Luacheck,
      "--formatter=gnu" => format = ReportFormat::Gnu,
      "--mode=strict" => mode = Mode::Strict,
      // cpp Analyze.cpp:428: `--mode=nonstrict` 显式接受，设回默认 Nonstrict
      "--mode=nonstrict" => mode = Mode::Nonstrict,
      "--annotate" => annotate = true,
      "--timetrace" => fflag::DebugLuauTimeTracing.set(true),
      "--solver=old" => solver_mode = SolverMode::Old,
      "--solver=new" => solver_mode = SolverMode::New,
      _ => {
        if let Some(rest) = arg.strip_prefix("--fflags=") {
          set_luau_flags(rest);
        } else if let Some(rest) = arg.strip_prefix("-j") {
          // cpp: strtol(argv[i] + 2, nullptr, 10) — 前缀数字语义
          thread_count = atoi(rest);
        } else if let Some(rest) = arg.strip_prefix("--logbase=") {
          base_path = String::from(rest);
        } else {
          report_unrecognized_option(arg);
          display_help(argv0);
          return 1;
        }
      }
    }
  }

  // The Rust build does not define LUAU_ENABLE_TIME_TRACE; mirror the C++ guard.
  if time_trace_unsupported() {
    return 1;
  }

  // FrontendOptions frontendOptions; retainFullTypeGraphs = annotate; runLintChecks = true;
  let frontend_options = FrontendOptions {
    retain_full_type_graphs: annotate,
    run_lint_checks: true,
    ..Default::default()
  };

  // CliFileResolver fileResolver; CliConfigResolver configResolver(mode);
  // resolver 与 frontend 一并 Box 固定堆地址：`Frontend` 存的是构造时布线的
  // 裸指针，移动任何一个都会让内部指针悬垂；Box 句柄可自由移动、堆内容恒定，
  // 从类型结构上消灭「落位后不得移动」的栈假设。构造入参以 `&mut dyn
  // FileResolver` 引用借用传入（unsize 强制在参数上完成）。
  let mut file_resolver = Box::new(CliFileResolver::new());
  let mut config_resolver = Box::new(CliConfigResolver::new(mode));

  // Frontend frontend(solverMode, &fileResolver, &configResolver, frontendOptions);
  // `new_boxed` 在 safe 边界内完成「构造 → 堆上落位 → 自指针布线」全序列，
  // 调用点不再有 unsafe 构造/`wire_self_pointers` 两步手写；resolver 经
  // `Box` 钉住堆地址、存活至 `run` 结束，满足其外部句柄长寿契约。
  let mut frontend = Frontend::new_boxed(
    solver_mode,
    &mut *file_resolver,
    Some(&mut config_resolver.base),
    frontend_options,
  );

  // if (FFlag::DebugLuauLogSolverToJsonFile) { frontend.writeJsonLog = ...; }
  if fflag::DebugLuauLogSolverToJsonFile.get() {
    // base_path 后续不再使用, 直接 move 进闭包免 clone
    frontend.write_json_log = Some(Rc::new(move |module_name: &ModuleName, log: String| {
      let mut path = alloc::format!("{}.log.json", module_name);
      if let Some(pos) = module_name.rfind('/') {
        path = String::from(&module_name[pos + 1..]);
      }
      if !base_path.is_empty() {
        path = join_paths(&base_path, &path, false);
      }
      // cpp: ofstream 写入失败同样静默, 但 printf 无条件执行
      let _ = write(&path, alloc::format!("{}\n", log));
      println!("Wrote JSON log to {path}");
    }));
  }

  // registerBuiltinGlobals(frontend, frontend.globals);
  // freeze(frontend.globals.globalTypes);
  // cpp 签名是 `registerBuiltinGlobals(Frontend&, GlobalTypes&)`，同一对象的
  // 整体与字段并存两把可变借用，安全借用检查器表达不了；#6 wave2 步④已把这层
  // 拆借收进 ulua-analysis 的单 `&mut Frontend` 门面
  // [`Frontend::register_builtin_globals`]（重叠别名窗口只剩库内一处），
  // 原 NOTE(跨 crate 缺口，勿模仿；review A1 C12) 在此了结，本处回到普通借用。
  frontend.register_builtin_globals(false);
  freeze(frontend.globals.global_types_mut());

  // std::vector<std::string> files = getSourceFiles(argc, argv);
  let files = get_source_files_from_slice(args);
  let file_module_names: Vec<ModuleName> =
    files.iter().map(|f| ModuleName::from(f.as_str())).collect();

  // for (const std::string& path : files) frontend.queueModuleCheck(path);
  frontend.queue_module_check_vector_module_name(&file_module_names);

  // cpp: `TaskScheduler scheduler(threadCount)` + `executeTasks` 把每个
  // performQueueItemTask 派发到 worker 线程上执行。Rust 端 `Frontend` 经
  // `wire_self_pointers` 自引用，且类型检查会原地改写它，既不是 `Send` 也不是 `Sync`，
  // 所以队列任务只能在持有 `&mut Frontend` 的本线程上跑：这里等价于 cpp 的默认 executor
  // （顺序立即执行）。`-j` 保持解析兼容但不再决定并发度——对用户可观测地说明缺口，
  // 避免以为参数生效。
  if thread_count > 1 {
    eprintln!("note: -j is not effective in this build; typechecking runs single-threaded");
  }

  // try { checkedModules = frontend.checkQueuedModules(...); }
  let result = catch_unwind(AssertUnwindSafe(|| {
    let execute_tasks: TaskQueue = Box::new(|tasks, run| {
      for task in tasks {
        run(task);
      }
    });

    frontend.check_queued_modules(None, execute_tasks, |_done, _total| true)
  }));

  let checked_modules: Vec<ModuleName> = match result {
    Ok(modules) => modules,
    Err(payload) => {
      // catch (const InternalCompilerError& ice)
      let ice: InternalCompilerError =
        if let Some(e) = payload.downcast_ref::<InternalCompilerError>() {
          e.clone()
        } else if let Some(e) = payload.downcast_ref::<TimeLimitError>() {
          e.base.clone()
        } else if let Some(e) = payload.downcast_ref::<UserCancelError>() {
          e.base.clone()
        } else {
          resume_unwind(payload);
        };

      let location = ice.location.unwrap_or_default();
      let module_name = ModuleName::from(ice.module_name.as_deref().unwrap_or("<unknown module>"));
      let human_readable_name = frontend
        .file_resolver_ref()
        .get_human_readable_module_name(&module_name);

      let error = TypeError::type_error_location_module_name_type_error_data(
        location,
        module_name,
        InternalError::new(ice.message.clone()).into(),
      );

      let message = to_string_type_error_type_error_to_string_options(
        &error,
        TypeErrorToStringOptions {
          file_resolver: Some(frontend.file_resolver_ref()),
        },
      );
      report(
        format,
        &human_readable_name,
        &location,
        "InternalCompilerError",
        &message,
      );
      // cpp Analyze.cpp:538: "Internal compile errors get their own exit code."
      // ICE 与类型错误共享退出码 1 会让 CI 无法区分两者，独立返回 2。
      return 2;
    }
  };

  let mut failed = 0i32;

  // for (const ModuleName& name : checkedModules) failed += !reportModuleResult(...);
  for name in &checked_modules {
    if !report_module_result(&mut frontend, name, format, annotate) {
      failed += 1;
    }
  }

  // for (const std::string& path : files) if (!checkedNames.count(path)) { ... }
  // cpp: std::set checkedNames(checkedModules.begin(), checkedModules.end())
  let checked_names: HashSet<&str> = checked_modules.iter().map(|m| m.as_str()).collect();
  // cpp: fprintf(stderr, "Error opening %s\n"); failed++;
  for path in &files {
    if !checked_names.contains(path.as_str()) {
      report_open_error(path);
      failed += 1;
    }
  }

  // if (!configResolver.configErrors.empty()) { ... }
  let config_errors = config_resolver.config_errors();
  if !config_errors.is_empty() {
    failed += config_errors.len() as i32;

    for (path, error) in config_errors.iter() {
      eprintln!("{path}: {error}");
    }
  }

  if format == ReportFormat::Luacheck {
    0
  } else {
    i32::from(failed != 0)
  }
}
