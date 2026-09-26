//! Source: `Ast/src/Parser.cpp:1683`
//!
//! Faithful port of `Parser::parseDeclaredExternTypeMethod` — parse one method
//! signature inside a `declare class`: `name(self, args...): ret`. The first
//! parameter must be an unannotated `self`; every other must be annotated. The
//! result is an `AstTypeFunction` wrapped in an `AstDeclaredExternTypeProperty`
//! (generic method declarations are not yet supported, matching the C++ TODO).

use core::ptr::NonNull;

use crate::{
  enums::{ast_table_access::AstTableAccess, type_lexer::Type},
  functions::optional_node::{node_opt, opt_node},
  records::{
    ast_array::AstArray, ast_attr::AstAttr,
    ast_declared_extern_type_property::AstDeclaredExternTypeProperty,
    ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
    ast_type_function::AstTypeFunction, ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, location::Location, match_lexeme::MatchLexeme,
    parser::Parser, temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_declared_extern_type_method(
    &mut self,
    attributes: &AstArray<*mut AstAttr>,
  ) -> AstDeclaredExternTypeProperty {
    let start = self.lexer.current().location;

    self.next_lexeme();

    let fn_name = self.parse_name("function name");

    // TODO: generic method declarations CLI-39909
    let generics: AstArray<*mut AstGenericType> = AstArray::EMPTY;
    let generic_packs: AstArray<*mut AstGenericTypePack> = AstArray::EMPTY;

    let match_paren = *self.lexer.current();
    self.expect_and_consume_char('(', "function parameter list start");

    let mut args = TempVector::new(&mut self.scratch_binding);

    let mut vararg = false;
    // cpp `AstTypePack* varargAnnotation = nullptr`：无 `...` 尾注即 None。
    let mut vararg_annotation: Option<NonNull<AstTypePack>> = None;
    if self.lexer.current().r#type != Type::RPAREN {
      let (v, _vl, va) = self.parse_binding_list(&mut args, true, None, None, None, false);
      vararg = v;
      vararg_annotation = va;
    }

    self.expect_match_and_consume(')', &MatchLexeme::new(&match_paren), false);

    let mut ret_types = self.parse_optional_return_type(None);
    if ret_types.is_none() {
      let loc = self.lexer.current().location;
      ret_types = node_opt(self.alloc_type_pack(AstTypePackExplicit::new(
        loc,
        AstTypeList::new(AstArray::EMPTY, None),
      )));
    }
    let end = *self.lexer.previous_location();

    let mut vars = TempVector::new(&mut self.scratch_type);
    let mut var_names = TempVector::new(&mut self.scratch_opt_arg_name);

    if args.is_empty() || args[0].name.name != "self" || node_opt(args[0].annotation).is_some() {
      let err = self.report_type_error(
        Location::new(start.begin, end.end),
        AstArray::EMPTY,
        format_args!("'self' must be present as the unannotated first parameter"),
      );
      return AstDeclaredExternTypeProperty {
        name: fn_name.name,
        name_location: fn_name.location,
        ty: err,
        is_method: true,
        location: Location::default(),
        access: AstTableAccess::ReadWrite,
      };
    }

    // Skip the first index ('self').
    for arg in &args[1..] {
      let arg_name = (arg.name.name, arg.name.location);
      var_names.push_back(Some(arg_name));

      let annotation = arg.annotation;
      if let Some(annotation) = node_opt(annotation) {
        vars.push_back(annotation.as_ptr());
      } else {
        let err = self.report_type_error(
          Location::new(start.begin, end.end),
          AstArray::EMPTY,
          format_args!("All declaration parameters aside from 'self' must be annotated"),
        );
        vars.push_back(err);
      }
    }

    if vararg && vararg_annotation.is_none() {
      self.report(
        start,
        format_args!("All declaration parameters aside from 'self' must be annotated"),
      );
    }

    let arg_types = AstTypeList::new(self.copy_temp_vector_t(&vars), vararg_annotation);
    let arg_names = self.copy_temp_vector_t(&var_names);

    let fn_type = self.alloc_type(AstTypeFunction::ast_type_function_location_ast_array_ast_attr_ast_array_ast_generic_type_ast_array_ast_generic_type_pack_ast_type_list_ast_array_optional_ast_argument_name_ast_type_pack(
                Location::new(start.begin, end.end),
                *attributes,
                generics,
                generic_packs,
                arg_types,
                arg_names,
                opt_node(ret_types),
            ));

    AstDeclaredExternTypeProperty {
      name: fn_name.name,
      name_location: fn_name.location,
      ty: fn_type,
      is_method: true,
      location: Location::new(start.begin, end.end),
      access: AstTableAccess::ReadWrite,
    }
  }
}
