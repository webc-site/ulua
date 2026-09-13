/// A `Box<dyn Fn()>` carried across the thread boundary. The C++ `TaskScheduler`
/// stores `std::function<void()>` with no `Send` concept; the queued tasks capture
/// shared frontend state and run on worker threads exactly as here.
use alloc::rc::Rc;
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{cmp::min, ffi::c_char, mem::take, ptr::null_mut};
use std::{
  env::args,
  ffi::CString,
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
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::enums::mode::Mode;
use ulua_cli_lib::functions::{
  get_source_files::get_source_files,
  join_paths_file_utils_alt_b::join_paths_string_view_string_view,
  set_luau_flags_default::set_luau_flags_default, set_luau_flags_flags_alt_b::set_luau_flags,
};
use ulua_common::{FFlag, functions::assert_handler::assert_handler};

use crate::{
  enums::report_format::ReportFormat,
  functions::{
    assertion_handler::assertion_handler, display_help::display_help, report::report,
    report_module_result::report_module_result,
  },
  methods::task_scheduler_push::task_scheduler_push,
  records::{
    cli_config_resolver::CliConfigResolver,
    cli_file_resolver::CliFileResolver,
    task_scheduler::{Task, TaskScheduler},
  },
};
struct SendTask(Box<dyn Fn()>);
unsafe impl Send for SendTask {}

/// C++ `int main(int argc, char** argv)` (`CLI/src/Analyze.cpp:394-542`).
/// 执行器回调：把任务批量压入调度器队列（C++ executor lambda）。
type TaskSpawner = Box<dyn Fn(Vec<Box<dyn Fn()>>)>;

pub fn main() {
  exit(run());
}

fn run() -> i32 {
  // Build an owned argv of NUL-terminated C strings so the FileUtils/Flags ports
  // (which take `int argc, char** argv`) can be called faithfully.
  let owned_args: Vec<CString> = args()
    .map(|a| CString::new(a).unwrap_or_else(|_| CString::new("").unwrap()))
    .collect();
  let mut argv: Vec<*mut c_char> = owned_args
    .iter()
    .map(|c| c.as_ptr() as *mut c_char)
    .collect();
  argv.push(null_mut());
  let argc = owned_args.len() as i32;

  // Luau::assertHandler() = assertionHandler;
  *assert_handler() = Some(assertion_handler);

  // setLuauFlagsDefault();
  set_luau_flags_default();

  // if (argc >= 2 && strcmp(argv[1], "--help") == 0) { displayHelp(argv[0]); return 0; }
  let args: Vec<String> = args().collect();
  if args.len() >= 2 && args[1] == "--help" {
    display_help(&args[0]);
    return 0;
  }

  let mut format = ReportFormat::Default;
  let mut mode = Mode::Nonstrict;
  let mut annotate = false;
  let mut thread_count: i32 = 0;
  let mut base_path = String::new();
  let mut solver_mode = SolverMode::New;

  // for (int i = 1; i < argc; ++i)
  for arg in args.iter().skip(1) {
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
      FFlag::DebugLuauTimeTracing.set(true);
    } else if let Some(rest) = arg.strip_prefix("--fflags=") {
      set_luau_flags(rest);
    } else if let Some(rest) = arg.strip_prefix("-j") {
      thread_count = rest.parse::<i32>().unwrap_or(0);
    } else if let Some(rest) = arg.strip_prefix("--logbase=") {
      base_path = String::from(rest);
    } else if arg == "--solver=old" {
      solver_mode = SolverMode::Old;
    } else if arg == "--solver=new" {
      solver_mode = SolverMode::New;
    } else {
      eprintln!("Error: Unrecognized option '{arg}'.\n");
      display_help(&args[0]);
      return 1;
    }
  }

  // The Rust build does not define LUAU_ENABLE_TIME_TRACE; mirror the C++ guard.
  if FFlag::DebugLuauTimeTracing.get() {
    eprintln!("To run with --timetrace, Luau has to be built with LUAU_ENABLE_TIME_TRACE enabled");
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
  let mut frontend = Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
    solver_mode,
    &mut file_resolver.base,
    &mut config_resolver.base,
    frontend_options,
  );
  // Re-establish the resolver pointers and the self-referential pointers now that
  // `frontend` lives at a stable address (mirrors the project's wiring convention).
  frontend.file_resolver = &mut file_resolver.base;
  frontend.config_resolver = &mut config_resolver.base;
  unsafe {
    frontend.wire_self_pointers();
  }

  // if (FFlag::DebugLuauLogSolverToJsonFile) { frontend.writeJsonLog = ...; }
  if FFlag::DebugLuauLogSolverToJsonFile.get() {
    let base_path = base_path.clone();
    frontend.write_json_log = Some(Rc::new(move |module_name: &ModuleName, log: String| {
      let mut path = alloc::format!("{}.log.json", module_name);
      if let Some(pos) = module_name.rfind('/') {
        path = String::from(&module_name[pos + 1..]);
      }
      if !base_path.is_empty() {
        path = join_paths_string_view_string_view(&base_path, &path);
      }
      if write(&path, alloc::format!("{}\n", log)).is_ok() {
        println!("Wrote JSON log to {}", path);
      }
    }));
  }

  // registerBuiltinGlobals(frontend, frontend.globals);
  // freeze(frontend.globals.globalTypes);
  unsafe {
    let frontend_ptr: *mut Frontend = &mut frontend;
    register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
    freeze((*frontend_ptr).globals.global_types_mut());
  }

  // std::vector<std::string> files = getSourceFiles(argc, argv);
  let files = unsafe { get_source_files(argc, argv.as_mut_ptr()) };

  // for (const std::string& path : files) frontend.queueModuleCheck(path);
  frontend.queue_module_check_vector_module_name(&files);

  let mut checked_modules: Vec<ModuleName>;

  // if (threadCount <= 0) threadCount = std::min(getThreadCount(), 8u);
  if thread_count <= 0 {
    thread_count = min(TaskScheduler::get_thread_count(), 8) as i32;
  }

  // try { TaskScheduler scheduler(threadCount); checkedModules = frontend.checkQueuedModules(...); }
  let frontend_ptr: *mut Frontend = &mut frontend;
  let result = catch_unwind(AssertUnwindSafe(|| {
    let scheduler = TaskScheduler::task_scheduler_task_scheduler(thread_count as u32);
    let scheduler_ptr: *const TaskScheduler = &scheduler;

    // The executor pushes each task onto the scheduler queue, matching:
    //   [&](std::vector<std::function<void()>> tasks) { for (auto& t : tasks) scheduler.push(std::move(t)); }
    let execute_tasks: TaskSpawner = Box::new(move |tasks| {
      for task in tasks {
        let send_task = SendTask(task);
        let boxed: Task = Some(Box::new(move || {
          // Move the whole `SendTask` (which is `Send`) into the Closure so
          // the auto-trait analysis sees `Send`, rather than capturing only
          // the inner non-`Send` `Box<dyn Fn()>` (edition-2021 disjoint capture).
          let send_task = send_task;
          (send_task.0)();
        }));
        task_scheduler_push(unsafe { &*scheduler_ptr }, boxed);
      }
    });

    let progress: Box<dyn Fn(usize, usize) -> bool> = Box::new(|_done, _total| true);

    let modules = unsafe { (*frontend_ptr).check_queued_modules(None, execute_tasks, progress) };

    // scheduler is dropped here (joins workers), matching the C++ block scope.
    drop(scheduler);
    modules
  }));

  match result {
    Ok(modules) => checked_modules = modules,
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
      let human_readable_name = unsafe {
        FileResolver::get_human_readable_module_name(frontend.file_resolver, &module_name)
      };

      let error = TypeError::type_error_location_module_name_type_error_data(
        location,
        module_name,
        InternalError::new(ice.message.clone()).into(),
      );

      let message = to_string_type_error_type_error_to_string_options(
        &error,
        TypeErrorToStringOptions {
          file_resolver: frontend.file_resolver,
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
  }

  let mut failed = 0i32;

  // for (const ModuleName& name : checkedModules) failed += !reportModuleResult(...);
  let names = take(&mut checked_modules);
  for name in &names {
    if !report_module_result(&mut frontend, name, format, annotate) {
      failed += 1;
    }
  }

  // if (!configResolver.configErrors.empty()) { ... }
  if !config_resolver.config_errors.is_empty() {
    failed += config_resolver.config_errors.len() as i32;

    for (path, error) in &config_resolver.config_errors {
      eprintln!("{}: {}", path, error);
    }
  }

  if format == ReportFormat::Luacheck {
    0
  } else if failed != 0 {
    1
  } else {
    0
  }
}
