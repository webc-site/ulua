use ulua_common::{fflag::DebugLuauUserDefinedClasses, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_name::AstName, ast_stat::AstStat,
    ast_stat_class::AstStatClass, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, cst_stat_local::CstStatLocal,
    lexeme::lexeme_name_is, location::Location, node_handle::Node, parser::Parser,
    position::Position,
  },
  rtti::ast_node_try_as_mut,
};

impl Parser {
  fn check_duplicate_export_value(&mut self, name: AstName, location: Location) -> bool {
    if self.declared_export_bindings.find(&name).is_some() {
      return false;
    }

    *self.declared_export_bindings.get_or_insert(name) = location;
    true
  }

  fn export_local_stat_value(
    &mut self,
    stat: *mut AstStat,
    keyword_position: Position,
  ) -> *mut AstStat {
    // `stat` 由 parse_local 产出、非空 arena 存活 `AstStat`（alloc 恒非空，失败中止）；
    // 句柄 get_mut 交出独占借用后 ast_node_try_as_mut 安全门面依 class_index 判型
    // 下转（未命中返回 None，命中即 repr(C) 基址重合下转为 `AstStatLocal`）。
    let mut stat_node = Node::from_raw(stat);
    if let Some(local_stat) = ast_node_try_as_mut::<AstStatLocal>(stat_node.get_mut()) {
      local_stat.is_exported = true;

      // cpp: `for (AstLocal* local : statLocal->vars) { ...; local->isExported = true; }`
      // vars 元素是 parse_local 经 push_local（arena alloc 恒非空、失败中止）产出的
      // 非空存活槽位：读侧 slot_ref 只读 name/location（借用按 NLL 于报错判断后结束），
      // 写侧句柄 get_mut 独占写 `is_exported`，与 cpp 逐元素写语义一致。
      for &local_ptr in local_stat.vars.iter() {
        let local = slot_ref(local_ptr);
        // cpp:2140-2145 逐 var 判重：重复仅 report 并 continue（该 var 不置
        // is_exported，其余 var 继续），不提前返回
        if !self.check_duplicate_export_value(local.name, local.location) {
          self.report(
            local.location,
            format_args!("Duplicate exported identifier '{}'", local.name),
          );
          continue;
        }

        Node::from_raw(local_ptr).get_mut().is_exported = true;
      }

      if self.options.store_cst_data {
        let cst_stat_local = self.lookup_cst_node_mut::<CstStatLocal>(&mut local_stat.base.base);
        LUAU_ASSERT!(cst_stat_local.is_some());
        if let Some(cst_stat_local) = cst_stat_local {
          cst_stat_local.declaration_keyword_position = keyword_position;
        }
      }
    } else {
      LUAU_ASSERT!(
        false,
        "Expected export local/const to parse as AstStatLocal"
      );
    }

    stat
  }

  pub fn parse_export_value(
    &mut self,
    start: &Location,
    keyword_position: Position,
    attributes: &AstArray<*mut AstAttr>,
  ) -> *mut AstStat {
    if self.function_stack.len() != 1 || self.recursion_counter != 1 {
      self.report(
        *start,
        format_args!("'export' may only be applied to top-level statements"),
      );
    }

    if self.has_module_return {
      self.report(
        *start,
        format_args!(
          "Exporting values is not compatible with top-level return (export/return conflict)"
        ),
      );
    }

    if !attributes.is_empty() && self.lexer.current().r#type != Type::RESERVED_FUNCTION {
      let current = *self.lexer.current();
      self.report(
        current.location,
        format_args!(
          "Expected 'function' after export declaration with attribute, but got {current} instead"
        ),
      );
    }

    if self.lexer.current().r#type == Type::RESERVED_LOCAL {
      let local_keyword_position = self.lexer.current().location.begin;

