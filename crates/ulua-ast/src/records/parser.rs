use core::ptr::from_mut;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::type_lexer::Type,
  records::{
    allocator::Allocator, ast_array::AstArray, ast_attr::AstAttr,
    ast_declared_extern_type_property::AstDeclaredExternTypeProperty, ast_expr::AstExpr,
    ast_expr_table::Item as AstExprTableItem, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_local::AstLocal, ast_name::AstName,
    ast_node::AstNode, ast_stat::AstStat, ast_stat_class::AstStatClass,
    ast_table_prop::AstTableProp, ast_type::AstType, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic, binding::Binding, comment::Comment,
    cst_expr_table::CstExprTableItem, cst_node::CstNode, cst_type_table::CstTypeTableItem,
    function::Function, hot_comment::HotComment, lexer::Lexer, location::Location,
    match_lexeme::MatchLexeme, parse_error::ParseError, parse_options::ParseOptions,
    position::Position,
  },
  rtti::{AstNodeClass, CstNodeClass, cst_node_as, is_expr_class, is_stat_class, is_type_class},
  type_aliases::{
    ast_argument_name::AstArgumentName, ast_class_member::AstClassMember, cst_node_map::CstNodeMap,
  },
};

/// cpp `matchRecoveryStopOnToken` 的定长表：下标是 `Type` 的 i32 值（0..=311，
/// 即 `[Type::EOF, Type::RESERVED_END_TOKEN)`），长度取保留字区间末尾哨兵。
pub(crate) type RecoveryStopTable = [u32; Type::RESERVED_END_TOKEN.0 as usize];

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
  pub(crate) classes_within_module: DenseHashMap<AstName, *mut AstStatClass>,
  pub(crate) parse_errors: Vec<ParseError>,
  /// cpp `matchRecoveryStopOnToken`（Parser.cpp:330）：长度即编译期常量
  /// `Type::RESERVED_END_TOKEN`(=312，cpp `Lexeme::Type::Reserved_END`)，
  /// 故用定长数组而非 `Vec` —— 省掉每次 `Parser::new` 的一次堆分配，
  /// 且表长不再依赖运行时 `vec![0; n]` 求值。
  pub(crate) match_recovery_stop_on_token: RecoveryStopTable,
  pub(crate) declared_export_bindings: DenseHashMap<AstName, Location>,
  pub(crate) has_module_return: bool,
  pub(crate) scratch_attr: Vec<*mut AstAttr>,
  pub(crate) scratch_stat: Vec<*mut AstStat>,
  pub(crate) scratch_string: Vec<AstArray<u8>>,
  pub(crate) scratch_string_2: Vec<AstArray<u8>>,
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
  /// arena 分配统一入口：收拢 cpp `allocator.alloc<T>(...)` 在解析方法体内的
  /// 散落裸指针解引用（`(*self.allocator).alloc`），调用点不再各自开 unsafe。
  ///
  /// Safety: allocator 由 Parser 独占持有（Box 固定地址），`&mut self` 借用
  /// 保证调用瞬间无并发别名。
  pub(crate) fn alloc<T>(&mut self, value: T) -> *mut T {
    // Safety: 见函数级说明。
    unsafe { (*self.allocator).alloc(value) }
  }

  /// 分配并给出 `AstExpr` 家族视图（cpp `allocator.alloc<AstExprX>(..)` 的
  /// 返回即按基类使用）：repr(C) 单继承保证基址重合，家族归属按编译期
  /// class index 校验，调用点不再手写 `as *mut AstExpr` 未检强转。
  pub(crate) fn alloc_expr<T: AstNodeClass>(&mut self, value: T) -> *mut AstExpr {
    debug_assert!(is_expr_class(T::CLASS_INDEX), "not an AstExpr class");
    self.alloc(value).cast::<AstExpr>()
  }

  /// [`Self::alloc_expr`] 的 `AstStat` 家族形态。
  pub(crate) fn alloc_stat<T: AstNodeClass>(&mut self, value: T) -> *mut AstStat {
    debug_assert!(is_stat_class(T::CLASS_INDEX), "not an AstStat class");
    self.alloc(value).cast::<AstStat>()
  }

  /// [`Self::alloc_expr`] 的 `AstType` 家族形态。
  pub(crate) fn alloc_type<T: AstNodeClass>(&mut self, value: T) -> *mut AstType {
    debug_assert!(is_type_class(T::CLASS_INDEX), "not an AstType class");
    self.alloc(value).cast::<AstType>()
  }

  /// [`Self::alloc_expr`] 的 `AstTypePack` 家族形态。pack 家族只有 cpp
  /// `AstTypePack` 的三个直接派生类，判别表就地列出。
  pub(crate) fn alloc_type_pack<T: AstNodeClass>(&mut self, value: T) -> *mut AstTypePack {
    debug_assert!(
      matches!(
        T::CLASS_INDEX,
        AstTypePackExplicit::CLASS_INDEX
          | AstTypePackVariadic::CLASS_INDEX
          | AstTypePackGeneric::CLASS_INDEX
      ),
      "not an AstTypePack class"
    );
    self.alloc(value).cast::<AstTypePack>()
  }

  /// 建 ast→cst 映射：仅在 `store_cst_data` 时分配 CST 节点并插入。cpp 各
  /// 解析点 `if (options.storeCstData) { alloc<CstX>(...); cstNodeMap[n]=c; }`
  /// 的统一收口——调用点不再重复选项判空、指针强转与 unsafe（闭包内拿
  /// `&mut Allocator`，构造 CST 全程安全代码）。
  pub(crate) fn attach_cst<A, C>(
    &mut self,
    ast_node: *mut A,
    make_cst: impl FnOnce(&mut Allocator) -> *mut C,
  ) where
    C: CstNodeClass,
  {
    if !self.options.store_cst_data {
      return;
    }
    // Safety: allocator 由 Parser 独占持有（Box 固定地址），self 为 &mut 借用。
    let cst_node = make_cst(unsafe { &mut *self.allocator });
    // `cast` 仅做 repr(C) 基址重合的类型视图转换（std 安全 API，恒可逆），
    // 与 cpp `static_cast<AstNode*>/static_cast<CstNode*>` 入 map 同形。
    self
      .cst_node_map
      .try_insert(ast_node.cast::<AstNode>(), cst_node.cast::<CstNode>());
  }

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
    // Safety: 同函数级注释；'b 的独占借用即调用方对该 arena 区间的独占证明。
    unsafe { cst_node_as::<T>(cst_node) }
  }
}
