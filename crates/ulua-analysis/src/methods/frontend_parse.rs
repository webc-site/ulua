use alloc::{sync::Arc, vec::Vec};

use ulua_ast::{
  enums::mode::Mode,
  records::{
    ast_stat_block::AstStatBlock, location::Location, node_handle::Nodes,
    parse_options::ParseOptions, parser::Parser, position::Position,
  },
};
use ulua_common::macros::luau_timetrace_scope::{LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_SCOPE};

use crate::{
  functions::{get_timestamp::get_timestamp, parse_mode::parse_mode},
  records::{
    arena_handle::Handle, frontend::Frontend, source_module::SourceModule,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};

impl Frontend {
  pub fn parse_module_name(&mut self, name: &ModuleName) {
    LUAU_TIMETRACE_SCOPE!("Frontend::parse", "Frontend");
    LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

    if self.get_check_result(name, false, false).is_some() {
      return;
    }

    let mut build_queue: Vec<ModuleName> = Vec::new();
    self.parse_graph(&mut build_queue, name, &TypeCheckLimits::default(), false);
  }

  pub fn parse_module_name_string_view_parse_options(
    &mut self,
    name: &ModuleName,
    src: &str,
    parse_options: &ParseOptions,
  ) -> SourceModule {
    LUAU_TIMETRACE_SCOPE!("Frontend::parse", "Frontend");
    LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

    let mut source_module = SourceModule::new();

    let timestamp = get_timestamp();

    let parse_result = Parser::parse(
      src,
      Arc::get_mut(&mut source_module.names)
        .expect("SourceModule names must be uniquely owned while parsing"),
      Arc::get_mut(&mut source_module.allocator)
        .expect("SourceModule allocator must be uniquely owned while parsing"),
      parse_options.clone(),
    );

    self.stats.time_parse += get_timestamp() - timestamp;
    self.stats.files += 1;
    self.stats.lines += parse_result.lines;

    if !parse_result.errors.is_empty() {
      source_module
        .parse_errors
        .extend(parse_result.errors.iter().cloned());
    }

    if parse_result.errors.is_empty() || !parse_result.root.is_null() {
      // cpp `sourceModule.root = parseResult.root`：null 产物折叠为缺席态 `None`。
      source_module.root = Handle::from_opt_ptr(parse_result.root);
      source_module.mode = parse_mode(&parse_result.hotcomments);
    } else {
      // 兜底空块由 arena 现分配、恒非空（cpp `NotNull` 同形），句柄直接 `Some`。
      source_module.root = Some(Handle::from_ptr(
        Arc::get_mut(&mut source_module.allocator)
          .expect("SourceModule allocator must be uniquely owned while parsing")
          .alloc(AstStatBlock::new(
            Location::new(Position::default(), Position::default()),
            Nodes::default(),
            false,
          )),
      ));
      source_module.mode = Some(Mode::NoCheck);
    }

    source_module.name = name.clone();
    source_module.human_readable_name = self.module_resolver.get_human_readable_module_name(name);

    if parse_options.capture_comments {
      source_module.comment_locations = parse_result.comment_locations;
      source_module.hotcomments = parse_result.hotcomments;
    }

    source_module
  }
}
