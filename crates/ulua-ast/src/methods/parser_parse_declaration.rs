use core::ptr::NonNull;

use ulua_common::fflag;

use crate::{
  enums::{ast_table_access::AstTableAccess, type_lexer::Type},
  functions::optional_node::{node_opt, opt_node, slot_ref},
  records::{
    ast_array::AstArray, ast_attr::AstAttr,
    ast_declared_extern_type_property::AstDeclaredExternTypeProperty, ast_name::AstName,
    ast_stat::AstStat, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_table_indexer::AstTableIndexer,
    ast_type_list::AstTypeList, ast_type_pack_explicit::AstTypePackExplicit, location::Location,
    match_lexeme::MatchLexeme, parser::Parser, temp_vector::TempVector,
  },
  rtti::AstNodeClass,
};

impl Parser {
  pub fn parse_declaration(
    &mut self,
    start: &Location,
    attributes: &AstArray<*mut AstAttr>,
  ) -> *mut AstStat {
    // `declare` token is already parsed at this point

    if !attributes.is_empty() && self.lexer.current().r#type != Type::RESERVED_FUNCTION {
      let current = *self.lexer.current();
      return self.report_stat_error(
        current.location,
        AstArray::EMPTY,
        AstArray::EMPTY,
        format_args!(
          "Expected a function type declaration after attribute, but got {current} instead",
        ),
      );
    }

    if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      self.next_lexeme();

      let global_name = self.parse_name("global function name");
      let (generics, generic_packs) = self.parse_generic_type_list(false, None, None, None);

      let match_paren = MatchLexeme::new(self.lexer.current());

      self.expect_and_consume_type(Type::LPAREN, "global function declaration");

      let mut args = TempVector::new(&mut self.scratch_binding);

      let mut vararg = false;
      let mut vararg_location = Location::default();
      // cpp `AstTypePack* varargAnnotation = nullptr`（Parser.cpp:1837）：无 `...` 尾注即 None。
      let mut vararg_annotation = None;

