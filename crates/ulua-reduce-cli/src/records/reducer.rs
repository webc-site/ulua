use core::ptr::null_mut;

use ulua_ast::{
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
    parse_result::ParseResult,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

use super::node::Block;

/// `CstNodeMap` 的空表。`null_mut()` 是 `DenseHashMap` 的「空槽哨兵 key」实参
/// （跨 crate API 签名要求，纯数据值，从不被解引用），不是可空指针字段。
pub(crate) fn empty_cst_node_map() -> CstNodeMap {
  CstNodeMap::new(null_mut())
}

/// `Reducer` is a native-only utility for reducing Luau source code while preserving a bug.
/// It is not portable to wasm32-unknown-unknown.
///
/// 用 Rust 默认布局：本类型只在 CLI 内部持有（cpp `Reduce.cpp` 的普通 `struct Reducer`），
/// 从不跨 FFI 边界、不参与 `transmute`/C ABI，`repr(C)` 是移植期残留，只会妨碍字段重排。
#[derive(Debug)]
pub struct Reducer {
  // Box 钉堆：name_table 内部存 `*mut Allocator`（捕获宿主地址）；堆地址
  // 永不移动，Reducer 整体被 move 也不会悬垂（先例：ulua-compiler tests.rs
  // string_table!；此前的"移动后 rebind"舞步已随之移除）。
  pub allocator: Box<Allocator>,
  pub name_table: AstNameTable,
  pub parse_options: ParseOptions,
  pub parse_result: ParseResult,
  pub cst_node_map: CstNodeMap,
  /// 根块句柄；`run_from_source` 解析成功前为 `None`（cpp 空 `root` 哨兵的
  /// Option 化）。解析产物节点存活由 `allocator` 保证。
  pub root: Option<Block>,
  pub script_name: String,
  pub command: String,
  pub search_text: String,
  pub step: i32,
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
      // C++ 默认构造等价物：root 为空、各集合为空（ParseResult::default 即此形状）
      parse_result: ParseResult::default(),
      cst_node_map: empty_cst_node_map(),
      root: None,
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
