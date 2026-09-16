use core::ptr::null_mut;

use ulua_common::{FFlag::DebugLuauUserDefinedClasses, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_name::AstName, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_class::AstStatClass, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, cst_stat_local::CstStatLocal, lexeme::Type,
    location::Location, parser::Parser, position::Position,
  },
  rtti::{ast_node_as, ast_node_is, cst_node_as},
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
      unsafe {
        (*local_stat).is_exported = true;
      }

      let vars = unsafe { (*local_stat).vars };
      for &local in vars.iter() {
        // SAFETY: local 指向 arena 中存活的 AstLocal 节点。
        unsafe {
          if !self.check_duplicate_export_value((*local).name, (*local).location) {
            let stats = self.copy_initializer_list_t(&[stat]);
            return self.report_stat_error(
              (*local).location,
              AstArray::default(),
              stats,
              format_args!("Duplicate exported identifier '{}'", (*local).name),
            ) as *mut AstStat;
          }

          (*local).is_exported = true;
        }
      }

      if self.options.store_cst_data {
        let cst_stat_local = unsafe {
          let cst_node = self.cst_node_map.find(&(stat as *mut AstNode));
          if let Some(cst_node_ptr) = cst_node {
            cst_node_as::<CstStatLocal>(*cst_node_ptr)
          } else {
            null_mut()
          }
        };
        LUAU_ASSERT!(!cst_stat_local.is_null());
        if !cst_stat_local.is_null() {
          unsafe {
            (*cst_stat_local).declaration_keyword_position = keyword_position;
          }
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
      self.report_location_c_char_item(
        *start,
        format_args!("'export' may only be applied to top-level statements"),
      );
    }

    if self.has_module_return {
      self.report_location_c_char_item(
        *start,
        format_args!(
          "Exporting values is not compatible with top-level return (export/return conflict)"
        ),
      );
    }

    if attributes.size != 0 && self.lexer.current().r#type != Type::RESERVED_FUNCTION {
      let current = *self.lexer.current();
      self.report_location_c_char_item(
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
          AstArray::default(),
          AstArray::default(),
          format_args!(
            "'export' must be followed by an identifier or 'function'; try removing 'local'"
          ),
        ) as *mut AstStat;
      }

      let stat = self.parse_local(*start, keyword_position, &AstArray::default(), false);
      self.export_local_stat_value(stat, local_keyword_position)
    } else if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      let func_stat = self.parse_local(*start, keyword_position, attributes, true);
      if !ast_node_is::<AstStatLocalFunction>(func_stat as *mut AstNode) {
        return func_stat;
      }

      let func = func_stat as *mut AstStatLocalFunction;
      let name = unsafe { (*func).name };
      if !self.check_duplicate_export_value(unsafe { (*name).name }, unsafe { (*name).location }) {
        let stats = self.copy_initializer_list_t(&[func_stat]);
        return self.report_stat_error(
          unsafe { (*name).location },
          AstArray::default(),
          stats,
          format_args!("Duplicate exported identifier '{}'", unsafe {
            (*name).name
          }),
        ) as *mut AstStat;
      }

      unsafe {
        (*name).is_exported = true;
        (*name).is_const = true;
      }
      func_stat
    } else if self.lexer.current().r#type == Type::NAME && self.lexer.current().name() == "const" {
      let const_keyword_position = self.lexer.current().location.begin;
      self.next_lexeme();

      if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
        return self.report_stat_error(
          *start,
          AstArray::default(),
          AstArray::default(),
          format_args!("'export' must be followed by an identifier or 'function'"),
        ) as *mut AstStat;
      }

      let stat = self.parse_local(*start, const_keyword_position, &AstArray::default(), true);
      self.export_local_stat_value(stat, const_keyword_position)
    } else if DebugLuauUserDefinedClasses.get()
      && self.lexer.current().r#type == Type::NAME
      && (self.lexer.current().name() == "class" || self.lexer.current().name() == "open")
    {
      let open = self.lexer.current().name() == "open";
      self.next_lexeme();
      if open {
        if self.lexer.current().r#type != Type::NAME || self.lexer.current().name() != "class" {
          return self.report_stat_error(
            *start,
            AstArray::default(),
            AstArray::default(),
            format_args!("Incomplete statement: expected a class definition after 'open'"),
          ) as *mut AstStat;
        }
        self.next_lexeme(); // consume 'class' after 'open'
      }

      let stat = self.parse_class_stat(start, true, open);
      let class_stat = stat as *mut AstStatClass;
      if !class_stat.is_null() {
        let name = unsafe { (*class_stat).name };
        if !self.check_duplicate_export_value(unsafe { (*name).name }, unsafe { (*name).location })
        {
          let stats = self.copy_initializer_list_t(&[stat]);
          return self.report_stat_error(
            unsafe { (*name).location },
            AstArray::default(),
            stats,
            format_args!("Duplicate exported class '{}'", unsafe { (*name).name }),
          ) as *mut AstStat;
        }

        unsafe {
          (*name).is_exported = true;
        }
      }
      stat
    } else {
      self.report_stat_error(
        *start,
        AstArray::default(),
        AstArray::default(),
        format_args!("'export' must be followed by an identifier or 'function'"),
      ) as *mut AstStat
    }
  }
}