      if self.lexer.current().r#type != Type::RPAREN {
        // C++ `auto [vararg, varargLocation, varargAnnotation] = ...`：元组解构
        (vararg, vararg_location, vararg_annotation) =
          self.parse_binding_list(&mut args, true, None, None, None, false);
      }

      self.expect_match_and_consume(')', &match_paren, false);

      // cpp `if (!retTypes) retTypes = alloc<AstTypePackExplicit>(..., AstTypeList{{}, nullptr})`
      // （Parser.cpp:1845-1846）：缺省返回类型是一个「空类型列表、无尾注」的显式 pack。
      let ret_types = self.parse_optional_return_type(None).or_else(|| {
        let empty = self.alloc_type_pack(AstTypePackExplicit::new(
          self.lexer.current().location,
          AstTypeList::new(AstArray::EMPTY, None),
        ));
        node_opt(empty)
      });
      let end = self.lexer.current().location;

      let mut vars = TempVector::new(&mut self.scratch_type);
      let mut var_names = TempVector::new(&mut self.scratch_arg_name);

      for arg in &args {
        // cpp `if (!args[i].annotation) return reportStatError(...)`（Parser.cpp:1855）：
        // 「未标注」判空折叠为 Option::is_none，不再直接问裸指针。
        if node_opt(arg.annotation).is_none() {
          return self.report_unannotated_param(start, &end);
        }

        vars.push_back(arg.annotation);
        var_names.push_back((arg.name.name, arg.name.location));
      }

      if vararg && vararg_annotation.is_none() {
        return self.report_unannotated_param(start, &end);
      }

      let vars_array = self.copy_temp_vector_t(&vars);
      let var_names_array = self.copy_temp_vector_t(&var_names);
      self.alloc_stat(AstStatDeclareFunction {
        base: AstStat::new(
          AstStatDeclareFunction::CLASS_INDEX,
          Location::new(start.begin, end.end),
        ),
        attributes: *attributes,
        name: global_name.name,
        name_location: global_name.location,
        generics,
        generic_packs,
        params: AstTypeList::new(vars_array, vararg_annotation),
        param_names: var_names_array,
        vararg,
        vararg_location,
        ret_types: opt_node(ret_types),
      })
    } else if (self.lexer.current().name() == "class"
      && (if fflag::LuauAllowGlobalDeclarationToBeCalledClass.get() {
        self.lexer.lookahead().r#type != Type::COLON
      } else {
        true
      }))
      || self.lexer.current().name() == "extern"
    {
      let mut found_extern = false;
      if self.lexer.current().name() == "extern" {
        found_extern = true;
        self.next_lexeme();
        if self.lexer.current().name() != "type" {
          return self.report_stat_error(
            self.lexer.current().location,
            AstArray::EMPTY,
            AstArray::EMPTY,
            format_args!(
              "Expected `type` keyword after `extern`, but got {} instead",
              self.lexer.current().name()
            ),
          );
        }
      }

      self.next_lexeme();

      let class_start = self.lexer.current().location;
      let class_name = self.parse_name("type name");
      let mut super_name: Option<AstName> = None;

      if self.lexer.current().name() == "extends" {
        self.next_lexeme();
        super_name = Some(self.parse_name("supertype name").name);
      }

      if found_extern {
        if self.lexer.current().name() != "with" {
          self.report(
            self.lexer.current().location,
            format_args!(
              "Expected `with` keyword before listing properties of the external type, but got {} instead",
              self.lexer.current().name()
            ),
          );
        } else {
          self.next_lexeme();
        }
      }

      let mut props = TempVector::new(&mut self.scratch_declared_class_props);
      // cpp `AstTableIndexer* indexer = nullptr`（Parser.cpp:1916）：extern type 至多一个
      // indexer，未出现即 None；判重走 Option 而非裸指针非空。
      let mut indexer: Option<NonNull<AstTableIndexer>> = None;

      while self.lexer.current().r#type != Type::RESERVED_END {
        let mut attributes = AstArray::EMPTY;

        if self.lexer.current().r#type == Type::ATTRIBUTE
          || self.lexer.current().r#type == Type::ATTRIBUTE_OPEN
        {
          attributes = self.parse_attributes();

          if self.lexer.current().r#type != Type::RESERVED_FUNCTION {
            let current = *self.lexer.current();
            return self.report_stat_error(
              current.location,
              AstArray::EMPTY,
              AstArray::EMPTY,
              format_args!(
                "Expected a method type declaration after attribute, but got {current} instead",
              ),
            );
          }
        }

        // There are two possibilities: Either it's a property or a function.
        if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
          props.push_back(self.parse_declared_extern_type_method(&attributes));
        } else if self.lexer.current().r#type == Type::LBRACKET {
          let begin = *self.lexer.current();
          self.next_lexeme(); // [

          if (self.lexer.current().r#type == Type::RAW_STRING
            || self.lexer.current().r#type == Type::QUOTED_STRING)
            && self.lexer.lookahead().r#type == Type::RBRACKET
          {
            let name_begin = self.lexer.current().location;
            let chars = self.parse_char_array(false).map(|(value, _)| value);

            let name_end = *self.lexer.previous_location();

            self.expect_match_and_consume(']', &MatchLexeme::new(&begin), false);
            self.expect_and_consume_char(':', "property type annotation");
            let ty = self.parse_type(false);

            // since AstName contains a char*, it can't contain null
            let contains_null = chars.as_ref().is_some_and(|c| c.contains_null());

            if let (Some(chars), false) = (chars, contains_null) {
              props.push_back(AstDeclaredExternTypeProperty {
                name: AstName {
                  value: chars.data as *const u8,
                  len: chars.size as u32,
                },
                name_location: Location::new(name_begin.begin, name_end.end),
                ty,
                is_method: false,
                location: Location::new(begin.location.begin, self.lexer.previous_location().end),
                access: AstTableAccess::ReadWrite,
              });
            } else {
              self.report(
                begin.location,
                format_args!("String literal contains malformed escape sequence or \\0"),
              );
            }
          } else if indexer.is_some() {
            let bad_indexer_res = self.parse_table_indexer(AstTableAccess::ReadWrite, None, begin);
            let bad_indexer_location = slot_ref(bad_indexer_res.node).location;
            self.report(
              bad_indexer_location,
              format_args!("Cannot have more than one indexer on an extern type"),
            );
          } else {
            indexer = node_opt(
              self
                .parse_table_indexer(AstTableAccess::ReadWrite, None, begin)
                .node,
            );
          }
        } else {
          let mut access = AstTableAccess::ReadWrite;

          if self.lexer.current().r#type == Type::NAME
            && self.lexer.lookahead().r#type != Type::COLON
          {
            let current_name = self.lexer.current().name();
            if current_name == "read" {
              access = AstTableAccess::Read;
              self.lexer.next_lexeme();
            } else if current_name == "write" {
              access = AstTableAccess::Write;
              self.lexer.next_lexeme();
            } else {
              self.report(
                self.lexer.current().location,
                format_args!(
                  "Expected blank or 'read' or 'write' attribute, got '{}'",
                  self.lexer.current().name()
                ),
              );
              self.lexer.next_lexeme();
            }
          }

          let prop_start = self.lexer.current().location;

          let Some(prop_name) = self.parse_name_opt("property name") else {
            break;
          };

          self.expect_and_consume_char(':', "property type annotation");
          let prop_type = self.parse_type(false);
          props.push_back(AstDeclaredExternTypeProperty {
            name: prop_name.name,
            name_location: prop_name.location,
            ty: prop_type,
            is_method: false,
            location: Location::new(prop_start.begin, self.lexer.previous_location().end),
            access,
          });
        }
      }

      let class_end = self.lexer.current().location;
      self.next_lexeme(); // skip past `end`

      let props_array = self.copy_temp_vector_t(&props);
      self.alloc_stat(AstStatDeclareExternType::new(
        Location::new(class_start.begin, class_end.end),
        class_name.name,
        super_name,
        props_array,
        indexer,
      ))
    } else if let Some(global_name) = self.parse_name_opt("global variable name") {
      self.expect_and_consume_char(':', "global variable declaration");

      let ty = self.parse_type(true);
      self.alloc_stat(AstStatDeclareGlobal::new(
        Location::new(start.begin, slot_ref(ty).base.location.end),
        global_name.name,
        global_name.location,
        ty,
      ))
    } else {
      self.report_stat_error(
        *start,
        AstArray::EMPTY,
        AstArray::EMPTY,
        format_args!("declare must be followed by an identifier, 'function', or 'extern type'"),
      )
    }
  }

  /// 声明参数必须全部带标注的报错（原文件两处重复块）。C++ 对应
  /// `reportStatError(Location(start, end), {}, {}, "All declaration parameters must be annotated")`。
  fn report_unannotated_param(&mut self, start: &Location, end: &Location) -> *mut AstStat {
    self.report_stat_error(
      Location::new(start.begin, end.end),
      AstArray::EMPTY,
      AstArray::EMPTY,
      format_args!("All declaration parameters must be annotated"),
    )
  }
}