      if self.lexer.lookahead().r#type == Type::RESERVED_FUNCTION {
        self.report(
          *start,
          format_args!(
            "'export' must be followed by an identifier or 'function'; try removing 'local'"
          ),
        );
        // cpp:2170 错误恢复：仍解析为 local function（isLocalFunction=true）
        return self.parse_local(*start, local_keyword_position, &AstArray::EMPTY, true);
      }

      let stat = self.parse_local(*start, keyword_position, &AstArray::EMPTY, false);
      self.export_local_stat_value(stat, local_keyword_position)
    } else if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      let func_stat = self.parse_local(*start, keyword_position, attributes, true);
      // cpp `funcStat->as<AstStatLocalFunction>()`：判型与下转一步收敛——句柄
      // get_mut 交出独占借用后走安全门面 ast_node_try_as_mut（未命中返回 None，
      // 走 else 原样返回；命中即 repr(C) 基址重合下转正确）。
      let mut func_stat_node = Node::from_raw(func_stat);
      let Some(local_func) = ast_node_try_as_mut::<AstStatLocalFunction>(func_stat_node.get_mut())
      else {
        return func_stat;
      };

      // `local_func.name` 已句柄化为 Node<AstLocal>（类型层非空）：parse_local 的
      // local function 路径无条件以 `Some(&name)` 传入 parse_function_body、由其
      // push_local（arena alloc 恒非空，失败中止）建出；`get_mut` 的可变性沿上方
      // try_as_ptr_mut 派生的 `&mut` 继承，判重、报错与 is_exported/is_const 写入
      // 全走该安全引用，无裸指针重建。
      let name = local_func.name.get_mut();
      // cpp:2209-2215 重复仅 report，标志照常置位并返回原 stat
      if !self.check_duplicate_export_value(name.name, name.location) {
        self.report(
          name.location,
          format_args!("Duplicate exported identifier '{}'", name.name),
        );
      }

      name.is_exported = true;
      name.is_const = true;
      func_stat
    } else if lexeme_name_is(self.lexer.current(), "const") {
      let const_keyword_position = self.lexer.current().location.begin;
      self.next_lexeme();

      if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
        self.report(
          *start,
          format_args!("'export' must be followed by an identifier or 'function'"),
        );
        // cpp:2203 错误恢复：仍解析为 local function
        return self.parse_local(*start, const_keyword_position, &AstArray::EMPTY, true);
      }

      let stat = self.parse_local(*start, const_keyword_position, &AstArray::EMPTY, true);
      self.export_local_stat_value(stat, const_keyword_position)
    } else if DebugLuauUserDefinedClasses.get()
      && (lexeme_name_is(self.lexer.current(), "class")
        || lexeme_name_is(self.lexer.current(), "open"))
    {
      let open = lexeme_name_is(self.lexer.current(), "open");
      self.next_lexeme();
      if open {
        if !lexeme_name_is(self.lexer.current(), "class") {
          return self.report_stat_error(
            *start,
            AstArray::EMPTY,
            AstArray::EMPTY,
            format_args!("Incomplete statement: expected a class definition after 'open'"),
          );
        }
        self.next_lexeme(); // consume 'class' after 'open'
      }

      let stat = self.parse_class_stat(start, true, open);
      // cpp `parseClassStat` 结果判型后写 name：句柄 get_mut 交出独占借用后走
      // 安全门面 ast_node_try_as_mut（未命中返回 None，命中即 repr(C) 基址重合
      // 下转为 `AstStatClass`）。
      let mut stat_node = Node::from_raw(stat);
      if let Some(class_stat) = ast_node_try_as_mut::<AstStatClass>(stat_node.get_mut()) {
        // `class_stat.name` 是 parse_class_stat 无条件 `self.alloc(AstLocal{..})`
        // 写入 AstStatClass 的存活 `*mut AstLocal`（alloc 恒非空，失败中止）；句柄
        // get_mut 独占写，判重、报错与 is_exported 全走该安全引用。
        let mut name_node = Node::from_raw(class_stat.name);
        let name = name_node.get_mut();
        // cpp:2225-2229 重复仅 report，is_exported 照常置位并返回原 stat
        if !self.check_duplicate_export_value(name.name, name.location) {
          self.report(
            name.location,
            format_args!("Duplicate exported class '{}'", name.name),
          );
        }

        name.is_exported = true;
      }
      stat
    } else {
      self.report_stat_error(
        *start,
        AstArray::EMPTY,
        AstArray::EMPTY,
        format_args!("'export' must be followed by an identifier or 'function'"),
      )
    }
  }
}
