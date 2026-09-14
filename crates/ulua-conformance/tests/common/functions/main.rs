use alloc::{string::String, vec::Vec};
use std::{
  env::args,
  time::{SystemTime, UNIX_EPOCH},
};

use ulua_common::functions::assert_handler::assert_handler;

use crate::common::functions::{
  get_register_callbacks::get_register_callbacks,
  init_system::init_system,
  run_conformance::{CODEGEN, OPTIMIZATION_LEVEL, VERBOSE},
  set_fast_flags::set_fast_flags,
  test_assertion_handler::test_assertion_handler,
};
extern crate alloc;
extern crate std;

fn parse_option_value(args: &[String], name: &str) -> Option<String> {
  for (i, arg) in args.iter().enumerate() {
    if let Some(value) = arg
      .strip_prefix(name)
      .and_then(|rest| rest.strip_prefix('='))
    {
      return Some(value.to_owned());
    }

    if arg == name {
      return args.get(i + 1).cloned();
    }
  }

  None
}

fn has_flag(args: &[String], name: &str) -> bool {
  args.iter().any(|arg| arg == name)
}

fn parse_optimization_level(args: &[String]) -> Option<i32> {
  for (i, arg) in args.iter().enumerate() {
    if arg == "-O" {
      return args.get(i + 1).and_then(|value| value.parse().ok());
    }

    if let Some(value) = arg.strip_prefix("-O")
      && !value.is_empty()
    {
      return value.parse().ok();
    }
  }

  None
}

fn print_help() {
  std::println!("Additional command line options:");
  std::println!(
    " -O[n]                                 Changes default optimization level (1) for conformance runs"
  );
  std::println!(
    " --verbose                             Enables verbose output (e.g. lua 'print' statements)"
  );
  std::println!(" --fflags=                             Sets specified fast flags");
  std::println!(" --list-fflags                         List all fast flags");
  std::println!(" --randomize                           Use a random RNG seed");
  std::println!(" --random-seed=n                       Use a particular RNG seed");
}

pub fn main() -> i32 {
  init_system();

  *assert_handler() = Some(test_assertion_handler);

  let args: Vec<String> = args().collect();

  if has_flag(&args, "--list-fflags") {
    return 0;
  }

  if has_flag(&args, "--verbose") {
    unsafe {
      VERBOSE = true;
    }
  }

  if has_flag(&args, "--codegen") {
    unsafe {
      CODEGEN = true;
    }
  }

  if let Some(level) = parse_optimization_level(&args) {
    if (0..=2).contains(&level) {
      unsafe {
        OPTIMIZATION_LEVEL = level;
      }
    } else {
      std::eprintln!("Optimization level must be between 0 and 2 inclusive");
    }
  } else if args.iter().any(|arg| arg == "-O" || arg.starts_with("-O")) {
    std::eprintln!("Optimization level must be between 0 and 2 inclusive");
  }

  let mut random_seed =
    parse_option_value(&args, "--random-seed").and_then(|value| value.parse::<u32>().ok());
  if has_flag(&args, "--randomize") && random_seed.is_none() {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default();
    let seed = now.as_secs() as u32;
    std::println!("Using RNG seed {}", seed);
    random_seed = Some(seed);
  }

  if let Some(flags) = parse_option_value(&args, "--fflags") {
    let parsed = flags
      .split(',')
      .filter(|flag| !flag.is_empty())
      .map(String::from)
      .collect::<Vec<_>>();
    set_fast_flags(&parsed);
  }

  if parse_option_value(&args, "--run_test").is_some() {
    if has_flag(&args, "--run_suites_in_file")
      || parse_option_value(&args, "--run_suites_in_file").is_some()
    {
      std::eprintln!("ERROR: Cannot pass both --run_test and --run_suites_in_file");
      return 1;
    }

    if has_flag(&args, "--run_cases_in_file")
      || parse_option_value(&args, "--run_cases_in_file").is_some()
    {
      std::eprintln!("ERROR: Cannot pass both --run_test and --run_cases_in_file");
      return 1;
    }
  }

  for cb in get_register_callbacks().iter().copied() {
    cb();
  }

  if has_flag(&args, "--help") || has_flag(&args, "-h") {
    print_help();
  }

  let _ = random_seed;
  0
}
