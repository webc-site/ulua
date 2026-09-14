use alloc::{string::String, vec::Vec};
use core::{ffi::c_char, ptr::null_mut};

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::records::{
  allocator::Allocator, ast_name::AstName, ast_name_table::AstNameTable, function::Function,
  lexeme::Type, lexer::Lexer, parse_options::ParseOptions, parser::Parser, position::Position,
};

const K_PARSE_NAME_ERROR: &str = "%error-id%";

impl Parser {
  pub fn new(
    buffer: &str,
    names: &mut AstNameTable,
    allocator: *mut Allocator,
    options: ParseOptions,
  ) -> Self {
    let resume_position = options
      .parse_fragment
      .as_ref()
      .map(|f| f.resume_position)
      .unwrap_or(Position { line: 0, column: 0 });

    let mut parser = Parser {
      options: options.clone(),
      lexer: Lexer::new(
        buffer.as_ptr() as *const c_char,
        buffer.len(),
        names,
        resume_position,
      ),
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
      local_map: DenseHashMap::new(AstName::default()),
      local_stack: Vec::with_capacity(16),
      classes_within_module: DenseHashSet::new(AstName::default()),
      parse_errors: Vec::new(),
      // C++ sizes this `[unsigned(Lexeme::Type::RESERVED_END_TOKEN)]`; 256 only
      // covered char tokens and overflowed on reserved-keyword types
      // (ReservedEnd=296 indexed into a len-256 vec).
      match_recovery_stop_on_token: alloc::vec![0; Type::RESERVED_END_TOKEN.0 as usize],
      declared_export_bindings: DenseHashMap::new(AstName::default()),
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
      cst_node_map: DenseHashMap::new(null_mut()),
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

    parser.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] = 0;
    parser.match_recovery_stop_on_token[Type::EOF.0 as usize] = 1;

    parser.lexer.set_skip_comments(true);

    LUAU_ASSERT!(parser.hotcomment_header);
    parser.next_lexeme();

    parser.hotcomment_header = false;

    if let Some(fragment) = &options.parse_fragment {
      parser.local_map = fragment.local_map.clone();
      parser.local_stack = fragment.local_stack.clone();
    }

    parser
  }
}
