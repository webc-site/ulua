use core::{ffi::c_char, ptr::from_mut};

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    allocator::Allocator, ast_array::AstArray, ast_attr::AstAttr,
    ast_declared_extern_type_property::AstDeclaredExternTypeProperty, ast_expr::AstExpr,
    ast_expr_table::Item as AstExprTableItem, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_local::AstLocal, ast_name::AstName,
    ast_node::AstNode, ast_stat::AstStat, ast_table_prop::AstTableProp, ast_type::AstType,
    binding::Binding, comment::Comment, cst_expr_table::CstExprTableItem,
    cst_type_table::CstTypeTableItem, function::Function, hot_comment::HotComment, lexer::Lexer,
    location::Location, match_lexeme::MatchLexeme, parse_error::ParseError,
    parse_options::ParseOptions, position::Position,
  },
  rtti::{CstNodeClass, cst_node_as},
  type_aliases::{
    ast_argument_name::AstArgumentName, ast_class_member::AstClassMember, cst_node_map::CstNodeMap,
  },
};

#[derive(Debug)]
pub struct Parser {
  pub(crate) options: ParseOptions,
  pub(crate) lexer: Lexer,
  pub(crate) allocator: *mut Allocator,
  pub(crate) comment_locations: Vec<Comment>,
  pub(crate) hotcomments: Vec<HotComment>,
  pub(crate) hotcomment_header: bool,
  pub(crate) recursion_counter: u32,
  pub(crate) name_self: AstName,
  pub(crate) name_number: AstName,
  pub(crate) name_error: AstName,
  pub(crate) name_nil: AstName,
  pub(crate) end_mismatch_suspect: Option<MatchLexeme>,
  pub(crate) function_stack: Vec<Function>,
  pub(crate) type_function_depth: usize,
  pub(crate) local_map: DenseHashMap<AstName, *mut AstLocal>,
  pub(crate) local_stack: Vec<*mut AstLocal>,
  pub(crate) classes_within_module: DenseHashSet<AstName>,
  pub(crate) parse_errors: Vec<ParseError>,
  pub(crate) match_recovery_stop_on_token: Vec<u32>,
  pub(crate) declared_export_bindings: DenseHashMap<AstName, Location>,
  pub(crate) has_module_return: bool,
  pub(crate) scratch_attr: Vec<*mut AstAttr>,
  pub(crate) scratch_stat: Vec<*mut AstStat>,
  pub(crate) scratch_string: Vec<AstArray<c_char>>,
  pub(crate) scratch_string_2: Vec<AstArray<c_char>>,
  pub(crate) scratch_expr: Vec<*mut AstExpr>,
  pub(crate) scratch_expr_aux: Vec<*mut AstExpr>,
  pub(crate) scratch_binding: Vec<Binding>,
  pub(crate) scratch_local: Vec<*mut AstLocal>,
  pub(crate) scratch_table_type_props: Vec<AstTableProp>,
  pub(crate) scratch_cst_table_type_props: Vec<CstTypeTableItem>,
  pub(crate) scratch_type: Vec<*mut AstType>,
  pub(crate) scratch_declared_class_props: Vec<AstDeclaredExternTypeProperty>,
  pub(crate) scratch_class_declarations: Vec<AstClassMember>,
  pub(crate) scratch_item: Vec<AstExprTableItem>,
  pub(crate) scratch_cst_item: Vec<CstExprTableItem>,
  pub(crate) scratch_arg_name: Vec<AstArgumentName>,
  pub(crate) scratch_generic_types: Vec<*mut AstGenericType>,
  pub(crate) scratch_generic_type_packs: Vec<*mut AstGenericTypePack>,
  pub(crate) scratch_opt_arg_name: Vec<Option<AstArgumentName>>,
  pub(crate) scratch_position: Vec<Position>,
  pub(crate) scratch_position_2: Vec<Position>,
  pub(crate) scratch_data: String,
  pub(crate) cst_node_map: CstNodeMap,
}

impl Parser {
  /// 查 ast→cst 映射并下转为 `T` 的可变形态（Printer::lookup_cst_node 的
  /// parser 对应）。unsafe 收口于此：映射值指向 arena 中存活的 CST 节点，
  /// `cst_node_as` 经 class_index 命中后 repr(C) 布局保证下转有效。
  ///
  /// 生命周期 `'b` 由调用方传入的 `&'b mut AstNode` 供给（不再凭空造
  /// `'static`）：AST 与 CST 同处一个 arena 且同步构造，故「该 AST 节点在
  /// `'b` 内被独占」即蕴含其映射的 CST 节点在 `'b` 内可独占写。
  /// 键只按地址比较，从不解引用写回。
  pub(crate) fn lookup_cst_node_mut<'b, T: CstNodeClass>(
    &mut self,
    ast_node: &'b mut AstNode,
  ) -> Option<&'b mut T> {
    // cpp 侧键型为 `AstNode*`；这里仅把地址重新拼回该形态用于查表。
    let key = from_mut(ast_node);
    let cst_node = *self.cst_node_map.find(&key)?;
    // SAFETY: 同函数级注释；'b 的独占借用即调用方对该 arena 区间的独占证明。
    unsafe { cst_node_as::<T>(cst_node).as_mut() }
  }
}
