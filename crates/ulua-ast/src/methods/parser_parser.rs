use alloc::{string::String, vec::Vec};

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::type_lexer::Type,
  records::{
    allocator::Allocator, ast_name::AstName, ast_name_table::AstNameTable, function::Function,
    lexer::Lexer, parse_options::ParseOptions, parser::Parser, position::Position,
  },
};

const K_PARSE_NAME_ERROR: &str = "%error-id%";

impl Parser {
  pub fn new<B>(
    buffer: &B,
    names: &mut AstNameTable,
    allocator: *mut Allocator,
    options: ParseOptions,
  ) -> Self
  where
    B: AsRef<[u8]> + ?Sized,
  {
    let resume_position = options
      .parse_fragment
      .as_ref()
      .map(|f| f.resume_position)
      .unwrap_or(Position { line: 0, column: 0 });

    // options 按值移入 parser，省一次 ParseOptions::clone（含 FragmentParseResumeSettings
    // 的 Vec + DenseHashMap 堆分配）；末尾 fragment 借用改从 parser.options 读取。
    let mut parser = Parser {
      options,
      lexer: Lexer::new(buffer.as_ref(), names, resume_position),
      allocator,
      comment_locations: Vec::new(),
      hotcomments: Vec::new(),
      hotcomment_header: true,
      recursion_counter: 0,
      name_self: AstName::default(),
      name_number: AstName::default(),
      name_error: AstName::default(),
      name_nil: AstName::default(),
      end_mismatch_suspect: None,
      function_stack: Vec::with_capacity(8),
      type_function_depth: 0,
      local_map: DenseHashMap::default(),
      local_stack: Vec::with_capacity(16),
      classes_within_module: DenseHashMap::default(),
      parse_errors: Vec::new(),
      // C++ 以哨兵 `Lexeme::Type::Reserved_END`(=312，即最后一个保留字之后一位) 为
      // 长度初始化该表（cpp/Ast/src/Parser.cpp `matchRecoveryStopOnToken.assign(...)`，
      // Parser 构造函数内）。Rust 侧对应哨兵是 `Type::RESERVED_END_TOKEN`(=312)；
      // `Type::RESERVED_END`(=296) 是关键字 'end'（cpp 驼峰 `ReservedEnd`），不是哨兵。
      // 256 只覆盖单字符 token，会对保留字类型越界（如 296 索引进 len-256 vec）。
      // 长度是编译期常量，故为零初始化定长数组（无堆分配），见 `RecoveryStopTable`。
      match_recovery_stop_on_token: [0; Type::RESERVED_END_TOKEN.0 as usize],
      declared_export_bindings: DenseHashMap::default(),
      has_module_return: false,
      scratch_attr: Vec::new(),
      scratch_stat: Vec::with_capacity(16),
      scratch_string: Vec::new(),
      scratch_string_2: Vec::new(),
      scratch_expr: Vec::with_capacity(16),
      scratch_expr_aux: Vec::new(),
      scratch_binding: Vec::with_capacity(16),
      scratch_local: Vec::with_capacity(16),
      scratch_table_type_props: Vec::new(),
      scratch_cst_table_type_props: Vec::new(),
      scratch_type: Vec::new(),
      scratch_declared_class_props: Vec::new(),
      scratch_class_declarations: Vec::new(),
      scratch_item: Vec::new(),
      scratch_cst_item: Vec::new(),
      scratch_arg_name: Vec::new(),
      scratch_generic_types: Vec::new(),
      scratch_generic_type_packs: Vec::new(),
      scratch_opt_arg_name: Vec::new(),
      scratch_position: Vec::new(),
      scratch_position_2: Vec::new(),
      scratch_data: String::new(),
      cst_node_map: DenseHashMap::default(),
    };

    let top = Function {
      vararg: true,
      loop_depth: 0,
    };
    parser.function_stack.push(top);

    parser.name_self = names.get_or_add_str("self");
    parser.name_number = names.get_or_add_str("number");
    parser.name_error = names.get_or_add_str(K_PARSE_NAME_ERROR);
    parser.name_nil = names.get_or_add_str("nil");

    // cpp/Ast/src/Parser.cpp:331 `matchRecoveryStopOnToken[Lexeme::Type::Eof] = 1;`
    // —— 恢复扫描遇 EOF 立即停止。ReservedEnd 槽位无需再写零（零初始化已覆盖，
    // cpp 亦无对应语句）。
    parser.match_recovery_stop_on_token[Type::EOF.0 as usize] = 1;

    parser.lexer.set_skip_comments(true);

    LUAU_ASSERT!(parser.hotcomment_header);
    parser.next_lexeme();

    parser.hotcomment_header = false;

    // parse_fragment 仅 new 内读取（此处为最后一读）：take() 移动所有权，
    // 省两次堆分配克隆（DenseHashMap + Vec）；options.parse_fragment 置 None
    // 不影响后续（无其他读取点）。
    if let Some(fragment) = parser.options.parse_fragment.take() {
      parser.local_map = fragment.local_map;
      parser.local_stack = fragment.local_stack;
    }

    parser
  }
}
