use core::ptr::null_mut;

use ulua_common::FFlag;

use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{
    ast_array::AstArray, ast_attr::AstAttr,
    ast_declared_extern_type_property::AstDeclaredExternTypeProperty, ast_name::AstName,
    ast_stat::AstStat, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_table_indexer::AstTableIndexer,
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, lexeme::Type, location::Location,
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

    if (attributes.size != 0) && (self.lexer.current().r#type != Type::RESERVED_FUNCTION) {
      let current = *self.lexer.current();
      return self.report_stat_error(
        current.location,
        AstArray::default(),
        AstArray::default(),
        format_args!(
          "Expected a function type declaration after attribute, but got {current} instead",
        ),
      ) as *mut AstStat;
    }

    if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      self.next_lexeme();

      let global_name = self.parse_name("global function name");
      let (generics, generic_packs) = self.parse_generic_type_list(false, None, None, None);

      let match_paren = MatchLexeme::new(self.lexer.current());

      self.expect_and_consume_type(Type('(' as i32), "global function declaration");

      let mut args = TempVector::new(&mut self.scratch_binding);

      let mut vararg = false;
      let mut vararg_location = Location::default();
      let mut vararg_annotation = null_mut();

      if self.lexer.current().r#type != Type(')' as i32) {
        let res =
          self.parse_binding_list(&mut args, true, null_mut(), null_mut(), null_mut(), false);
        vararg = res.0;
        vararg_location = res.1;
        vararg_annotation = res.2;
      }

      self.expect_match_and_consume(')', &match_paren, false);

      let mut ret_types = self.parse_optional_return_type(None);
      if ret_types.is_null() {
        ret_types = unsafe {
          (*self.allocator).alloc(AstTypePackExplicit::new(
            self.lexer.current().location,
            AstTypeList::default(),
          ))
        } as *mut AstTypePack;
      }
      let end = self.lexer.current().location;

      let mut vars = TempVector::new(&mut self.scratch_type);
      let mut var_names = TempVector::new(&mut self.scratch_arg_name);

      for arg in TempVector::iter(&args) {
        if arg.annotation.is_null() {
          return self.report_stat_error(
            Location::new(start.begin, end.end),
            AstArray::default(),
            AstArray::default(),
            format_args!("All declaration parameters must be annotated"),
          ) as *mut AstStat;
        }

        vars.push_back(arg.annotation);
        var_names.push_back((arg.name.name, arg.name.location));
      }

      if vararg && vararg_annotation.is_null() {
        return self.report_stat_error(
          Location::new(start.begin, end.end),
          AstArray::default(),
          AstArray::default(),
          format_args!("All declaration parameters must be annotated"),
        ) as *mut AstStat;
      }

      unsafe {
        (*self.allocator).alloc(AstStatDeclareFunction {
          base: AstStat::new(
            AstStatDeclareFunction::CLASS_INDEX,
            Location::new(start.begin, end.end),
          ),
          attributes: *attributes,
          name: global_name.name,
          name_location: global_name.location,
          generics,
          generic_packs,
          params: AstTypeList {
            types: self.copy_temp_vector_t(&vars),
            tail_type: vararg_annotation,
          },
          param_names: self.copy_temp_vector_t(&var_names),
          vararg,
          vararg_location,
          ret_types,
        }) as *mut AstStat
      }
    } else if (unsafe {
      AstName::ast_name_c_char(self.lexer.current().data.name).operator_eq_c_char(c"class")
    } && (if FFlag::LuauAllowGlobalDeclarationToBeCalledClass.get() {
      self.lexer.lookahead().r#type != Type(':' as i32)
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
            AstArray::default(),
            AstArray::default(),
            format_args!(
              "Expected `type` keyword after `extern`, but got {} instead",
              self.lexer.current().name()
            ),
          ) as *mut AstStat;
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
      let mut indexer: *mut AstTableIndexer = null_mut();

      while self.lexer.current().r#type != Type::RESERVED_END {
        let mut attributes = AstArray::<*mut AstAttr>::default();

        if self.lexer.current().r#type == Type::ATTRIBUTE
          || self.lexer.current().r#type == Type::ATTRIBUTE_OPEN
        {
          attributes = self.parse_attributes();

          if self.lexer.current().r#type != Type::RESERVED_FUNCTION {
            let current = *self.lexer.current();
            return self.report_stat_error(
              current.location,
              AstArray::default(),
              AstArray::default(),
              format_args!(
                "Expected a method type declaration after attribute, but got {current} instead",
              ),
            ) as *mut AstStat;
          }
        }

        // There are two possibilities: Either it's a property or a function.
        if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
          props.push_back(self.parse_declared_extern_type_method(&attributes));
        } else if self.lexer.current().r#type == Type('[' as i32) {
          let begin = *self.lexer.current();
          self.next_lexeme(); // [

          if (self.lexer.current().r#type == Type::RAW_STRING
            || self.lexer.current().r#type == Type::QUOTED_STRING)
            && self.lexer.lookahead().r#type == Type(']' as i32)
          {
            let name_begin = self.lexer.current().location;
            let chars = self.parse_char_array(None);

            let name_end = *self.lexer.previous_location();

            self.expect_match_and_consume(']', &MatchLexeme::new(&begin), false);
            self.expect_and_consume_char(':', "property type annotation");
            let ty = self.parse_type_bool(false);

            // since AstName contains a char*, it can't contain null
            let mut contains_null = false;
            if let Some(ref c) = chars {
              for &ch in c.as_slice() {
                if ch == 0 {
                  contains_null = true;
                  break;
                }
              }
            }

            if let (Some(chars), false) = (chars, contains_null) {
              props.push_back(AstDeclaredExternTypeProperty {
                name: AstName { value: chars.data },
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
          } else if !indexer.is_null() {
            let bad_indexer_res = self.parse_table_indexer(AstTableAccess::ReadWrite, None, begin);
            let bad_indexer = bad_indexer_res.node;
            self.report(
              unsafe { (*bad_indexer).location },
              format_args!("Cannot have more than one indexer on an extern type"),
            );
          } else {
            indexer = self
              .parse_table_indexer(AstTableAccess::ReadWrite, None, begin)
              .node;
          }
        } else {
          let mut access = AstTableAccess::ReadWrite;

          if self.lexer.current().r#type == Type::NAME
            && self.lexer.lookahead().r#type != Type(':' as i32)
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
          let prop_name = self.parse_name_opt("property name");

          if prop_name.is_none() {
            break;
          }
          let prop_name = prop_name.unwrap();

          self.expect_and_consume_char(':', "property type annotation");
          let prop_type = self.parse_type_bool(false);
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

      unsafe {
        (*self.allocator).alloc(AstStatDeclareExternType::new(
          Location::new(class_start.begin, class_end.end),
          class_name.name,
          super_name,
          self.copy_temp_vector_t(&props),
          indexer,
        )) as *mut AstStat
      }
    } else if let Some(global_name) = self.parse_name_opt("global variable name") {
      self.expect_and_consume_char(':', "global variable declaration");

      let ty = self.parse_type_bool(true);
      unsafe {
        (*self.allocator).alloc(AstStatDeclareGlobal::new(
          Location::new(start.begin, (*ty).base.location.end),
          global_name.name,
          global_name.location,
          ty,
        )) as *mut AstStat
      }
    } else {
      self.report_stat_error(
        *start,
        AstArray::default(),
        AstArray::default(),
        format_args!("declare must be followed by an identifier, 'function', or 'extern type'"),
      ) as *mut AstStat
    }
  }
}
