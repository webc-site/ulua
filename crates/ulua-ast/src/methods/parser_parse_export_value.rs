use ulua_common::{fflag::DebugLuauUserDefinedClasses, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_name::AstName, ast_stat::AstStat,
    ast_stat_class::AstStatClass, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, cst_stat_local::CstStatLocal,
    lexeme::lexeme_name_is, location::Location, parser::Parser, position::Position,
  },
  rtti::ast_node_try_as_ptr_mut,
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
    // Safety: `stat` 由 parse_local 产出、非空 arena 存活 `AstStat`；ast_node_try_as_ptr_mut 依
    // class_index 判定，未命中返回 None，命中即 repr(C) 基址重合下转为 `AstStatLocal`，此刻 parser 独占该 arena。
    if let Some(local_stat) = unsafe { ast_node_try_as_ptr_mut::<AstStatLocal>(stat) } {
      local_stat.is_exported = true;

      // cpp: `for (AstLocal* local : statLocal->vars) { ...; local->isExported = true; }`
      // —— 逐元素取裸指针副本（iter().copied()），只读判断用 `&*p`、唯一的写入
      // `is_exported` 用 `(*p).…`：`iter_nodes()` 只能给 `&T`，而写穿必须走裸指针。
      for local_ptr in local_stat.vars.iter().copied() {
        // `vars` 元素是 parse_local 写入的非空 arena 存活槽位；此只读引用仅用于
        // 读 name/location，随后 is_exported 写入走裸指针，二者不同时存活，无别名冲突。
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

        // Safety: `vars` 元素是 parse_local 经 push_local（arena alloc 恒非空、失败中止）产出的存活
        // `*mut AstLocal`；上方 `local` 只读借用末次用于报错判断后已按 NLL 结束，此处经裸指针独占写
        // `is_exported`，二者不同时存活、无别名冲突（与 cpp 逐元素写 local->isExported 语义一致）。
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
      // cpp `funcStat->as<AstStatLocalFunction>()`：判型与下转一步收敛，
      // 不再先 `ast_node_is` 再手写未检下转指针。
      // Safety: `func_stat` 由 parse_local 产出、非空 arena 存活 `AstStat`；try_as_ptr_mut 依 class_index
      // 判定，未命中返回 None（走 else 原样返回），命中即 repr(C) 基址重合下转正确，parser 独占该 arena。
      let Some(local_func) =
        (unsafe { ast_node_try_as_ptr_mut::<AstStatLocalFunction>(func_stat) })
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
      // cpp `parseClassStat` 结果判型后写 name：判空 + 未检下转收敛为
      // try_as_ptr_mut 一步（repr(C) 家族上转仅改视图，判型仍按 class index）。
      // Safety: `stat` 由 parse_class_stat 产出、非空 arena 存活 `AstStat`；try_as_ptr_mut 依 class_index
      // 判定，未命中返回 None，命中即 repr(C) 基址重合下转为 `AstStatClass`，parser 独占该 arena。
      if let Some(class_stat) = unsafe { ast_node_try_as_ptr_mut::<AstStatClass>(stat) } {
        // Safety: `class_stat.name` 是 parse_class_stat 无条件 `self.alloc(AstLocal{..})` 写入 AstStatClass 的
        // 存活 `*mut AstLocal`（alloc 恒非空，失败 handle_alloc_error 中止；bump 地址稳定）；判型命中后节点
        // 为 parser 独占，重建 `&mut` 无别名，判重、报错与 is_exported 写入全走该安全引用。
        let name = unsafe { &mut *class_stat.name };
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
