use core::ptr::{NonNull, from_mut};

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::node_opt,
  methods::ast_expr_function_ast_expr_function::AstExprFunctionArgs,
  records::{
    ast_array::AstArray,
    ast_attr::AstAttr,
    ast_expr_function::AstExprFunction,
    ast_local::AstLocal,
    ast_name::AstName,
    ast_node::AstNode,
    ast_type_pack::AstTypePack,
    binding::Binding,
    cst_expr_function::CstExprFunction,
    cst_node::CstNode,
    function::Function,
    lexeme::Lexeme,
    location::Location,
    match_lexeme::MatchLexeme,
    name::Name,
    node_handle::{Node, Nodes, OptNode},
    parser::Parser,
    position::Position,
    temp_vector::TempVector,
  },
};

impl Parser {
  /// cpp `Parser::parseFunctionBody`（`Parser.cpp:2273`）。
  ///
  /// 元组第二项是 `local function` 形态下为函数名建的控制变量：cpp 仅在传入
  /// `localName` 时 `pushLocal`，否则保持 `nullptr`（`Parser.cpp:2341-2346`），
  /// 故用 `Option<NonNull<AstLocal>>`；匿名函数即 `None`。
  pub fn parse_function_body(
    &mut self,
    hasself: bool,
    match_function: &Lexeme,
    debugname: &AstName,
    local_name: Option<&Name>,
    attributes: &AstArray<*mut AstAttr>,
    is_const: bool,
  ) -> (*mut AstExprFunction, Option<NonNull<AstLocal>>) {
    let mut start = match_function.location;
    start = self.first_attr_location(attributes, start);

    // CST 出参槽用 Option 表达「CST 记录是否开启」，cpp 置 null 关闭记录的形态在 Rust 侧消失。
    let mut cst_node: Option<&mut CstExprFunction> = if self.options.store_cst_data {
      // Safety: self.alloc 为 arena 分配且恒非空（失败 handle_alloc_error 中止），bump 落位后
      // 地址永不移动；本函数独占该 CST 节点，重建 `&mut` 无别名冲突。
      Some(unsafe { &mut *self.alloc(CstExprFunction::new()) })
    } else {
      None
    };

    let (generics, generic_packs) = match &mut cst_node {
      Some(cst) => {
        // 先做一层局部再借用，随后两个 disjoint 字段借位互不重叠。
        let cst = &mut **cst;
        let mut local_comma_positions = TempVector::new(&mut self.scratch_position);
        let res = self.parse_generic_type_list(
          false,
          Some(&mut cst.open_generics_position),
          Some(&mut local_comma_positions),
          Some(&mut cst.close_generics_position),
        );
        // copy_temp_vector_t 从 self 的 scratch arena 拷出新数组后整体写入出参槽字段，
        // 写位与 self 借用不重叠（cst 指向 arena CST 节点，独立于 self 的借用）。
        cst.generics_comma_positions = self.copy_temp_vector_t(&local_comma_positions);
        res
      }
      None => self.parse_generic_type_list(false, None, None, None),
    };

    let match_paren = MatchLexeme::new(self.lexer.current());
    self.expect_and_consume_char('(', "function");

    self.match_recovery_stop_on_token[')' as usize] += 1;

    let mut args = TempVector::new(&mut self.scratch_binding);

    let mut vararg = false;
    let mut vararg_location = Location::default();
    let mut vararg_annotation: Option<NonNull<AstTypePack>> = None;

    if self.lexer.current().r#type != Type::RPAREN {
      let res = match &mut cst_node {
        Some(cst) => {
          // 先做一层局部再借用，随后两个 disjoint 字段借位互不重叠；
          // Some 槽位表达调用方要记录 CST 位置。
          let cst = &mut **cst;
          self.parse_binding_list(
            &mut args,
            true,
            Some(&mut cst.args_comma_positions),
            None,
            Some(&mut cst.vararg_annotation_colon_position),
            false,
          )
        }
        None => self.parse_binding_list(&mut args, true, None, None, None, false),
      };
      vararg = res.0;
      vararg_location = res.1;
      vararg_annotation = res.2;
    }

    let mut arg_location: Option<Location> = None;
    if match_paren.type_ == Type::LPAREN && self.lexer.current().r#type == Type::RPAREN {
      arg_location = Some(Location::new(
        match_paren.position,
        self.lexer.current().location.end,
      ));
    }

    self.expect_match_and_consume(')', &match_paren, true);

    self.match_recovery_stop_on_token[')' as usize] -= 1;

    let typelist = self.parse_optional_return_type(match &mut cst_node {
      // 单字段再借用：Some 槽位表达调用方要记录 return 标注冒号位置。
      Some(cst) => Some(&mut cst.return_specifier_position),
      None => None,
    });
    let mut fun_local: Option<NonNull<AstLocal>> = None;
    if let Some(local_name) = local_name {
      // cpp `Binding(*localName, nullptr, {0, 0}, isConst)`（Parser.cpp:2345）：
      // `local function f()` 的名字绑定没有类型标注，`None` 即那个 `nullptr`。
      let binding = Binding::new(*local_name, None, Position::new(0, 0), is_const);
      fun_local = node_opt(self.push_local(&binding));
    }

    let locals_begin = self.save_locals();

    let mut fun = Function::new();
    fun.vararg = vararg;

    self.function_stack.push(fun);

    let (self_, vars) = self.prepare_function_arguments(&start, hasself, &args);

    let body = self.parse_block();

    self.function_stack.pop();

    self.restore_locals(locals_begin);

    let end = self.lexer.current().location;

    let has_end =
      self.expect_match_end_and_consume(Type::RESERVED_END, &MatchLexeme::new(match_function));
    // Safety: `body` 由 parse_block 返回、指向 arena 存活的 `AstStatBlock`（非空、bump 地址稳定、
    // 此刻仅本指针可达），写入其 `has_end` 字段不与其它借用重叠（parser 独占该 arena 节点）。
    unsafe {
      (*body).has_end = has_end;
    }

    let node = self.alloc(AstExprFunction::new(AstExprFunctionArgs {
      location: Location::new(start.begin, end.end),
      // 主干字段句柄化（records::node_handle）：scratch 的 arena 槽在此单点转成
      // 类型化句柄；`body` 的 has_end 写穿已在上方以裸指针完成，句柄建立后不再
      // 有指向该块的裸写。
      attributes: Nodes::from_raw_slice(attributes.as_slice()),
      generics: Nodes::from_raw_slice(generics.as_slice()),
      generic_packs: Nodes::from_raw_slice(generic_packs.as_slice()),
      self_: OptNode::from_non_null(self_),
      args: Nodes::from_raw_slice(vars.as_slice()),
      vararg,
      vararg_location,
      body: Node::from_raw(body),
      function_depth: self.function_stack.len(),
      debugname: *debugname,
      return_annotation: OptNode::from_non_null(typelist),
      vararg_annotation: OptNode::from_non_null(vararg_annotation),
      arg_location,
    }));

    // CST 记录开启时上方必已建 Some 槽（store_cst_data 与 Some 严格同轨），判 Some 即判开关。
    if let Some(cst) = &mut cst_node {
      cst.function_keyword_position = match_function.location.begin;
      cst.args_annotation_colon_positions = self.extract_annotation_colon_positions(&args);
      let cst_ptr: *mut CstExprFunction = from_mut(&mut **cst);
      self
        .cst_node_map
        .try_insert(node.cast::<AstNode>(), cst_ptr.cast::<CstNode>());
    }

    (node, fun_local)
  }
}
