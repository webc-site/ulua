use core::ptr::null_mut;

use ulua_ast::{
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
    parse_options::ParseOptions, parse_result::ParseResult,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

/// `Reducer` is a native-only utility for reducing Luau source code while preserving a bug.
/// It is not portable to wasm32-unknown-unknown.
#[repr(C)]
#[derive(Debug)]
pub struct Reducer {
  // Box 钉堆：name_table 内部存 `*mut Allocator`（捕获宿主地址）；堆地址
  // 永不移动，Reducer 整体被 move 也不会悬垂（先例：ulua-compiler tests.rs
  // string_table!；此前的"移动后 rebind"舞步已随之移除）。
  pub(crate) allocator: Box<Allocator>,
  pub(crate) name_table: AstNameTable,
  pub(crate) parse_options: ParseOptions,
  pub(crate) parse_result: ParseResult,
  pub(crate) cst_node_map: CstNodeMap,
  pub(crate) root: *mut AstStatBlock,
  pub(crate) script_name: String,
  pub(crate) command: String,
  pub(crate) search_text: String,
  pub(crate) step: i32,
}

impl Reducer {
  pub fn new() -> Self {
    let mut allocator = Box::new(Allocator::new());
    let name_table = AstNameTable::new(&mut allocator);
    let parse_options = ParseOptions {
      capture_comments: true,
      store_cst_data: true,
      ..ParseOptions::default()
    };

    Reducer {
      allocator,
      name_table,
      parse_options,
      parse_result: ParseResult {
        root: null_mut(),
        lines: 0,
        hotcomments: Vec::new(),
        errors: Vec::new(),
        comment_locations: Vec::new(),
        cst_node_map: CstNodeMap::new(null_mut()),
      },
      cst_node_map: CstNodeMap::new(null_mut()),
      root: null_mut(),
      script_name: String::new(),
      command: String::new(),
      search_text: String::new(),
      step: 0,
    }
  }
}

impl Default for Reducer {
  fn default() -> Self {
    Self::new()
  }
}
