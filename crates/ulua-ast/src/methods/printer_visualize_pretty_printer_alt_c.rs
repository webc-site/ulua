//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。CST 侧经 `lookup_cst_node` 返回 `Option<&T>`，
//! 判空与字段读取全部走安全代码。
//!
//! 节点子指针不再在调用点解引用：直接传裸指针给 `visualize_*`（`IntoNodePtr`
//! 归一），unsafe 收口在被调函数入口；`AstArray<*mut T>` 遍历统一走
//! `iter_nodes`；块体末尾的 `advance(body.end)` 提前读为局部值（visualize
//! 只写 writer，AST 不可变，读取与副作用无依赖）。

use ulua_common::{LUAU_ASSERT, fflag, records::variant::Variant2};

use crate::{
  records::{
    ast_array::AstArray,
    ast_class_method::AstClassMethod,
    ast_class_property::AstClassProperty,
    ast_expr_binary::AstExprBinaryOp,
    ast_local::AstLocal,
    ast_stat::AstStat,
    ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue,
    ast_stat_declare_global::AstStatDeclareGlobal,
    ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction,
    ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction,
    ast_stat_while::AstStatWhile,
    comma_separator_inserter::CommaSeparatorInserter,
    cst_generic_type::CstGenericType,
    cst_generic_type_pack::CstGenericTypePack,
    cst_stat_assign::CstStatAssign,
    cst_stat_compound_assign::CstStatCompoundAssign,
    cst_stat_do::CstStatDo,
    cst_stat_for::CstStatFor,
    cst_stat_for_in::CstStatForIn,
    cst_stat_function::CstStatFunction,
    cst_stat_local::CstStatLocal,
    cst_stat_local_function::CstStatLocalFunction,
    cst_stat_repeat::CstStatRepeat,
    cst_stat_return::CstStatReturn,
    cst_stat_type_alias::CstStatTypeAlias,
    cst_stat_type_function::CstStatTypeFunction,
    location::Location,
    position::{EMPTY_POSITIONS, Position},
    printer::{IntoNodePtr, Printer},
    writer::Writer,
  },
  rtti::{ast_node_is, ast_node_try_as},
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_ast_stat<S: IntoNodePtr<AstStat>>(&mut self, program: S) {
    // SAFETY: program 指向 arena 中存活的 AstStat 派生节点
    let program = unsafe { &*program.into_node_ptr() };
    // cpp 侧 `AstNode* node = program` 的只读形态：下转与 CST 查表都以共享借用
    // 为入参（打印器只写 Writer，从不写节点）。
    let node = &program.base;
    self.advance(program.base.location.begin);

    if let Some(block) = ast_node_try_as::<AstStatBlock>(node) {
      if let Some(cst) = self.lookup_cst_node::<CstStatDo>(node) {
        self.writer.keyword("do");
        self.advance(cst.stats_start_position);
        for s in AstArray::iter(&block.body) {
          self.visualize_ast_stat(s);
        }
        self.maybe_advance_and_write(&cst.end_position, "end", false);
      } else {
        for s in AstArray::iter(&block.body) {
          self.visualize_ast_stat(s);
        }
        self.advance(block.base.base.location.end);
        self.write_end(&program.base.location);
      }
    } else if let Some(a) = ast_node_try_as::<AstStatIf>(node) {
      self.writer.keyword("if");
      self.visualize_else_if(a);
    } else if let Some(a) = ast_node_try_as::<AstStatWhile>(node) {
      self.writer.keyword("while");
      self.visualize_ast_expr(a.condition);
      self.visualize_do_block(a.do_location, a.body);
    } else if let Some(a) = ast_node_try_as::<AstStatRepeat>(node) {
      self.writer.keyword("repeat");
      self.visualize_block_ast_stat_block(a.body);
      match self.lookup_cst_node::<CstStatRepeat>(node) {
        Some(cst) => self.maybe_advance_and_write(&cst.until_position, "until", false),
        None => {
          // SAFETY: condition 指向 arena 存活的 AstExpr
          self.advance_before(unsafe { &*a.condition }.base.location.begin, 6);
          self.writer.keyword("until");
        }
      }
      self.visualize_ast_expr(a.condition);
    } else if ast_node_is::<AstStatBreak>(node) {
      self.writer.keyword("break");
    } else if ast_node_is::<AstStatContinue>(node) {
      self.writer.keyword("continue");
    } else if let Some(a) = ast_node_try_as::<AstStatReturn>(node) {
      self.writer.keyword("return");
      let mut comma = CommaSeparatorInserter::new(
        self
          .lookup_cst_node::<CstStatReturn>(node)
          .map_or(EMPTY_POSITIONS, |cst| cst.comma_positions.as_slice()),
      );
      for expr in AstArray::iter(&a.list) {
        comma.operator_call(self.writer);
        self.visualize_ast_expr(expr);
      }
    } else if let Some(a) = ast_node_try_as::<AstStatExpr>(node) {
      self.visualize_ast_expr(a.expr);
    } else if let Some(a) = ast_node_try_as::<AstStatLocal>(node) {
      let cst_node = self.lookup_cst_node::<CstStatLocal>(node);
      if fflag::LuauExportValueSyntax.get() && a.is_exported {
        self.writer.keyword("export");
        if let Some(cst) = cst_node {
          self.advance(cst.declaration_keyword_position);
        }
      }
      self
        .writer
        .keyword(if a.is_const { "const" } else { "local" });
      let mut var_comma = CommaSeparatorInserter::new(
        cst_node.map_or(EMPTY_POSITIONS, |cst| cst.vars_comma_positions.as_slice()),
      );
      self.visualize_local_vars(
        &a.vars,
        &mut var_comma,
        cst_node.map(|cst| cst.vars_annotation_colon_positions.as_slice()),
      );
      if let Some(loc) = a.equals_sign_location {
        self.advance(loc.begin);
        self.writer.symbol("=");
      }
      let mut value_comma = CommaSeparatorInserter::new(
        cst_node.map_or(EMPTY_POSITIONS, |cst| cst.values_comma_positions.as_slice()),
      );
      for value in AstArray::iter(&a.values) {
        value_comma.operator_call(self.writer);
        self.visualize_ast_expr(value);
      }
    } else if let Some(a) = ast_node_try_as::<AstStatFor>(node) {
      let cst_node = self.lookup_cst_node::<CstStatFor>(node);
      self.writer.keyword("for");
      self.visualize_ast_local_position(
        // SAFETY: var 指向 arena 存活的 AstLocal
        unsafe { &*a.var },
        cst_node.map_or_else(Position::missing, |cst| cst.annotation_colon_position),
      );
      if let Some(cst) = cst_node {
        self.advance(cst.equals_position);
      }
      self.writer.symbol("=");
      self.visualize_ast_expr(a.from);
      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.end_comma_position), ",");
      self.visualize_ast_expr(a.to);
      if !a.step.is_null() {
        if let Some(cst) = cst_node {
          self.advance(cst.step_comma_position);
        }
        self.writer.symbol(",");
        self.visualize_ast_expr(a.step);
      }
      self.visualize_do_block(a.do_location, a.body);
    } else if let Some(a) = ast_node_try_as::<AstStatForIn>(node) {
      let cst_node = self.lookup_cst_node::<CstStatForIn>(node);
      self.writer.keyword("for");
      let mut var_comma = CommaSeparatorInserter::new(
        cst_node.map_or(EMPTY_POSITIONS, |cst| cst.vars_comma_positions.as_slice()),
      );
      self.visualize_local_vars(
        &a.vars,
        &mut var_comma,
        cst_node.map(|cst| cst.vars_annotation_colon_positions.as_slice()),
      );
      self.advance(a.in_location.begin);
      self.writer.keyword("in");
      let mut val_comma = CommaSeparatorInserter::new(
        cst_node.map_or(EMPTY_POSITIONS, |cst| cst.values_comma_positions.as_slice()),
      );
      for val in AstArray::iter(&a.values) {
        val_comma.operator_call(self.writer);
        self.visualize_ast_expr(val);
      }
      self.advance(a.do_location.begin);
      self.writer.keyword("do");
      // SAFETY: body 指向 arena 存活的 AstStatBlock；end 为 Copy 值，提前读取
      // 与末尾读取等价
      let body_end = unsafe { &*a.body }.base.base.location.end;
      self.visualize_block_ast_stat_block(a.body);
      self.advance(body_end);
      self.writer.keyword("end");
    } else if let Some(a) = ast_node_try_as::<AstStatAssign>(node) {
      let cst_node = self.lookup_cst_node::<CstStatAssign>(node);
      let mut var_comma = CommaSeparatorInserter::new(
        cst_node.map_or(EMPTY_POSITIONS, |cst| cst.vars_comma_positions.as_slice()),
      );
      for var in AstArray::iter(&a.vars) {
        var_comma.operator_call(self.writer);
        self.visualize_ast_expr(var);
      }
      match cst_node {
        Some(cst) => self.maybe_advance_and_write(&cst.equals_position, "=", false),
        None => {
          self.writer.space();
          self.writer.symbol("=");
        }
      }
      let mut value_comma = CommaSeparatorInserter::new(
        cst_node.map_or(EMPTY_POSITIONS, |cst| cst.values_comma_positions.as_slice()),
      );
      for value in AstArray::iter(&a.values) {
        value_comma.operator_call(self.writer);
        self.visualize_ast_expr(value);
      }
    } else if let Some(a) = ast_node_try_as::<AstStatCompoundAssign>(node) {
      let cst_node = self.lookup_cst_node::<CstStatCompoundAssign>(node);
      self.visualize_ast_expr(a.var);
      if let Some(cst) = cst_node {
        self.advance(cst.op_position);
      }
      let symbol = match a.op {
        AstExprBinaryOp::Add => "+=",
        AstExprBinaryOp::Sub => "-=",
        AstExprBinaryOp::Mul => "*=",
        AstExprBinaryOp::Div => "/=",
        AstExprBinaryOp::FloorDiv => "//=",
        AstExprBinaryOp::Mod => "%=",
        AstExprBinaryOp::Pow => "^=",
        AstExprBinaryOp::Concat => "..=",
        _ => {
          LUAU_ASSERT!(false);
          ""
        }
      };
      // 操作符宽度决定与右操作数的间距（cpp 每分支显式 2/3，symbol.len 同值）
      if !symbol.is_empty() {
        if cst_node.is_none() {
          // SAFETY: value 指向 arena 存活的 AstExpr
          self.writer.maybe_space(
            &unsafe { &*a.value }.base.location.begin,
            symbol.len() as i32,
          );
        }
        self.writer.symbol(symbol);
      }
      self.visualize_ast_expr(a.value);
    } else if let Some(a) = ast_node_try_as::<AstStatFunction>(node) {
      // SAFETY: func 指向 arena 存活的 AstExprFunction
      let func = unsafe { &*a.func };
      for attr in func.attributes.iter_nodes() {
        self.visualize_attribute(attr);
      }
      if let Some(cst) = self.lookup_cst_node::<CstStatFunction>(node) {
        self.advance(cst.function_keyword_position);
      }
      self.writer.keyword("function");
      self.visualize_ast_expr(a.name);
      self.visualize_function_body(func);
    } else if let Some(a) = ast_node_try_as::<AstStatLocalFunction>(node) {
      // SAFETY: func 指向 arena 存活的 AstExprFunction
      let func = unsafe { &*a.func };
      for attr in func.attributes.iter_nodes() {
        self.visualize_attribute(attr);
      }
      let cst_node = self.lookup_cst_node::<CstStatLocalFunction>(node);
      if let Some(cst) = cst_node {
        self.advance(cst.local_keyword_position);
      }
      // SAFETY: name 指向 arena 存活的 AstLocal
      let name = unsafe { &*a.name };
      if fflag::LuauExportValueSyntax.get() && name.is_exported {
        self.writer.keyword("export");
      } else if name.is_const {
        self.writer.keyword("const");
      } else {
        self.writer.keyword("local");
      }
      match cst_node {
        Some(cst) => self.advance(cst.function_keyword_position),
        None => self.writer.space(),
      }
      self.writer.keyword("function");
      self.advance(name.location.begin);
      self.writer.identifier(name.name.as_bytes());
      self.visualize_function_body(func);
    } else if let Some(a) = ast_node_try_as::<AstStatTypeAlias>(node) {
      if self.write_types {
        let cst_node = self.lookup_cst_node::<CstStatTypeAlias>(node);
        if a.exported {
          self.writer.keyword("export");
        }
        if let Some(cst) = cst_node {
          self.advance(cst.type_keyword_position);
        }
        self.writer.keyword("type");
        self.advance(a.name_location.begin);
        self.writer.identifier(a.name.as_bytes());
        if a.generics.size > 0 || a.generic_packs.size > 0 {
          if let Some(cst) = cst_node {
            self.advance(cst.generics_open_position);
          }
          self.writer.symbol("<");
          let mut comma = CommaSeparatorInserter::new(cst_node.map_or(EMPTY_POSITIONS, |cst| {
            cst.generics_comma_positions.as_slice()
          }));
          for o in a.generics.iter_nodes() {
            comma.operator_call(self.writer);
            self.writer.advance(&o.base.location.begin);
            self.writer.identifier(o.name.as_bytes());
            if !o.default_value.is_null() {
              if let Some(cst) = self.lookup_cst_node::<CstGenericType>(&o.base) {
                self.advance(cst.default_equals_position);
              } else {
                // SAFETY: default_value 指向 arena 存活的 AstType
                self
                  .writer
                  .maybe_space(&unsafe { &*o.default_value }.base.location.begin, 2);
              }
              self.writer.symbol("=");
              self.visualize_type_annotation(o.default_value);
            }
          }
          for o in a.generic_packs.iter_nodes() {
            comma.operator_call(self.writer);
            let generic_type_pack_cst_node = self.lookup_cst_node::<CstGenericTypePack>(&o.base);
            self.writer.advance(&o.base.location.begin);
            self.writer.identifier(o.name.as_bytes());
            if let Some(cst) = generic_type_pack_cst_node {
              self.maybe_advance_and_write(&cst.ellipsis_position, "...", false);
            } else {
              self.writer.symbol("...");
            }
            if !o.default_value.is_null() {
              if let Some(cst) = generic_type_pack_cst_node {
                self.advance(cst.default_equals_position);
              } else {
                // SAFETY: default_value 指向 arena 存活的 AstTypePack
                self
                  .writer
                  .maybe_space(&unsafe { &*o.default_value }.base.location.begin, 2);
              }
              self.writer.symbol("=");
              self.visualize_type_pack_annotation(o.default_value, false, true, false);
            }
          }
          match cst_node {
            Some(cst) => self.maybe_advance_and_write(&cst.generics_close_position, ">", false),
            None => self.writer.symbol(">"),
          }
        }
        match cst_node {
          Some(cst) => self.maybe_advance_and_write(&cst.equals_position, "=", false),
          None => {
            // SAFETY: type_ptr 指向 arena 存活的 AstType
            self
              .writer
              .maybe_space(&unsafe { &*a.type_ptr }.base.location.begin, 2);
            self.writer.symbol("=");
          }
        }
        self.visualize_type_annotation(a.type_ptr);
      }
    } else if let Some(t) = ast_node_try_as::<AstStatTypeFunction>(node) {
      if self.write_types {
        let cst_node = self.lookup_cst_node::<CstStatTypeFunction>(node);
        if t.exported {
          self.writer.keyword("export");
        }
        match cst_node {
          Some(cst) => self.advance(cst.type_keyword_position),
          None => self.writer.space(),
        }
        self.writer.keyword("type");
        match cst_node {
          Some(cst) => self.advance(cst.function_keyword_position),
          None => self.writer.space(),
        }
        self.writer.keyword("function");
        self.advance(t.name_location.begin);
        self.writer.identifier(t.name.as_bytes());
        self.visualize_function_body(t.body);
      }
    } else if let Some(a) = ast_node_try_as::<AstStatError>(node) {
      self.writer.symbol("(error-stat");
      let no_statements = a.statements.is_empty();
      for (i, &expression) in a.expressions.iter().enumerate() {
        self
          .writer
          .symbol(if i == 0 && no_statements { ": " } else { ", " });
        self.visualize_ast_expr(expression);
      }
      let no_expressions = a.expressions.is_empty();
      for (i, &statement) in a.statements.iter().enumerate() {
        self
          .writer
          .symbol(if i == 0 && no_expressions { ": " } else { ", " });
        self.visualize_ast_stat(statement);
      }
      self.writer.symbol(")");
    } else if let Some(a) = ast_node_try_as::<AstStatDeclareGlobal>(node) {
      self.writer.keyword("declare");
      self.advance(a.name_location.begin);
      self.writer.identifier(a.name.as_bytes());
      self.writer.symbol(":");
      self.visualize_type_annotation(a.type_);
    } else if let Some(c) = ast_node_try_as::<AstStatClass>(node) {
      if fflag::DebugLuauUserDefinedClasses.get() {
        self.writer.keyword("class");
        // SAFETY: name 指向 arena 存活的 AstName 表项（此处为 AstLocal 名字）
        let name = unsafe { &*c.name };
        self.advance(name.location.begin);
        self.writer.identifier(name.name.as_bytes());
        for member in AstArray::iter(&c.members) {
          match member {
            Variant2::V0(prop) => {
              let prop: &AstClassProperty = prop;
              self.advance(prop.qualifier_location.begin);
              self.writer.keyword("public");
              self.advance(prop.name_location.begin);
              self.writer.identifier(prop.name.as_bytes());
              if self.write_types && !prop.ty.is_null() {
                LUAU_ASSERT!(prop.type_colon_location.is_some());
                // 冒号缺失仅解析器 bug（断言同一不变式）；if let 免 panic
                //（cpp 侧解引用 optional 同样依赖断言）。
                if let Some(colon) = prop.type_colon_location {
                  self.advance(colon.begin);
                }
                self.writer.symbol(":");
                self.visualize_type_annotation(prop.ty);
              }
            }
            Variant2::V1(method) => {
              let method: &AstClassMethod = method;
              if let Some(qualifier_location) = method.qualifier_location {
                self.advance(qualifier_location.begin);
                self.writer.keyword("public");
              }
              self.advance(method.keyword_location.begin);
              self.writer.keyword("function");
              self.advance(method.name_location.begin);
              self.writer.identifier(method.function_name.as_bytes());
              self.visualize_function_body(method.function);
            }
          }
        }
        self.writer.newline();
        self.writer.keyword("end");
        self.writer.newline();
      }
    } else {
      LUAU_ASSERT!(false);
    }

    if program.has_semicolon {
      self.advance_before(program.base.location.end, 1);
      self.writer.symbol(";");
    }
  }

  /// `do <block> end` 收尾：while / for / for-in 三处共用（cpp 在三个分支里各
  /// 抄了一遍同一段落），unsafe 解引用也从 3 处收口到 1 处。
  fn visualize_do_block(&mut self, do_location: Location, body: *mut AstStatBlock) {
    self.advance(do_location.begin);
    self.writer.keyword("do");
    // SAFETY: body 指向 arena 存活的 AstStatBlock；end 为 Copy 值，visualize
    // 只写 writer，提前读取与末尾读取等价
    let body_end = unsafe { &*body }.base.base.location.end;
    self.visualize_block_ast_stat_block(body);
    self.advance(body_end);
    self.writer.keyword("end");
  }

  /// 遍历局部变量列表，逐个写逗号分隔符与本地位置；`colon_positions` 为
  /// CST 提供的冒号位置（无 CST 时用 `Position::missing()`，cpp 同款
  /// `Position{0, 0}` 分支此处以 missing 收口）。
  fn visualize_local_vars(
    &mut self,
    vars: &AstArray<*mut AstLocal>,
    var_comma: &mut CommaSeparatorInserter,
    colon_positions: Option<&[Position]>,
  ) {
    for (i, var) in vars.iter_nodes().enumerate() {
      var_comma.operator_call(self.writer);
      // vars 与 colon_positions 成对构造（解析器保证等长，cpp 同处有断言）；
      // 越界仅解析器 bug，退化 missing（cpp 侧 `data[i]` 同样依赖该不变式）。
      let colon_position = colon_positions
        .and_then(|c| c.get(i))
        .copied()
        .unwrap_or_else(Position::missing);
      self.visualize_ast_local_position(var, colon_position);
    }
  }
}
