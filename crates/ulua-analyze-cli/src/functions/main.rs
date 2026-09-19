use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use std::{
  fs::write,
  panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
  process::exit,
};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{
    freeze::freeze, register_builtin_globals::register_builtin_globals,
    to_string_error_alt_k::to_string_type_error_type_error_to_string_options,
  },
  records::{
    file_resolver::FileResolver, frontend::Frontend, frontend_options::FrontendOptions,
    internal_compiler_error::InternalCompilerError, internal_error::InternalError,
    time_limit_error::TimeLimitError, type_error::TypeError,
    type_error_to_string_options::TypeErrorToStringOptions, user_cancel_error::UserCancelError,
  },
  type_aliases::{collections::HashSet, frontend_callbacks::TaskQueue, module_name_type::ModuleName},
};
use ulua_ast::enums::mode::Mode;
use ulua_cli_lib::functions::{
  argv::argv, assertion_handler::install_assertion_handler, atoi::atoi,
  get_source_files::get_source_files_from_slice,
  join_paths_file_utils_alt_b::join_paths_string_view_string_view,
  set_luau_flags_default::set_luau_flags_default, set_luau_flags_flags_alt_b::set_luau_flags,
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
pub fn main() {
  exit(run());
}

fn run() -> i32 {
  // cpp 的 `char** argv` 按字节使用；`env::args()` 遇非 UTF-8 会 panic，统一走 lossy argv()
  let args = argv();
  let argv0 = args.first().map(String::as_str).unwrap_or("luau-analyze");

  // Luau::assertHandler() = assertionHandler;
  install_assertion_handler();

  // setLuauFlagsDefault();
  set_luau_flags_default();

  // if (argc >= 2 && strcmp(argv[1], "--help") == 0) { displayHelp(argv[0]); return 0; }
  if args.len() >= 2 && args[1] == "--help" {
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

    if arg == "--formatter=plain" {
      format = ReportFormat::Luacheck;
    } else if arg == "--formatter=gnu" {
      format = ReportFormat::Gnu;
    } else if arg == "--mode=strict" {
      mode = Mode::Strict;
    } else if arg == "--annotate" {
      annotate = true;
    } else if arg == "--timetrace" {
      // SAFETY: 启动期解析 --timetracing 时写入，先于任何 VM 线程
      unsafe { fflag::DebugLuauTimeTracing.set(true) };
    } else if let Some(rest) = arg.strip_prefix("--fflags=") {
      set_luau_flags(rest);
    } else if let Some(rest) = arg.strip_prefix("-j") {
      // cpp: strtol(argv[i] + 2, nullptr, 10) — 前缀数字语义
      thread_count = atoi(rest);
    } else if let Some(rest) = arg.strip_prefix("--logbase=") {
      base_path = String::from(rest);
    } else if arg == "--solver=old" {
      solver_mode = SolverMode::Old;
    } else if arg == "--solver=new" {
      solver_mode = SolverMode::New;
    } else {
      // cpp 消息以 "\n\n" 结尾
      eprintln!("Error: Unrecognized option '{arg}'.\n");
      display_help(argv0);
      return 1;
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
  let mut file_resolver = CliFileResolver::new();
  let mut config_resolver = CliConfigResolver::new(mode);

  // Frontend frontend(solverMode, &fileResolver, &configResolver, frontendOptions);
  // 两个解析器都是 main 的局部变量，地址不随 `frontend` 移动；构造函数已把它们存进
  // `file_resolver` / `config_resolver` 字段（frontend_frontend_frontend.rs），
  // 因此无需在构造后再写一次。`&raw mut` 只取地址、不派生 `&mut`，避免与
  // `frontend` 内部的长期裸指针互相失效。
  let mut frontend = Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
    solver_mode,
    &raw mut file_resolver as *mut dyn FileResolver,
    &raw mut config_resolver.base,
    frontend_options,
  );
  // 自引用指针（builtinTypes/moduleResolver(this)）由 wire_self_pointers 负责。
  unsafe {
    frontend.wire_self_pointers();
  }

  // if (FFlag::DebugLuauLogSolverToJsonFile) { frontend.writeJsonLog = ...; }
  if fflag::DebugLuauLogSolverToJsonFile.get() {
    // base_path 后续不再使用, 直接 move 进闭包免 clone
    frontend.write_json_log = Some(Rc::new(move |module_name: &ModuleName, log: String| {
      let mut path = alloc::format!("{}.log.json", module_name);
      if let Some(pos) = module_name.rfind('/') {
        path = String::from(&module_name[pos + 1..]);
      }
      if !base_path.is_empty() {
        path = join_paths_string_view_string_view(&base_path, &path);
      }
      // cpp: ofstream 写入失败同样静默, 但 printf 无条件执行
      let _ = write(&path, alloc::format!("{}\n", log));
      println!("Wrote JSON log to {}", path);
    }));
  }

  // registerBuiltinGlobals(frontend, frontend.globals);
  // freeze(frontend.globals.globalTypes);
  unsafe {
    let frontend_ptr: *mut Frontend = &mut frontend;
    // NOTE(跨 crate 缺口，勿模仿；review A1 C12)：cpp 签名是
    // `registerBuiltinGlobals(Frontend&, GlobalTypes&)`，本行照此从同一个
    // `frontend_ptr` 派生出两个并存的可变借用（`&mut *frontend_ptr` 与其字段
    // `(*frontend_ptr).globals`），且函数体内 `frontend.load_definition_file(globals, …)`
    // 会同时经两者写入 —— 这是刻意为等价移植保留的重叠借用，Stacked Borrows/noalias
    // 下并不成立。根因在 ulua-analysis 的签名未收窄（应收成 `&mut Frontend` 单一入参
    // 或让 globals 由 frontend 内部取得），收窄后本行改回单一借用。
    // **禁止在其他位置复制此形态。**
    register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
    freeze((*frontend_ptr).globals.global_types_mut());
  }

  // std::vector<std::string> files = getSourceFiles(argc, argv);
  let files = get_source_files_from_slice(&args);

  // for (const std::string& path : files) frontend.queueModuleCheck(path);
  frontend.queue_module_check_vector_module_name(&files);

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
      let module_name = ice
        .module_name
        .clone()
        .unwrap_or_else(|| String::from("<unknown module>"));
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
          file_resolver: Some(frontend.file_resolver),
        },
      );
      report(
        format,
        &human_readable_name,
        &location,
        "InternalCompilerError",
        &message,
      );
      return 1;
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
  let checked_names: HashSet<&str> = checked_modules.iter().map(String::as_str).collect();
  // cpp: fprintf(stderr, "Error opening %s\n"); failed++;
  for path in &files {
    if !checked_names.contains(path.as_str()) {
      eprintln!("Error opening {path}");
      failed += 1;
    }
  }

  // if (!configResolver.configErrors.empty()) { ... }
  let config_errors = config_resolver.config_errors();
  if !config_errors.is_empty() {
    failed += config_errors.len() as i32;

    for (path, error) in config_errors {
      eprintln!("{}: {}", path, error);
    }
  }

  if format == ReportFormat::Luacheck {
    0
  } else {
    i32::from(failed != 0)
  }
}
