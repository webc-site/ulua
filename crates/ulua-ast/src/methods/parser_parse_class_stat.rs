use core::{ffi::c_char, ptr::null_mut};

use ulua_common::{
  FFlag,
  records::{dense_hash_set::DenseHashSet, variant::Variant2},
};

use crate::records::{
  ast_array::AstArray, ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
  ast_name::AstName, ast_stat::AstStat, ast_stat_class::AstStatClass, ast_type::AstType,
  binding::Binding, lexeme::Type, location::Location, name::Name, parser::Parser,
  position::Position, temp_vector::TempVector,
};

const ALLOWED_METAMETHODS: &[&str] = &[
  "__init",
  "__call",
  "__concat",
  "__unm",
  "__add",
  "__sub",
  "__mul",
  "__div",
  "__mod",
  "__pow",
  "__tostring",
  "__eq",
  "__lt",
  "__le",
  "__iter",
  "__len",
  "__idiv",
];

const EXPLICITLY_DISALLOWED_METAMETHODS: &[&str] =
  &["__index", "__newindex", "__mode", "__metatable", "__type"];

use crate::records::ast_expr::AstExpr;

impl Parser {
  pub fn parse_class_stat(&mut self, start: &Location, exported: bool, open: bool) -> *mut AstStat {
    ulua_common::LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());
    let name_opt = self.parse_name_opt("type name");

    let name = name_opt.unwrap_or_else(|| Name {
      name: self.name_error,
      location: self.lexer.current().location,
    });

    let saved_locals = self.save_locals();

    let binding = Binding::new(name, null_mut(), Position::default(), true);
    let name_local = self.push_local(&binding);

    let mut super_: *mut AstExpr = null_mut();
    if self.lexer.current().r#type == Type::NAME
      && unsafe {
        AstName::ast_name_c_char(self.lexer.current().data.name).operator_eq_c_char(c"extends")
      }
    {
      self.next_lexeme();
      super_ = self.parse_class_ref_expr();
    }

    let mut declarations = TempVector::new(&mut self.scratch_class_declarations);

    let mut class_member_namespace: DenseHashSet<AstName> = DenseHashSet::new(AstName::default());

    while self.lexer.current().r#type != Type::RESERVED_END
      && self.lexer.current().r#type != Type::EOF
    {
      let mut qualifier_location: Option<Location> = None;
      if self.lexer.current().r#type == Type::NAME
        && unsafe {
          AstName::ast_name_c_char(self.lexer.current().data.name).operator_eq_c_char(c"public")
        }
      {
        qualifier_location = Some(self.lexer.current().location);
        self.next_lexeme();
      }

      if qualifier_location.is_some() && self.lexer.current().r#type != Type::RESERVED_FUNCTION {
        let prop_name_opt = self.parse_name_opt("class property name");
        if prop_name_opt.is_none() {
          continue;
        }
        let prop_name = prop_name_opt.unwrap();

        let mut prop_type: *mut AstType = null_mut();
        let mut type_colon_location: Option<Location> = None;

        if self.lexer.current().r#type == Type(':' as i32) {
          type_colon_location = Some(self.lexer.current().location);
          self.next_lexeme();
          prop_type = self.parse_type_bool(false);
        }

        unsafe {
          if !prop_name.name.value.is_null()
            && *prop_name.name.value == b'_' as c_char
            && *prop_name.name.value.add(1) == b'_' as c_char
          {
            self.report_location_c_char_item(
              prop_name.location,
              format_args!("Class properties cannot start with '__'"),
            );
          }
        }

        if class_member_namespace.contains(&prop_name.name) {
          self.report_location_c_char_item(
            prop_name.location,
            format_args!("Duplicate class member '{}'", prop_name.name),
          );
        } else {
          class_member_namespace.insert(prop_name.name);

          ulua_common::LUAU_ASSERT!(prop_type.is_null() == type_colon_location.is_none());
          declarations.push_back(Variant2::V0(AstClassProperty {
            qualifier_location: qualifier_location.unwrap_or_default(),
            name: prop_name.name,
            name_location: prop_name.location,
            type_colon_location,
            ty: prop_type,
          }));
        }
      } else if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
        let match_function = *self.lexer.current();
        self.next_lexeme();

        let name = self.parse_name("method name");

        self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] += 1;

        let (body, _) = self.parse_function_body(
          false,
          &match_function,
          &name.name,
          None,
          &AstArray::default(),
          false,
        );

        self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] -= 1;

        unsafe {
          if (*body).args.size > 0 {
            let first_arg = *(*body).args.data;
            if (*first_arg).name == "self" && !(*first_arg).annotation.is_null() {
              self.report_location_c_char_item(
                (*(*first_arg).annotation).base.location,
                format_args!("The 'self' parameter cannot have a type annotation"),
              );
            }
          }
        }

        let name_str = name.name.as_str_or_empty();
        if name_str.starts_with("__") {
          if EXPLICITLY_DISALLOWED_METAMETHODS.contains(&name_str) {
            self.report_location_c_char_item(
              name.location,
              format_args!("Classes cannot define '{}' as a metamethod", name_str),
            );
          } else if !ALLOWED_METAMETHODS.contains(&name_str) {
            self.report_location_c_char_item(
              name.location,
              format_args!(
                "Cannot use '{}' as a method name: names starting with '__' are reserved",
                name_str
              ),
            );
          }
        }

        if class_member_namespace.contains(&name.name) {
          self.report_location_c_char_item(
            name.location,
            format_args!("Duplicate class member '{}'", name.name),
          );
        } else {
          class_member_namespace.insert(name.name);

          declarations.push_back(Variant2::V1(AstClassMethod {
            qualifier_location,
            keyword_location: match_function.location,
            function_name: name.name,
            name_location: name.location,
            function: body,
          }));
        }
      } else {
        self.report_location_c_char_item(
          self.lexer.current().location,
          format_args!("Only class properties and functions can be declared within a class"),
        );
        self.next_lexeme();
      }
    }

    let end = self.lexer.current().location;
    self.expect_and_consume_type(Type::RESERVED_END, "class");
    let location = Location::new(start.begin, end.end);

    if self.recursion_counter > 1 {
      unsafe {
        self.report_location_c_char_item(
          (*name_local).location,
          format_args!(
            "Cannot declare class '{}' inside another statement or expression",
            (*name_local).name
          ),
        );
      }
    }

    let copied_declarations = self.copy_temp_vector_t(&declarations);
    let cls = unsafe {
      (*self.allocator).alloc(AstStatClass::new(
        location,
        name_local,
        super_,
        copied_declarations,
        exported,
        open,
      )) as *mut AstStat
    };

    let name_local_name = unsafe { (*name_local).name };
    if self.classes_within_module.contains(&name_local_name) {
      self.restore_locals(saved_locals);
      let expressions = self.copy_initializer_list_t(&[]);
      let statements = self.copy_initializer_list_t(&[cls]);
      unsafe {
        return self.report_stat_error(
          (*name_local).location,
          expressions,
          statements,
          format_args!(
            "A class named '{}' has already been declared in this module",
            name_local_name
          ),
        ) as *mut AstStat;
      }
    }
    self.classes_within_module.insert(name_local_name);
    cls
  }
}
