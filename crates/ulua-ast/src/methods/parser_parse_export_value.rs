use ulua_common::{fflag::DebugLuauUserDefinedClasses, macros::luau_assert::LUAU_ASSERT};

use crate::{
  methods::lexeme_name_is::lexeme_name_is,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_name::AstName, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_class::AstStatClass, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, cst_stat_local::CstStatLocal, lexeme::Type,
    location::Location, parser::Parser, position::Position,
  },
  rtti::{ast_node_as, ast_node_is},
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
    let local_stat = unsafe { ast_node_as::<AstStatLocal>(stat as *mut AstNode) };
    if !local_stat.is_null() {
      // SAFETY: local_stat 判空后指向 arena 存活的 AstStatLocal
      let local_stat = unsafe { &mut *local_stat };
      local_stat.is_exported = true;

      // cpp: `for (AstLocal* local : statLocal->vars) { ...; local->isExported = true; }`
      // —— 逐元素取裸指针副本（iter().copied()），只读判断用 `&*p`、唯一的写入
      // `is_exported` 用 `(*p).…`：`iter_nodes()` 只能给 `&T`，而写穿必须走裸指针。
      for local_ptr in local_stat.vars.iter().copied() {
        let local = unsafe { &*local_ptr };
        if !self.check_duplicate_export_value(local.name, local.location) {
          let stats = self.copy_initializer_list_t(&[stat]);
          return self.report_stat_error(
            local.location,
            AstArray::EMPTY,
            stats,
            format_args!("Duplicate exported identifier '{}'", local.name),
          ) as *mut AstStat;
        }

        // SAFETY: 元素指向 arena 存活的 AstLocal，解析期该 arena 独占
        unsafe { (*local_ptr).is_exported = true };
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
        return self.report_stat_error(
          *start,
          AstArray::EMPTY,
          AstArray::EMPTY,
          format_args!(
            "'export' must be followed by an identifier or 'function'; try removing 'local'"
          ),
        ) as *mut AstStat;
      }

      let stat = self.parse_local(*start, keyword_position, &AstArray::EMPTY, false);
      self.export_local_stat_value(stat, local_keyword_position)
    } else if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      let func_stat = self.parse_local(*start, keyword_position, attributes, true);
      if !ast_node_is::<AstStatLocalFunction>(func_stat as *mut AstNode) {
        return func_stat;
      }

      // SAFETY: func_stat 判型后 name 指向 arena 存活的 AstLocal；&mut 引用
      // 化，判重、报错与 is_exported/is_const 写入全走安全代码
      let name = unsafe { &mut *((*(func_stat as *mut AstStatLocalFunction)).name) };
      if !self.check_duplicate_export_value(name.name, name.location) {
        let stats = self.copy_initializer_list_t(&[func_stat]);
        return self.report_stat_error(
          name.location,
          AstArray::EMPTY,
          stats,
          format_args!("Duplicate exported identifier '{}'", name.name),
        ) as *mut AstStat;
      }

      name.is_exported = true;
      name.is_const = true;
      func_stat
    } else if lexeme_name_is(self.lexer.current(), "const") {
      let const_keyword_position = self.lexer.current().location.begin;
      self.next_lexeme();

      if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
        return self.report_stat_error(
          *start,
          AstArray::EMPTY,
          AstArray::EMPTY,
          format_args!("'export' must be followed by an identifier or 'function'"),
        ) as *mut AstStat;
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
          ) as *mut AstStat;
        }
        self.next_lexeme(); // consume 'class' after 'open'
      }

      let stat = self.parse_class_stat(start, true, open);
      let class_stat = stat as *mut AstStatClass;
      if !class_stat.is_null() {
        // SAFETY: class_stat 判空后 name 指向 arena 存活的 AstLocal；&mut
        // 引用化，判重、报错与 is_exported 写入全走安全代码
        let name = unsafe { &mut *((*class_stat).name) };
        if !self.check_duplicate_export_value(name.name, name.location) {
          let stats = self.copy_initializer_list_t(&[stat]);
          return self.report_stat_error(
            name.location,
            AstArray::EMPTY,
            stats,
            format_args!("Duplicate exported class '{}'", name.name),
          ) as *mut AstStat;
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
      ) as *mut AstStat
    }
  }
}
