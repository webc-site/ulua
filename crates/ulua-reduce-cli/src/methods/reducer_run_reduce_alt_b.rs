use core::{mem::replace, ptr::null_mut};
use std::process;

use ulua_ast::{records::parser::Parser, type_aliases::cst_node_map::CstNodeMap};

use crate::{enums::test_result::TestResult, records::reducer::Reducer};

impl Reducer {
  pub fn run_string_string_string_view_string_view(
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

    // `allocator` 字段是 Box 钉堆（见 Reducer 定义）：`name_table` 捕获的
    // `*mut Allocator` 指向堆上固定地址，Reducer 被 move 也不会悬垂，
    // 无需再 rebind。
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

    self.root = self.parse_result.root;
    self.cst_node_map = replace(
      &mut self.parse_result.cst_node_map,
      CstNodeMap::new(null_mut()),
    );

    let initial_result = self.run();
    if initial_result == TestResult::NoBug {
      println!(
        "Could not find failure string in the unmodified script!  Check your commandline arguments"
      );
      process::exit(2);
    }

    self.walk(self.root);

    self.write_temp_script(true);

    println!("Done!  Check {}", self.script_name);
  }
}
